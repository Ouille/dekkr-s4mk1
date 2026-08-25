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
mod journal;
mod midi;
mod sortie;
mod usb;

// `dire!` est `#[macro_export]`, donc deja dans la racine du crate : pas de
// `use` ici, il entrerait en conflit avec lui-meme. Les sous-modules, eux,
// doivent l'importer (`use crate::dire;`).
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

use affichage::Moniteur;
use decode::{Decodeur, Evenement};
use midi::{MessageMidi, Traducteur};
use sortie::PortMidi;
use usb::{Lecture, S4};

/// Rythme d'attente quand le boitier est absent. Assez lent pour ne rien
/// couter, assez vif pour que le branchement paraisse immediat.
const ATTENTE: Duration = Duration::from_millis(500);

/// Journal par defaut. TRONQUE a chaque lancement : pour garder une seance,
/// la nommer avec `--journal`.
const JOURNAL_PAR_DEFAUT: &str = "sonde.log";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("sonde-s4mk1 — Traktor Kontrol S4 MK1");
        println!("  (sans argument)     pont MIDI + affichage des controles decodes");
        println!("  --sans-midi         n'ouvre aucun port MIDI (sonde seule)");
        println!("  --brut              blocs de 16 octets en hexadecimal, SANS pont MIDI");
        println!("  --diagnostic        dump du descripteur USB, puis sort");
        println!("  --journal <fichier> journal a ecrire (defaut : {JOURNAL_PAR_DEFAUT})");
        println!("  --sans-journal      console seule");
        return Ok(());
    }

    // Tout ce qui suit part a la fois a l'ecran et dans le journal.
    let chemin = if args.iter().any(|a| a == "--sans-journal") {
        None
    } else {
        Some(
            args.iter()
                .position(|a| a == "--journal")
                .and_then(|i| args.get(i + 1))
                .cloned()
                .unwrap_or_else(|| JOURNAL_PAR_DEFAUT.to_string()),
        )
    };
    if let Some(c) = &chemin {
        match journal::ouvrir(c) {
            // 🔴 Annoncer le fichier AVANT d'ecrire dedans : sans ca, on ne
            // sait pas ou est parti ce qu'on vient de lire a l'ecran.
            Ok(()) => println!("Journal : {c}  (ecrase a chaque lancement)\n"),
            Err(e) => println!("⚠️ journal {c} impossible ({e}) — console seule\n"),
        }
    }

    let resultat = if args.iter().any(|a| a == "--diagnostic" || a == "-d") {
        diagnostic()
    } else {
        let brut = args.iter().any(|a| a == "--brut" || a == "-b");
        // Le mode brut n'assemble aucun evenement : il n'y a rien a traduire.
        let midi = !brut && !args.iter().any(|a| a == "--sans-midi");
        surveiller(brut, midi)
    };

    journal::vider();
    if let Some(c) = &chemin {
        println!("Journal ecrit dans {c}");
    }
    resultat
}

// ── Boucle de surveillance ────────────────────────────────────────────────────

/// Tourne indefiniment : attend le boitier, l'arme, le lit, encaisse les
/// debranchements, et rend l'interface a l'arret.
///
/// 🔑 Le programme ne depend PAS de l'ordre de branchement. On peut le lancer
/// avant le S4, le debrancher, le rebrancher : il se rearme seul.
fn surveiller(brut: bool, midi: bool) -> Result<(), Box<dyn std::error::Error>> {
    let stop = Arc::new(AtomicBool::new(false));
    {
        let stop = Arc::clone(&stop);
        ctrlc::set_handler(move || stop.store(true, Ordering::SeqCst))?;
    }

    dire!("sonde-s4mk1 — surveillance ({:04x}:{:04x})", usb::VID, usb::PID);
    if brut {
        dire!("Mode brut : blocs hexadecimaux, octets modifies encadres.");
    }

    // 🔴 Ouvrir le port AVANT d'attendre le boitier : le port virtuel doit
    // exister au moment ou Chrome enumere le MIDI, sinon DekkR ne verra rien
    // tant qu'il n'aura pas ete relance.
    let mut port: Option<PortMidi> = None;
    if midi {
        match PortMidi::ouvrir() {
            Ok(p) => {
                dire!("🎹 Port MIDI virtuel « {} » ouvert.", sortie::NOM_PORT);
                port = Some(p);
            }
            // Perdre le MIDI ne justifie pas de perdre le diagnostic.
            Err(e) => dire!("⚠️ MIDI indisponible ({e}) — sonde seule."),
        }
    }
    dire!("Ctrl-C pour arreter.\n");

    let depart = Instant::now();
    let mut boitier: Option<S4> = None;
    let mut moniteur = Moniteur::new(brut);
    let mut decodeur = Decodeur::new();
    let mut traducteur = Traducteur::new();
    let mut evenements: Vec<Evenement> = Vec::new();
    let mut messages: Vec<MessageMidi> = Vec::new();
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
                        // Les valeurs du boitier precedent n'ont plus cours —
                        // et un encodeur dont on ignore le cran de depart ne
                        // doit pas produire une rotation au rebranchement.
                        traducteur.oublier();
                        boitier = Some(s4);
                        attente_annoncee = false;
                        derniere_panne.clear();
                    }
                    Err(panne) => {
                        // Ne pas noyer la console en repetant la meme panne.
                        let texte = panne.to_string();
                        if texte != derniere_panne {
                            dire!("⛔ {texte}");
                            derniere_panne = texte;
                        }
                        sleep(ATTENTE);
                    }
                },
                Ok(None) => {
                    if !attente_annoncee {
                        dire!("… S4 absent, en attente. Branche-le, ca se fera tout seul.");
                        attente_annoncee = true;
                    }
                    sleep(ATTENTE);
                }
                Err(e) => {
                    dire!("⛔ libusb ne repond pas : {e}");
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
                        let ms = depart.elapsed().as_millis() as u64;
                        for e in &evenements {
                            moniteur.evenement(e);
                            if let Some(p) = &mut port {
                                messages.clear();
                                traducteur.traduire(e, ms, &mut messages);
                                for m in &messages {
                                    p.emettre(m);
                                }
                            }
                        }
                    }
                    moniteur.battre();
                }
                Lecture::Silence => moniteur.battre(),
                Lecture::Perdu => perdu = true,
            }
        }
        if perdu {
            dire!("\n🔌 S4 debranche. Retour en attente.\n");
            // Le Drop rend l'interface ici, avant de repartir en recherche.
            boitier = None;
            attente_annoncee = false;
        }
    }

    // Ctrl-C : le Drop de `boitier` relache l'interface ici, pas a la mort du
    // processus. Ca compte pour l'application suivante qui voudra la reserver.
    drop(boitier);
    dire!("\nArret. {} blocs recus au total.", moniteur.blocs_recus());
    Ok(())
}

fn annoncer(s4: &S4) {
    dire!("✅ S4 arme.");
    match s4.spec {
        Some(s) => {
            dire!("   firmware {}  ·  sous-type {}", s.firmware, s.sous_type);
            dire!(
                "   {} entrees analogiques · {} encodeurs · {} boutons · {} LED",
                s.entrees_analogiques, s.encodeurs, s.entrees_numeriques, s.sorties_numeriques
            );
            dire!(
                "   audio analogique {} in / {} out  ·  numerique {} in / {} out",
                s.audio_in, s.audio_out, s.audio_num_in, s.audio_num_out
            );
            // `alignement` = `data_alignment` de caiaq_device_spec. caiaq s'en
            // sert pour placer les trames audio : il faudra le relire a la
            // SPEC-S4-004, autant l'avoir sous les yeux des maintenant.
            dire!(
                "   MIDI {} in / {} out  ·  alignement des donnees {}",
                s.midi_in, s.midi_out, s.alignement
            );
        }
        None => dire!("   (GET_DEVICE_INFO sans reponse — non bloquant)"),
    }
    dire!("   Bouge un controle : seuls les blocs qui changent s'affichent.\n");
}

// ── Mode diagnostic ───────────────────────────────────────────────────────────

fn diagnostic() -> Result<(), Box<dyn std::error::Error>> {
    dire!("sonde-s4mk1 — diagnostic\n");

    dire!("--- Tous les peripheriques USB vus par libusb ---");
    let liste = usb::inventaire()?;
    for l in &liste {
        dire!("{l}");
    }
    dire!("  ({} peripherique(s) au total)\n", liste.len());

    let Some(device) = usb::chercher()? else {
        dire!("⛔ {:04x}:{:04x} ABSENT de la liste ci-dessus.", usb::VID, usb::PID);
        dire!("   La liste n'est pas filtree, donc :");
        dire!("   · d'autres appareils y figurent -> libusb fonctionne, c'est le S4");
        dire!("     qui ne s'enumere pas (cable, alimentation, boitier) ;");
        dire!("   · elle est vide -> c'est libusb ou les droits, et la sonde ne dit");
        dire!("     RIEN sur le S4. Reessayer avec sudo.");
        return Ok(());
    };

    let desc = device.device_descriptor()?;
    dire!("--- Descripteur ---");
    dire!(
        "  classe {:#04x}  sous-classe {:#04x}  protocole {:#04x}  configurations : {}",
        desc.class_code(),
        desc.sub_class_code(),
        desc.protocol_code(),
        desc.num_configurations()
    );

    let plan = usb::planifier(&device)?;
    dire!(
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
        dire!("{l}");
    }

    dire!("\n--- Verification des endpoints attendus par caiaq ---");
    dire!("  ep 0x01 OUT (commandes)      : {}", oui_non(plan.ep1_out));
    dire!("  ep 0x81 IN  (reponses)       : {}", oui_non(plan.ep1_in));
    dire!("  ep 0x84 IN  bulk (controles) : {}", oui_non(plan.ep4_bulk_in));
    dire!("  ep 0x08 OUT (LED)            : {}", oui_non(plan.ep8_out));
    dire!(
        "  endpoints isochrones (audio) : {}",
        if plan.isochrones.is_empty() {
            "aucun".to_string()
        } else {
            plan.isochrones.join(", ")
        }
    );
    dire!("  interface HID dans le descripteur : {}", oui_non(plan.hid));
    match plan.porteur_ep4 {
        Some((i, a)) => dire!("  ep 0x84 porte par : interface {i}, alt setting {a}"),
        None => dire!("  ⛔ aucun alt setting ne porte l'ep 0x84"),
    }

    dire!("\n--- Armement ---");
    match S4::armer(device) {
        Ok(s4) => annoncer(&s4),
        Err(panne) => dire!("⛔ {panne}"),
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
