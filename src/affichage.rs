// SPDX-License-Identifier: GPL-2.0-only
//
// Affichage des blocs bruts. Portage de `sound/usb/caiaq/input.c` :
//   Copyright (c) 2007 Daniel Mack, Karsten Wiese
//
// Domicile provisoire du decodage — il deviendra `decode.rs` a la tache 3,
// quand les blocs produiront des evenements types au lieu de lignes.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// caiaq/input.c : `#define TKS4_MSGBLOCK_SIZE 16`
pub const TAILLE_BLOC: usize = 16;

pub struct Moniteur {
    debut: Instant,
    dernier: HashMap<u16, [u8; TAILLE_BLOC]>,
    blocs: u64,
    dernier_battement: Instant,
    blocs_au_battement: u64,
}

impl Moniteur {
    pub fn new() -> Moniteur {
        let t = Instant::now();
        Moniteur {
            debut: t,
            dernier: HashMap::new(),
            blocs: 0,
            dernier_battement: t,
            blocs_au_battement: 0,
        }
    }

    pub fn blocs_recus(&self) -> u64 {
        self.blocs
    }

    /// Decoupe le paquet en blocs de 16 o et affiche ceux qui ont change.
    pub fn absorber(&mut self, paquet: &[u8]) {
        let mut off = 0;
        while off + TAILLE_BLOC <= paquet.len() {
            let mut bloc = [0u8; TAILLE_BLOC];
            bloc.copy_from_slice(&paquet[off..off + TAILLE_BLOC]);
            off += TAILLE_BLOC;
            self.blocs += 1;

            let id = ((bloc[0] as u16) << 8) | bloc[1] as u16;
            let (premier, modifies) = match self.dernier.get(&id) {
                None => (true, Vec::new()),
                Some(prec) => (
                    false,
                    (0..TAILLE_BLOC)
                        .filter(|&i| prec[i] != bloc[i])
                        .collect::<Vec<usize>>(),
                ),
            };
            if premier || !modifies.is_empty() {
                self.afficher(id, &bloc, &modifies, premier);
            }
            self.dernier.insert(id, bloc);
        }
    }

    /// 🔑 Un silence a l'ecran est ambigu : « plus rien n'arrive » et « ca
    /// arrive mais rien ne change » se ressemblent. Le battement les separe.
    pub fn battre(&mut self) {
        if self.dernier_battement.elapsed() < Duration::from_secs(3) {
            return;
        }
        let nouveaux = self.blocs - self.blocs_au_battement;
        let t = self.debut.elapsed().as_secs_f32();
        if nouveaux == 0 {
            println!("[{t:>7.2}s] … plus aucun bloc ne parvient (total {})", self.blocs);
        } else {
            println!(
                "[{t:>7.2}s] … {nouveaux} blocs recus depuis 3 s, aucun changement (total {})",
                self.blocs
            );
        }
        self.dernier_battement = Instant::now();
        self.blocs_au_battement = self.blocs;
    }

    /// Repartir de zero apres une reconnexion : les valeurs de reference du
    /// boitier precedent n'ont plus cours.
    pub fn oublier(&mut self) {
        self.dernier.clear();
    }

    fn afficher(&self, id: u16, bloc: &[u8; TAILLE_BLOC], modifies: &[usize], premier: bool) {
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
            self.debut.elapsed().as_secs_f32(),
            id,
            if premier { " (ref)" } else { "      " },
            hex
        );

        if id == 0 {
            // 96 boutons alloues par caiaq, bitmask a partir de buf[4].
            // Le boitier n'en declare que 78 : ne pas reprendre 96 sans verifier.
            let enfonces: Vec<usize> = (0..96)
                .filter(|i| ((bloc[4 + i / 8] >> (i % 8)) & 1) == 1)
                .collect();
            println!("                     boutons enfonces : {enfonces:?}");
        } else if id == 1 {
            // Les jogs sont sur 10 bits, pas 16 : l'affichage generique serait faux.
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
}
