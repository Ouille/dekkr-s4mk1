// SPDX-License-Identifier: GPL-2.0-only
//
// sonde-s4mk1 — Traktor Kontrol S4 MK1 (17cc:baff)
//
// Portage en espace utilisateur du protocole de `sound/usb/caiaq` (noyau Linux) :
//   Copyright (c) 2007 Daniel Mack <daniel@caiaq.de>
//                      Karsten Wiese <fzu@wemgehoertderstaat.de>
// Ce fichier en est une oeuvre derivee. Les .c de caiaq sont GPL-2.0-or-later,
// mais `device.h` — origine des constantes reprises — est GPL-2.0 seul :
// l'ensemble est donc distribue sous GPL-2.0, sans « or later ».
// ⛔ Ne JAMAIS integrer ce code au depot DekkR (front proprietaire).

mod affichage;
mod decode;
mod usb;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use affichage::Moniteur;
use decode::{Decodeur, Evenement};
use usb::{Lecture, S4};

/// Rythme d'attente quand le boitier est absent. Assez lent pour ne rien
/// couter, assez vif pour que le branchement paraisse immediat.
const ATTENTE: Duration = Duration::from_millis(500);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|a| a == "--diagnostic" || a == "-d") {
        return diagnostic();
    }
    if std::env::args().any(|a| a == "--help" || a == "-h") {
        println!("sonde-s4mk1 — Traktor Kontrol S4 MK1");
        println!("  (sans argument)  surveille le boitier et affiche les controles decodes");
        println!("  --brut           affiche les blocs de 16 octets en hexadecimal");
        println!("  --diagnostic     dump du descripteur USB, puis sort");
        return Ok(());
    }
    let brut = std::env::args().any(|a| a == "--brut" || a == "-b");
    surveiller(brut)
}

// ── Boucle de surveillance ────────────────────────────────────────────────────

/// Tourne indefiniment : attend le boitier, l'arme, le lit, encaisse les
/// debranchements, et rend l'interface a l'arret.
///
/// 🔑 Le programme ne depend PAS de l'ordre de branchement. On peut le lancer
/// avant le S4, le debrancher, le rebrancher : il se rearme seul.
fn surveiller(brut: bool) -> Result<(), Box<dyn std::error::Error>> {
    let stop = Arc::new(AtomicBool::new(false));
    {
        let stop = Arc::clone(&stop);
        ctrlc::set_handler(move || stop.store(true, Ordering::SeqCst))?;
    }

    println!("sonde-s4mk1 — surveillance ({:04x}:{:04x})", usb::VID, usb::PID);
    if brut {
        println!("Mode brut : blocs hexadecimaux, octets modifies encadres.");
    }
    println!("Ctrl-C pour arreter.\n");

    let mut boitier: Option<S4> = None;
    let mut moniteur = Moniteur::new(brut);
    let mut decodeur = Decodeur::new();
    let mut evenements: Vec<Evenement> = Vec::new();
    let mut tampon = [0u8; usb::EP4_BUFSIZE];
    // Ne pas repeter « en attente » a chaque tour de boucle.
    let mut attente_annoncee = false;
    // Ne pas repeter la meme panne d'armement en boucle.
    let mut derniere_panne = String::new();

    while !stop.load(Ordering::SeqCst) {
        // Phase 1 — pas de boitier : chercher, armer.
        if boitier.is_none() {
            match usb::chercher() {
                Ok(Some(device)) => match S4::armer(device) {
                    Ok(s4) => {
                        annoncer(&s4);
                        moniteur.oublier();
                        decodeur.oublier();
                        boitier = Some(s4);
                        attente_annoncee = false;
                        derniere_panne.clear();
                    }
                    Err(panne) => {
                        // Ne pas noyer la console en repetant la meme panne.
                        let texte = panne.to_string();
                        if texte != derniere_panne {
                            println!("⛔ {texte}");
                            derniere_panne = texte;
                        }
                        sleep(ATTENTE);
                    }
                },
                Ok(None) => {
                    if !attente_annoncee {
                        println!("… S4 absent, en attente. Branche-le, ca se fera tout seul.");
                        attente_annoncee = true;
                    }
                    sleep(ATTENTE);
                }
                Err(e) => {
                    println!("⛔ libusb ne repond pas : {e}");
                    sleep(ATTENTE);
                }
            }
            continue;
        }

        // Phase 2 — boitier arme : lire.
        // L'emprunt de `boitier` se termine avec le bloc ; la reaffectation
        // eventuelle a lieu apres, sinon le compilateur la refuserait.
        let mut perdu = false;
        if let Some(s4) = &boitier {
            match s4.lire(&mut tampon) {
                Lecture::Blocs(paquet) => {
                    moniteur.absorber(paquet);
                    if !brut {
                        evenements.clear();
                        decodeur.absorber(paquet, &mut evenements);
                        for e in &evenements {
                            moniteur.evenement(e);
                        }
                    }
                    moniteur.battre();
                }
                Lecture::Silence => moniteur.battre(),
                Lecture::Perdu => perdu = true,
            }
        }
        if perdu {
            println!("\n🔌 S4 debranche. Retour en attente.\n");
            // Le Drop rend l'interface ici, avant de repartir en recherche.
            boitier = None;
            attente_annoncee = false;
        }
    }

    // Ctrl-C : le Drop de `boitier` relache l'interface ici, pas a la mort du
    // processus. Ca compte pour l'application suivante qui voudra la reserver.
    drop(boitier);
    println!("\nArret. {} blocs recus au total.", moniteur.blocs_recus());
    Ok(())
}

fn annoncer(s4: &S4) {
    println!("✅ S4 arme.");
    match s4.spec {
        Some(s) => {
            println!("   firmware {}  ·  sous-type {}", s.firmware, s.sous_type);
            println!(
                "   {} entrees analogiques · {} encodeurs · {} boutons · {} LED",
                s.entrees_analogiques, s.encodeurs, s.entrees_numeriques, s.sorties_numeriques
            );
            println!(
                "   audio {} in / {} out  ·  MIDI {} in / {} out",
                s.audio_in, s.audio_out, s.midi_in, s.midi_out
            );
        }
        None => println!("   (GET_DEVICE_INFO sans reponse — non bloquant)"),
    }
    println!("   Bouge un controle : seuls les blocs qui changent s'affichent.\n");
}

// ── Mode diagnostic ───────────────────────────────────────────────────────────

fn diagnostic() -> Result<(), Box<dyn std::error::Error>> {
    println!("sonde-s4mk1 — diagnostic\n");

    println!("--- Tous les peripheriques USB vus par libusb ---");
    let liste = usb::inventaire()?;
    for l in &liste {
        println!("{l}");
    }
    println!("  ({} peripherique(s) au total)\n", liste.len());

    let Some(device) = usb::chercher()? else {
        println!("⛔ {:04x}:{:04x} ABSENT de la liste ci-dessus.", usb::VID, usb::PID);
        println!("   La liste n'est pas filtree, donc :");
        println!("   · d'autres appareils y figurent -> libusb fonctionne, c'est le S4");
        println!("     qui ne s'enumere pas (cable, alimentation, boitier) ;");
        println!("   · elle est vide -> c'est libusb ou les droits, et la sonde ne dit");
        println!("     RIEN sur le S4. Reessayer avec sudo.");
        return Ok(());
    };

    let desc = device.device_descriptor()?;
    println!("--- Descripteur ---");
    println!(
        "  classe {:#04x}  sous-classe {:#04x}  protocole {:#04x}  configurations : {}",
        desc.class_code(),
        desc.sub_class_code(),
        desc.protocol_code(),
        desc.num_configurations()
    );

    let plan = usb::planifier(&device)?;
    println!(
        "  alimentation : {}  ·  courant demande : {} mA{}",
        if plan.self_powered {
            "AUTONOME (bloc secteur)"
        } else {
            "PAR LE BUS USB"
        },
        plan.courant_ma,
        if plan.courant_ma > 500 {
            "   ⚠️ au-dela de ce qu'un port 500 mA garantit"
        } else {
            ""
        }
    );
    for l in &plan.lignes {
        println!("{l}");
    }

    println!("\n--- Verification des endpoints attendus par caiaq ---");
    println!("  ep 0x01 OUT (commandes)      : {}", oui_non(plan.ep1_out));
    println!("  ep 0x81 IN  (reponses)       : {}", oui_non(plan.ep1_in));
    println!("  ep 0x84 IN  bulk (controles) : {}", oui_non(plan.ep4_bulk_in));
    println!("  ep 0x08 OUT (LED)            : {}", oui_non(plan.ep8_out));
    println!(
        "  endpoints isochrones (audio) : {}",
        if plan.isochrones.is_empty() {
            "aucun".to_string()
        } else {
            plan.isochrones.join(", ")
        }
    );
    println!("  interface HID dans le descripteur : {}", oui_non(plan.hid));
    match plan.porteur_ep4 {
        Some((i, a)) => println!("  ep 0x84 porte par : interface {i}, alt setting {a}"),
        None => println!("  ⛔ aucun alt setting ne porte l'ep 0x84"),
    }

    println!("\n--- Armement ---");
    match S4::armer(device) {
        Ok(s4) => annoncer(&s4),
        Err(panne) => println!("⛔ {panne}"),
    }
    Ok(())
}

fn oui_non(b: bool) -> &'static str {
    if b {
        "OUI"
    } else {
        "NON"
    }
}
