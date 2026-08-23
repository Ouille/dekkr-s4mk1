// SPDX-License-Identifier: GPL-2.0-only
//
// Acces USB au Traktor Kontrol S4 MK1. Portage de `sound/usb/caiaq` :
//   Copyright (c) 2007 Daniel Mack, Karsten Wiese
//
// Tout ce qui touche au peripherique vit ici. Le reste du programme ne
// manipule que `S4` et `Lecture`.

use std::time::Duration;

use rusb::{Device, DeviceHandle, Direction, GlobalContext, TransferType};

pub const VID: u16 = 0x17cc;
pub const PID: u16 = 0xbaff;

// device.h
pub const EP1_CMD_GET_DEVICE_INFO: u8 = 0x01;
pub const EP1_CMD_AUTO_MSG: u8 = 0x0b;
pub const EP4_BUFSIZE: usize = 512;

pub const EP1_OUT: u8 = 0x01;
pub const EP1_IN: u8 = 0x81;
pub const EP4_IN: u8 = 0x84;
/// control.c ecrit { index, valeur } en bulk sur l'ep 8 : le canal des LED.
pub const EP8_OUT: u8 = 0x08;

/// 🔴 Sur macOS, IOKit veut un tampon de lecture bulk multiple du
/// `wMaxPacketSize` de l'endpoint — 512 ici. Les 64 octets de caiaq
/// fonctionnent sous Linux et echouent EN SILENCE sur macOS.
const TAMPON_LECTURE: usize = 512;

const DELAI_COMMANDE: Duration = Duration::from_millis(500);
/// Delai de lecture de l'ep 0x84. Il borne aussi le temps de reaction au
/// Ctrl-C : la boucle ne peut s'arreter qu'entre deux lectures.
const DELAI_LECTURE: Duration = Duration::from_millis(250);

// ── Ce que le boitier declare de lui-meme ─────────────────────────────────────

/// Reponse a GET_DEVICE_INFO : `struct caiaq_device_spec` de device.h,
/// 14 octets compactes, precedee de l'octet de commande.
#[derive(Debug, Clone, Copy)]
pub struct Spec {
    pub firmware: u16,
    pub sous_type: u8,
    pub encodeurs: u8,
    pub entrees_analogiques: u8,
    pub entrees_numeriques: u8,
    pub sorties_numeriques: u8,
    pub audio_out: u8,
    pub audio_in: u8,
    pub audio_num_out: u8,
    pub audio_num_in: u8,
    pub midi_out: u8,
    pub midi_in: u8,
    pub alignement: u8,
}

impl Spec {
    fn depuis(buf: &[u8]) -> Option<Spec> {
        // 1 octet de commande + 14 de charge utile
        if buf.len() < 15 || buf[0] != EP1_CMD_GET_DEVICE_INFO {
            return None;
        }
        Some(Spec {
            firmware: u16::from_le_bytes([buf[1], buf[2]]),
            sous_type: buf[3],
            encodeurs: buf[4],
            entrees_analogiques: buf[5],
            entrees_numeriques: buf[6],
            sorties_numeriques: buf[7],
            audio_out: buf[8],
            audio_in: buf[9],
            audio_num_out: buf[10],
            audio_num_in: buf[11],
            midi_out: buf[12],
            midi_in: buf[13],
            alignement: buf[14],
        })
    }
}

// ── Description du descripteur, pour le mode diagnostic ───────────────────────

pub struct Plan {
    /// Interface et alt setting qui portent l'ep 0x84. `None` = decodage
    /// caiaq inapplicable a ce boitier.
    pub porteur_ep4: Option<(u8, u8)>,
    pub ep1_out: bool,
    pub ep1_in: bool,
    pub ep4_bulk_in: bool,
    pub ep8_out: bool,
    pub isochrones: Vec<String>,
    pub hid: bool,
    pub self_powered: bool,
    pub courant_ma: u16,
    pub lignes: Vec<String>,
}

/// Lit le descripteur SANS ouvrir le peripherique.
///
/// 🔴 On DETECTE l'alt setting porteur, on ne suppose pas « alt 1 » : l'ep 0x84
/// est absent de l'alt par defaut, et un « alt 1 » code en dur donnerait un
/// silence total apres avoir passe toutes les verifications.
pub fn planifier(device: &Device<GlobalContext>) -> Result<Plan, rusb::Error> {
    let config = device.active_config_descriptor()?;
    let mut p = Plan {
        porteur_ep4: None,
        ep1_out: false,
        ep1_in: false,
        ep4_bulk_in: false,
        ep8_out: false,
        isochrones: Vec::new(),
        hid: false,
        self_powered: config.self_powered(),
        courant_ma: config.max_power(),
        lignes: Vec::new(),
    };

    for iface in config.interfaces() {
        for id in iface.descriptors() {
            p.lignes.push(format!(
                "  interface {} (alt {}) : classe {:#04x}  sous-classe {:#04x}  protocole {:#04x}",
                id.interface_number(),
                id.setting_number(),
                id.class_code(),
                id.sub_class_code(),
                id.protocol_code()
            ));
            if id.class_code() == 0x03 {
                p.hid = true;
            }
            for ep in id.endpoint_descriptors() {
                p.lignes.push(format!(
                    "      endpoint {:#04x}  {:?} {:?}  max {} o  intervalle {}",
                    ep.address(),
                    ep.direction(),
                    ep.transfer_type(),
                    ep.max_packet_size(),
                    ep.interval()
                ));
                match ep.address() {
                    EP1_OUT => p.ep1_out = true,
                    EP1_IN => p.ep1_in = true,
                    EP8_OUT => p.ep8_out = true,
                    EP4_IN => {
                        p.ep4_bulk_in = ep.transfer_type() == TransferType::Bulk
                            && ep.direction() == Direction::In;
                        p.porteur_ep4 = Some((id.interface_number(), id.setting_number()));
                    }
                    _ => {}
                }
                if ep.transfer_type() == TransferType::Isochronous {
                    let s = format!("{:#04x} {:?}", ep.address(), ep.direction());
                    if !p.isochrones.contains(&s) {
                        p.isochrones.push(s);
                    }
                }
            }
        }
    }
    Ok(p)
}

// ── Recherche ─────────────────────────────────────────────────────────────────

/// Le boitier, s'il est branche. `Ok(None)` = absent, ce n'est PAS une erreur :
/// le programme doit pouvoir demarrer avant lui et l'attendre.
pub fn chercher() -> Result<Option<Device<GlobalContext>>, rusb::Error> {
    for device in rusb::devices()?.iter() {
        if let Ok(d) = device.device_descriptor() {
            if d.vendor_id() == VID && d.product_id() == PID {
                return Ok(Some(device));
            }
        }
    }
    Ok(None)
}

/// Inventaire complet, sans filtre. Une liste vide et une liste sans le S4 ne
/// veulent pas dire la meme chose : la premiere accuse libusb ou les droits,
/// la seconde accuse le boitier.
pub fn inventaire() -> Result<Vec<String>, rusb::Error> {
    let mut v = Vec::new();
    for device in rusb::devices()?.iter() {
        let Ok(d) = device.device_descriptor() else {
            v.push("  (descripteur illisible)".to_string());
            continue;
        };
        let cible = d.vendor_id() == VID && d.product_id() == PID;
        v.push(format!(
            "  bus {:03} adr {:03}  {:04x}:{:04x}  classe {:#04x}{}",
            device.bus_number(),
            device.address(),
            d.vendor_id(),
            d.product_id(),
            d.class_code(),
            if cible { "   <== S4 MK1" } else { "" }
        ));
    }
    Ok(v)
}

// ── Boitier arme ──────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum Panne {
    /// Le descripteur ne correspond pas a ce que caiaq attend.
    Descripteur(String),
    /// Reservation refusee — sur macOS, le cas normal en utilisateur simple.
    Acces(rusb::Error),
    Usb(rusb::Error),
}

impl std::fmt::Display for Panne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Panne::Descripteur(s) => write!(f, "descripteur inattendu : {s}"),
            Panne::Acces(e) => write!(
                f,
                "acces refuse ({e}) — relancer le binaire compile en root :\n\
                 \x20      sudo ./target/release/sonde-s4mk1"
            ),
            Panne::Usb(e) => write!(f, "erreur USB : {e}"),
        }
    }
}

/// Resultat d'une lecture sur l'ep 0x84.
pub enum Lecture<'a> {
    /// Des blocs sont arrives.
    Blocs(&'a [u8]),
    /// Rien dans le delai. **Normal** : le S4 est evenementiel, il n'emet
    /// strictement rien au repos.
    Silence,
    /// Le boitier a disparu — debranchement, coupure, veille.
    Perdu,
}

pub struct S4 {
    handle: DeviceHandle<GlobalContext>,
    iface: u8,
    pub spec: Option<Spec>,
}

impl S4 {
    /// Ouvre, reserve, bascule sur l'alt porteur, interroge, met en emission.
    ///
    /// 🔴 L'ordre est celui de `init_card()` de caiaq, et il n'est pas
    /// negociable : sans `AUTO_MSG` en dernier, le boitier reste MUET.
    pub fn armer(device: Device<GlobalContext>) -> Result<S4, Panne> {
        let plan = planifier(&device).map_err(Panne::Usb)?;

        let Some((iface, alt)) = plan.porteur_ep4 else {
            return Err(Panne::Descripteur(
                "aucun alt setting ne porte l'ep 0x84".into(),
            ));
        };
        if !(plan.ep1_out && plan.ep1_in) {
            return Err(Panne::Descripteur(
                "les endpoints de commande 0x01/0x81 manquent".into(),
            ));
        }

        let handle = device.open().map_err(|e| match e {
            rusb::Error::Access => Panne::Acces(e),
            autre => Panne::Usb(autre),
        })?;
        // Sans effet sur macOS. Utile sous Linux, ou snd-usb-caiaq tient l'interface.
        let _ = handle.set_auto_detach_kernel_driver(true);

        handle.claim_interface(iface).map_err(|e| match e {
            rusb::Error::Access | rusb::Error::Busy => Panne::Acces(e),
            autre => Panne::Usb(autre),
        })?;

        // caiaq/device.c, init_card() ligne 1 : usb_set_interface(usb_dev, 0, 1).
        handle
            .set_alternate_setting(iface, alt)
            .map_err(Panne::Usb)?;

        let mut s4 = S4 {
            handle,
            iface,
            spec: None,
        };
        s4.spec = s4.interroger();
        s4.mettre_en_emission().map_err(Panne::Usb)?;
        Ok(s4)
    }

    /// GET_DEVICE_INFO. `None` n'est pas fatal : le boitier peut tres bien
    /// livrer ses controles sans avoir repondu ici.
    fn interroger(&self) -> Option<Spec> {
        self.handle
            .write_bulk(EP1_OUT, &[EP1_CMD_GET_DEVICE_INFO], DELAI_COMMANDE)
            .ok()?;

        let mut buf = [0u8; TAMPON_LECTURE];
        for _ in 0..5 {
            match self.handle.read_bulk(EP1_IN, &mut buf, DELAI_COMMANDE) {
                Ok(n) => {
                    if let Some(spec) = Spec::depuis(&buf[..n]) {
                        return Some(spec);
                    }
                }
                Err(rusb::Error::Timeout) => continue,
                Err(_) => return None,
            }
        }
        None
    }

    /// `set_auto_msg(1, 10, 5)` — cf. caiaq/input.c pour le S4.
    /// 🔴 Sans elle, l'ep 0x84 ne delivre RIEN. Un silence avant cet envoi ne
    /// dit rien de l'etat du materiel.
    fn mettre_en_emission(&self) -> Result<(), rusb::Error> {
        self.handle
            .write_bulk(EP1_OUT, &[EP1_CMD_AUTO_MSG, 1, 10, 5], DELAI_COMMANDE)
            .map(|_| ())
    }

    pub fn lire<'a>(&self, buf: &'a mut [u8]) -> Lecture<'a> {
        match self.handle.read_bulk(EP4_IN, buf, DELAI_LECTURE) {
            Ok(n) => Lecture::Blocs(&buf[..n]),
            Err(rusb::Error::Timeout) => Lecture::Silence,
            // Debranchement, veille, coupure d'alimentation.
            Err(rusb::Error::NoDevice) | Err(rusb::Error::Io) | Err(rusb::Error::NotFound) => {
                Lecture::Perdu
            }
            // Un endpoint bloque se reprend en general seul ; on le traite comme
            // un silence plutot que de tout demonter.
            Err(_) => Lecture::Silence,
        }
    }
}

impl Drop for S4 {
    /// Arret propre : rendre l'interface. Le noyau le ferait de toute facon a
    /// la mort du processus, mais pas lors d'une simple reconnexion a chaud.
    fn drop(&mut self) {
        let _ = self.handle.release_interface(self.iface);
    }
}
