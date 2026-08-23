// SPDX-License-Identifier: GPL-2.0-only
//
// sonde-s4mk1 — sonde de diagnostic USB pour Traktor Kontrol S4 MK1 (17cc:baff)
//
// Portage en espace utilisateur du protocole de `sound/usb/caiaq` (noyau Linux) :
//   Copyright (c) 2007 Daniel Mack <daniel@caiaq.de>
//                      Karsten Wiese <fzu@wemgehoertderstaat.de>
// Ce fichier en est une oeuvre derivee. Les .c de caiaq sont GPL-2.0-or-later,
// mais `device.h` — origine des constantes reprises ci-dessous — est GPL-2.0
// seul : l'ensemble est donc distribue sous GPL-2.0, sans « or later ».
// ⛔ Ne JAMAIS integrer ce code au depot DekkR (front proprietaire).
//
// Constantes et sequences reprises telles quelles du noyau :
//   device.h : EP1_BUFSIZE 64, EP4_BUFSIZE 512,
//              EP1_CMD_GET_DEVICE_INFO 0x1, EP1_CMD_AUTO_MSG 0xb
//   device.c : snd_usb_caiaq_send_command() -> bulk OUT ep 1, buf[0] = commande
//              snd_usb_caiaq_set_auto_msg(cdev, d, a, e) -> payload { d, a, e }
//              init_card() -> GET_DEVICE_INFO puis attente de la reponse sur ep 1 IN
//   input.c  : S4 -> bulk IN ep 4, blocs de TKS4_MSGBLOCK_SIZE = 16 octets,
//              block_id = (buf[0] << 8) | buf[1]
//              snd_caiaq_input_report_abs() -> (buf[o*2] << 8) | buf[o*2+1]
//              bloc 0 = 96 boutons, bitmask a partir de buf[4]
//              init S4 -> snd_usb_caiaq_set_auto_msg(cdev, 1, 10, 5)

use std::collections::HashMap;
use std::time::{Duration, Instant};

use rusb::{Direction, TransferType};

const VID: u16 = 0x17cc;
const PID: u16 = 0xbaff;

const EP1_CMD_GET_DEVICE_INFO: u8 = 0x01;
const EP1_CMD_AUTO_MSG: u8 = 0x0b;
const EP4_BUFSIZE: usize = 512;
const TKS4_MSGBLOCK_SIZE: usize = 16;

const EP1_OUT: u8 = 0x01;
const EP1_IN: u8 = 0x81;
const EP4_IN: u8 = 0x84;
// control.c ecrit { index, valeur } en bulk sur l'ep 8 : c'est le canal des LED.
const EP8_OUT: u8 = 0x08;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("sonde-s4mk1 — Traktor Kontrol S4 MK1 ({VID:04x}:{PID:04x})");
    println!("=========================================================\n");

    // --- 1. Inventaire COMPLET, sans filtre -------------------------------
    // Une liste filtree ne permet pas de distinguer « l'appareil est absent »
    // de « mon filtre est faux ». On montre tout, on marque la cible.
    println!("--- Tous les peripheriques USB vus par libusb ---");
    let mut target = None;
    let mut seen = 0;
    let liste = rusb::devices()?;
    for device in liste.iter() {
        let d = match device.device_descriptor() {
            Ok(d) => d,
            Err(e) => {
                println!("  (descripteur illisible : {e})");
                continue;
            }
        };
        seen += 1;
        let hit = d.vendor_id() == VID && d.product_id() == PID;
        println!(
            "  bus {:03} adr {:03}  {:04x}:{:04x}  classe {:#04x}{}",
            device.bus_number(),
            device.address(),
            d.vendor_id(),
            d.product_id(),
            d.class_code(),
            if hit { "   <== S4 MK1" } else { "" }
        );
        if hit {
            target = Some(device);
        }
    }
    println!("  ({seen} peripherique(s) au total)\n");

    let device = match target {
        Some(d) => d,
        None => {
            println!("⛔ {VID:04x}:{PID:04x} ABSENT de la liste ci-dessus.");
            println!("   La liste n'est pas filtree, donc :");
            println!("   · si elle contient d'autres appareils -> libusb fonctionne,");
            println!("     c'est bien le S4 qui ne s'enumere pas (cable, alimentation, boitier) ;");
            println!("   · si elle est vide -> c'est libusb ou les droits qui sont en cause,");
            println!("     la sonde ne dit RIEN sur le S4. Reessayer avec sudo.");
            return Ok(());
        }
    };

    // --- 2. Descripteur : ce que Windows ne pouvait pas montrer -----------
    let desc = device.device_descriptor()?;
    println!("--- Descripteur du peripherique ---");
    println!(
        "  classe {:#04x}  sous-classe {:#04x}  protocole {:#04x}  configurations : {}",
        desc.class_code(),
        desc.sub_class_code(),
        desc.protocol_code(),
        desc.num_configurations()
    );

    let config = device.active_config_descriptor()?;
    println!(
        "\n--- Configuration {} : {} interface(s) ---",
        config.number(),
        config.num_interfaces()
    );
    // C'est le boitier qui declare ses besoins electriques, pas la doc.
    // Un port USB standard fournit 500 mA (900 sur un port USB 3).
    println!(
        "  alimentation : {}  ·  courant demande sur le bus : {} mA{}",
        if config.self_powered() {
            "AUTONOME (bloc secteur)"
        } else {
            "PAR LE BUS USB"
        },
        config.max_power(),
        if config.max_power() > 500 {
            "   ⚠️ au-dela de ce qu'un port 500 mA garantit"
        } else {
            ""
        }
    );

    // L'ep 0x84 n'est pas forcement present dans l'alt setting par defaut :
    // on retient QUEL alt le porte, au lieu de supposer que c'est le 0.
    let mut porteur_ep4: Option<(u8, u8)> = None;
    let mut hid_present = false;
    let (mut ep1_out_ok, mut ep1_in_ok, mut ep4_in_ok) = (false, false, false);
    let mut ep4_bulk = false;
    let mut ep8_out_ok = false;
    let mut isochrones: Vec<String> = Vec::new();

    for iface in config.interfaces() {
        for id in iface.descriptors() {
            println!(
                "  interface {} (alt {}) : classe {:#04x}  sous-classe {:#04x}  protocole {:#04x}",
                id.interface_number(),
                id.setting_number(),
                id.class_code(),
                id.sub_class_code(),
                id.protocol_code()
            );
            if id.class_code() == 0x03 {
                hid_present = true;
                println!("      ⚠️  CLASSE HID (0x03) presente dans le descripteur");
            }
            for ep in id.endpoint_descriptors() {
                println!(
                    "      endpoint {:#04x}  {:?} {:?}  max {} o  intervalle {}",
                    ep.address(),
                    ep.direction(),
                    ep.transfer_type(),
                    ep.max_packet_size(),
                    ep.interval()
                );
                match ep.address() {
                    EP1_OUT => ep1_out_ok = true,
                    EP1_IN => ep1_in_ok = true,
                    EP8_OUT => ep8_out_ok = true,
                    EP4_IN => {
                        ep4_in_ok = true;
                        ep4_bulk = ep.transfer_type() == TransferType::Bulk
                            && ep.direction() == Direction::In;
                        porteur_ep4 = Some((id.interface_number(), id.setting_number()));
                    }
                    _ => {}
                }
                if ep.transfer_type() == TransferType::Isochronous {
                    let s = format!("{:#04x} {:?}", ep.address(), ep.direction());
                    if !isochrones.contains(&s) {
                        isochrones.push(s);
                    }
                }
            }
        }
    }
    println!("\n--- Verification des endpoints attendus par caiaq ---");
    println!("  ep 0x01 OUT (commandes)      : {}", oui_non(ep1_out_ok));
    println!("  ep 0x81 IN  (reponses)       : {}", oui_non(ep1_in_ok));
    println!("  ep 0x84 IN  (controles)      : {}", oui_non(ep4_in_ok));
    println!("  ep 0x84 est bien bulk IN     : {}", oui_non(ep4_bulk));
    println!("  ep 0x08 OUT (LED, control.c) : {}", oui_non(ep8_out_ok));
    println!(
        "  endpoints isochrones (audio) : {}",
        if isochrones.is_empty() {
            "aucun".to_string()
        } else {
            isochrones.join(", ")
        }
    );
    println!(
        "  interface HID dans le descripteur : {}",
        if hid_present { "OUI" } else { "NON" }
    );

    let (n_iface, n_alt) = match porteur_ep4 {
        Some(x) => x,
        None => {
            println!("\n⛔ Aucun alt setting ne porte l'ep 0x84 : le decodage caiaq ne");
            println!("   s'applique pas tel quel a ce boitier. La sortie ci-dessus suffit a conclure.");
            return Ok(());
        }
    };
    println!("  ep 0x84 porte par : interface {n_iface}, alt setting {n_alt}");

    if !(ep1_out_ok && ep1_in_ok) {
        println!("\n⛔ Les endpoints de commande manquent : impossible de dialoguer.");
        return Ok(());
    }

    // --- 3. Ouverture et reservation --------------------------------------
    println!("\n--- Ouverture ---");
    let handle = match device.open() {
        Ok(h) => h,
        Err(e) => {
            println!("⛔ ouverture impossible : {e}");
            println!("   macOS : relancer le binaire deja compile en root —");
            println!("       sudo ./target/release/sonde-s4mk1");
            return Ok(());
        }
    };
    // Sans effet sur macOS (non supporte) — utile sous Linux ou caiaq tient l'interface.
    let _ = handle.set_auto_detach_kernel_driver(true);

    if let Err(e) = handle.claim_interface(n_iface) {
        println!("⛔ reservation de l'interface {n_iface} impossible : {e}");
        println!("   macOS : c'est le cas normal en utilisateur simple. Relancer le");
        println!("   binaire DEJA COMPILE en root (pas `sudo cargo`, qui recompilerait) :");
        println!("       sudo ./target/release/sonde-s4mk1");
        println!("   Linux : `sudo modprobe -r snd_usb_caiaq` puis relancer.");
        return Ok(());
    }
    println!("  interface {n_iface} reservee");

    // ⚠️ caiaq/device.c, init_card() ligne 1 : usb_set_interface(usb_dev, 0, 1).
    // L'ep 0x84 n'existe QUE dans l'alt 1 : sans ce basculement il n'y a
    // strictement rien a lire, et le silence serait mis sur le dos du materiel.
    if let Err(e) = handle.set_alternate_setting(n_iface, n_alt) {
        println!("⛔ bascule en alt {n_alt} impossible : {e}");
        println!("   Sans elle l'ep 0x84 n'existe pas — inutile d'ecouter.");
        return Ok(());
    }
    println!("  interface {n_iface} basculee en alt {n_alt} — ep 0x84 disponible");

    // --- 4. GET_DEVICE_INFO : le boitier parle-t-il ? ----------------------
    let t = Duration::from_millis(500);
    println!("\n--- GET_DEVICE_INFO (commande {EP1_CMD_GET_DEVICE_INFO:#04x} sur ep 0x01) ---");
    let mut spec_lue = tente_device_info(&handle);
    if !spec_lue {
        println!("  ⛔ aucune reponse exploitable a GET_DEVICE_INFO.");
        println!("     On continue : la suite dira si c'est le dialogue qui manque");
        println!("     ou seulement cette commande-la.");
    }

    // --- 5. Mise en route : sans elle, le S4 n'emet RIEN -------------------
    println!("\n--- AUTO_MSG (commande {EP1_CMD_AUTO_MSG:#04x}, payload 1/10/5) ---");
    handle.write_bulk(EP1_OUT, &[EP1_CMD_AUTO_MSG, 1, 10, 5], t)?;
    println!("  envoyee.");

    // Second essai : si le boitier ne repondait qu'une fois « reveille »,
    // c'est l'ORDRE des commandes qui est en cause, pas le dialogue.
    if !spec_lue {
        println!("\n--- GET_DEVICE_INFO, 2e essai (apres AUTO_MSG) ---");
        spec_lue = tente_device_info(&handle);
        if spec_lue {
            println!("  🔑 il ne repondait qu'APRES AUTO_MSG : c'est l'ordre qui compte.");
        }
    }
    println!();

    // --- 6. Lecture des blocs de controle ---------------------------------
    println!("--- Ecoute sur ep 0x84 (Ctrl-C pour arreter) ---");
    println!("  Bouge UN controle a la fois : jog gauche, puis un fader, puis un bouton.");
    println!("  Les octets qui changent sont encadres [xx].");
    println!("  o1..o7 = valeurs 16 bits gros-boutiste aux offsets pairs (faders sur 12 bits).\n");

    let mut dernier: HashMap<u16, [u8; TKS4_MSGBLOCK_SIZE]> = HashMap::new();
    let mut buf4 = [0u8; EP4_BUFSIZE];
    let mut blocs = 0u64;
    let debut = Instant::now();
    // Un silence a l'ecran est ambigu : « plus rien n'arrive » et « ca arrive
    // mais rien ne change » se ressemblent. Le battement les separe.
    let mut dernier_battement = Instant::now();
    let mut blocs_au_battement = 0u64;

    loop {
        match handle.read_bulk(EP4_IN, &mut buf4, Duration::from_millis(1000)) {
            Ok(n) => {
                let mut off = 0;
                while off + TKS4_MSGBLOCK_SIZE <= n {
                    let mut bloc = [0u8; TKS4_MSGBLOCK_SIZE];
                    bloc.copy_from_slice(&buf4[off..off + TKS4_MSGBLOCK_SIZE]);
                    off += TKS4_MSGBLOCK_SIZE;
                    blocs += 1;

                    let id = ((bloc[0] as u16) << 8) | bloc[1] as u16;
                    let (premier, modifies) = match dernier.get(&id) {
                        None => (true, Vec::new()),
                        Some(prec) => (
                            false,
                            (0..TKS4_MSGBLOCK_SIZE)
                                .filter(|&i| prec[i] != bloc[i])
                                .collect::<Vec<usize>>(),
                        ),
                    };
                    if premier || !modifies.is_empty() {
                        affiche_bloc(debut.elapsed(), id, &bloc, &modifies, premier);
                    }
                    dernier.insert(id, bloc);
                }
                battement(
                    debut,
                    &mut dernier_battement,
                    blocs,
                    &mut blocs_au_battement,
                );
            }
            Err(rusb::Error::Timeout) => {
                battement(
                    debut,
                    &mut dernier_battement,
                    blocs,
                    &mut blocs_au_battement,
                );
                if blocs == 0 && debut.elapsed() > Duration::from_secs(10) {
                    println!("⛔ aucun bloc recu en 10 s sur l'ep 0x84.");
                    if spec_lue {
                        println!("   MAIS GET_DEVICE_INFO a repondu : le boitier est VIVANT et");
                        println!("   dialogue. Le probleme est dans la mise en route, pas dans");
                        println!("   le materiel — parametres d'AUTO_MSG ou endpoint a revoir.");
                    } else {
                        println!("   Et GET_DEVICE_INFO n'avait rien donne non plus : le boitier");
                        println!("   s'enumere sans repondre. C'est le seul cas ou « surface HS »");
                        println!("   devient l'hypothese la plus probable.");
                    }
                    break;
                }
            }
            Err(e) => {
                println!("erreur de lecture sur l'ep 0x84 : {e}");
                break;
            }
        }
    }

    Ok(())
}

/// Envoie GET_DEVICE_INFO et tente de lire la reponse sur l'ep 0x81.
///
/// ⚠️ Le tampon de lecture fait 512 o, pas les 64 de caiaq (`EP1_BUFSIZE`) : sur macOS,
/// IOKit veut un tampon multiple du `wMaxPacketSize` de l'endpoint (512 ici).
/// C'est la seule difference avec l'ep 0x84, qui lui fonctionne.
fn tente_device_info<T: rusb::UsbContext>(handle: &rusb::DeviceHandle<T>) -> bool {
    let t = Duration::from_millis(500);
    if let Err(e) = handle.write_bulk(EP1_OUT, &[EP1_CMD_GET_DEVICE_INFO], t) {
        println!("  ecriture de la commande impossible : {e}");
        return false;
    }

    let mut buf = [0u8; 512];
    for _ in 0..5 {
        match handle.read_bulk(EP1_IN, &mut buf, t) {
            Ok(n) if n >= 15 && buf[0] == EP1_CMD_GET_DEVICE_INFO => {
                println!(
                    "  ✅ le boitier repond ({n} o) — firmware {}",
                    u16::from_le_bytes([buf[1], buf[2]])
                );
                println!("     sous-type materiel      : {}", buf[3]);
                println!("     encodeurs (erp)         : {}", buf[4]);
                println!("     entrees analogiques     : {}", buf[5]);
                println!("     entrees numeriques      : {}", buf[6]);
                println!("     sorties numeriques      : {}", buf[7]);
                println!("     audio analogique out/in : {} / {}", buf[8], buf[9]);
                println!("     audio numerique  out/in : {} / {}", buf[10], buf[11]);
                println!("     MIDI out / in           : {} / {}", buf[12], buf[13]);
                println!("     alignement des donnees  : {}", buf[14]);
                return true;
            }
            Ok(n) => println!("  (reponse ignoree : {n} o, commande {:#04x})", buf[0]),
            Err(rusb::Error::Timeout) => println!("  (pas de reponse dans le delai)"),
            Err(e) => {
                println!("  erreur de lecture : {e}");
                return false;
            }
        }
    }
    false
}

/// Toutes les 3 s, dit si des blocs continuent d'arriver meme quand rien ne change.
fn battement(debut: Instant, dernier: &mut Instant, blocs: u64, blocs_avant: &mut u64) {
    if dernier.elapsed() < Duration::from_secs(3) {
        return;
    }
    let nouveaux = blocs - *blocs_avant;
    if nouveaux == 0 {
        println!(
            "[{:>7.2}s] … plus aucun bloc ne parvient (total {blocs})",
            debut.elapsed().as_secs_f32()
        );
    } else {
        println!(
            "[{:>7.2}s] … {nouveaux} blocs recus depuis 3 s, aucun changement (total {blocs})",
            debut.elapsed().as_secs_f32()
        );
    }
    *dernier = Instant::now();
    *blocs_avant = blocs;
}

fn oui_non(b: bool) -> &'static str {
    if b {
        "OUI"
    } else {
        "NON"
    }
}

fn affiche_bloc(
    t: Duration,
    id: u16,
    bloc: &[u8; TKS4_MSGBLOCK_SIZE],
    modifies: &[usize],
    premier: bool,
) {
    let hex: String = bloc
        .iter()
        .enumerate()
        .map(|(i, o)| {
            if modifies.contains(&i) {
                format!("[{o:02x}]")
            } else {
                format!(" {o:02x} ")
            }
        })
        .collect();

    println!(
        "[{:>7.2}s] bloc {:>3}{}  {}",
        t.as_secs_f32(),
        id,
        if premier { " (ref)" } else { "      " },
        hex
    );

    if id == 0 {
        // 96 boutons, bitmask a partir de buf[4] (12 octets)
        let enfonces: Vec<usize> = (0..96)
            .filter(|i| ((bloc[4 + i / 8] >> (i % 8)) & 1) == 1)
            .collect();
        println!("                     boutons enfonces : {enfonces:?}");
    } else if id == 1 {
        // caiaq/input.c, bloc 1 : les jogs sont sur 10 bits, pas 16 —
        // l'affichage generique o1..o7 serait faux ici.
        let jog_g = bloc[9] as u16 | ((bloc[8] as u16 & 0x3) << 8);
        let jog_d = bloc[13] as u16 | ((bloc[12] as u16 & 0x3) << 8);
        let enc = [
            bloc[3] & 0xf,
            bloc[4] >> 4,
            bloc[4] & 0xf,
            bloc[5] >> 4,
            bloc[5] & 0xf,
            bloc[6] >> 4,
            bloc[6] & 0xf,
            bloc[7] >> 4,
            bloc[7] & 0xf,
        ];
        println!("                     jog G={jog_g:<4} jog D={jog_d:<4}  encodeurs {enc:?}");
    } else {
        let vals: Vec<String> = (1..8)
            .map(|o| {
                let v = ((bloc[o * 2] as u16) << 8) | bloc[o * 2 + 1] as u16;
                format!("o{o}={v:<5}")
            })
            .collect();
        println!("                     {}", vals.join(" "));
    }
}
