// SPDX-License-Identifier: GPL-2.0-only
//
// Decodage des blocs de controle du S4 MK1. Portage de
// `sound/usb/caiaq/input.c` :
//   Copyright (c) 2007 Daniel Mack, Karsten Wiese
//
// Module PUR : aucun acces USB, aucune entree-sortie. C'est ce qui le rend
// testable — et c'est la seule partie du programme qui puisse l'etre sans
// materiel branche.

/// caiaq/input.c : `#define TKS4_MSGBLOCK_SIZE 16`
pub const TAILLE_BLOC: usize = 16;

/// 🔴 caiaq alloue **96** boutons alors que le boitier en declare **78**
/// (`entrees_numeriques` de GET_DEVICE_INFO). On garde 96, et ce n'est pas
/// de la prudence : au repos, la mesure du 2026-08-23 montre les bits
/// **86 et 87** a 1. Tronquer a 78 supprimerait deux controles bien reels.
/// Le compte de 78 ne designe donc pas le nombre de bits utiles.
pub const NB_BOUTONS: usize = 96;
pub const NB_ANALOGIQUES: usize = 36;
pub const NB_ENCODEURS: usize = 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cote {
    Gauche,
    Droit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Evenement {
    /// `index` = numero de bit dans le bloc 0, 0..95.
    Bouton { index: u8, enfonce: bool },
    /// `index` = axe caiaq 0..35, `valeur` sur **12 bits** (0..4095).
    Analogique { index: u8, valeur: u16 },
    /// `index` = encodeur 0..8, `cran` sur 4 bits (0..15), compteur circulaire.
    Encodeur { index: u8, cran: u8 },
    /// Jog : position sur **10 bits** (0..1023), circulaire.
    ///
    /// `horodatage` = les deux octets que **caiaq jette**. Mesure le
    /// 2026-08-23 : compteur d'environ 83 000 pas par seconde, attache a
    /// chaque cran. Decode ici parce que le redecouvrir couterait plus cher
    /// que le transporter ; **aucun consommateur ne s'en sert encore**.
    Jog {
        cote: Cote,
        position: u16,
        horodatage: u16,
    },
}

/// Etiquette de chaque axe analogique. **Source de verite : `MAPPING.md`.**
///
/// ✅ **35 des 36 confrontees au boitier** (tache 4, 2026-08-23/24), un controle
/// a la fois. L'ordre des canaux du S4 est **C-A-B-D de gauche a droite**, ce qui
/// rend les intuitions trompeuses : ne rien deduire ici sans mesure.
///
/// 🔴 **Les 8 axes de FX etaient FAUX dans caiaq** : il annonce `32 = FX1 dry/wet`
/// puis 1, 2, 3, alors que l'ordre reel est inverse — mesure directe du 2026-08-24,
/// le DRY/WET du panneau de gauche tourne seul fait bouger le **bloc 7 offset 5**,
/// soit l'axe **35**. Seul desaccord avec caiaq sur les 35 axes verifies.
///
/// ⬜ **L'axe 4 reste le seul non confronte** : aucun geste ne l'a jamais reveille.
pub const ETIQUETTES: [&str; NB_ANALOGIQUES] = [
    "volume canal D",       // 0
    "volume canal B",       // 1
    "volume canal A",       // 2
    "volume canal C",       // 3
    "volume de boucle",     // 4
    "tempo gauche",         // 5
    "tempo droit",          // 6
    "crossfader",           // 7
    "volume micro",         // 8
    "cue mix",              // 9
    "proximite jog G",      // 10
    "proximite jog D",      // 11
    "EQ D filtre",          // 12
    "EQ D bas",             // 13
    "EQ D medium",          // 14
    "EQ D haut",            // 15
    "FX D 3",               // 16  ⚠️ caiaq disait « FX2 dry/wet » : ordre inverse
    "FX D 2",               // 17
    "FX D 1",               // 18
    "FX D dry/wet",         // 19
    "EQ B filtre",          // 20
    "EQ B bas",             // 21
    "EQ B medium",          // 22
    "EQ B haut",            // 23
    "EQ A filtre",          // 24
    "EQ A bas",             // 25
    "EQ A medium",          // 26
    "EQ A haut",            // 27
    "EQ C filtre",          // 28
    "EQ C bas",             // 29
    "EQ C medium",          // 30
    "EQ C haut",            // 31
    "FX G 3",               // 32  ⚠️ caiaq disait « FX1 dry/wet » : ordre inverse
    "FX G 2",               // 33
    "FX G 1",               // 34
    "FX G dry/wet",         // 35  ✅ mesure directe : bloc 7 offset 5
];

/// Correspondance bloc → axes, telle qu'ecrite dans `snd_usb_caiaq_tks4_dispatch`.
///
/// Chaque couple `(offset, axe)` se lit : la valeur 16 bits gros-boutiste a
/// l'octet `offset * 2` du bloc porte l'axe `axe`.
///
/// 🔴 **CETTE TABLE S'ECARTE DE `caiaq` SUR UN POINT, ET C'EST MESURE.**
/// `caiaq` place l'axe 4 (potard du Loop Recorder) sur le **bloc 2 offset 6**.
/// C'est faux sur ce boitier : l'offset 6 vaut **0 et rien d'autre** sur tous les
/// releves bruts, tandis que l'**offset 5**, que `caiaq` ne lit pas, parcourt la
/// course complete quand on tourne ce potard — mesure du 2026-08-24, potard tourne
/// seul, `releves/boucle-seule.log` : etendue **9 → 4095**, et aucun autre offset,
/// bouton ou encodeur ne bouge. La table lit donc l'offset **5**.
///
/// 🔎 Les autres trous de `caiaq` sont conserves : bloc 3 offsets 1, 2, 5 et
/// bloc 7 offsets 6, 7. Tous **exactement a zero** dans les releves bruts — aucun
/// controle connu ne leur correspond. Le bloc 2 offset 6 les rejoint.
///
/// 🔴 J'avais d'abord annonce que *quatre* trous portaient des valeurs vivantes,
/// en generalisant la seule mesure que j'avais. Un releve brut les a tous montres
/// a zero sauf un. *Une mesure ne se propage pas a ses voisins.*
const AXES_PAR_BLOC: [&[(usize, u8)]; 6] = [
    // bloc 2 — 🔴 l'axe 4 est a l'offset 5, PAS 6 : voir la note ci-dessus.
    &[(1, 0), (2, 1), (3, 2), (4, 3), (5, 4), (7, 7)],
    // bloc 3
    &[(3, 6), (4, 5), (6, 8), (7, 9)],
    // bloc 4
    &[(1, 10), (2, 11), (3, 12), (4, 13), (5, 14), (6, 15), (7, 16)],
    // bloc 5
    &[(1, 17), (2, 18), (3, 19), (4, 20), (5, 21), (6, 22), (7, 23)],
    // bloc 6
    &[(1, 24), (2, 25), (3, 26), (4, 27), (5, 28), (6, 29), (7, 30)],
    // bloc 7
    &[(1, 31), (2, 32), (3, 33), (4, 34), (5, 35)],
];

/// `snd_caiaq_input_report_abs` : gros-boutiste, aux offsets pairs.
fn mot(bloc: &[u8], offset: usize) -> u16 {
    ((bloc[offset * 2] as u16) << 8) | bloc[offset * 2 + 1] as u16
}

pub struct Decodeur {
    boutons: [bool; NB_BOUTONS],
    analogiques: [u16; NB_ANALOGIQUES],
    encodeurs: [u8; NB_ENCODEURS],
    jogs: [u16; 2],
}

impl Default for Decodeur {
    fn default() -> Self {
        Self::new()
    }
}

impl Decodeur {
    /// L'etat initial — tout a zero, tous les boutons relaches — sert de
    /// **reference**. Il n'y a donc pas de « premier passage » a traiter a
    /// part : un fader deja a mi-course emet des son premier bloc, un fader
    /// a zero n'emet rien, et c'est exact dans les deux cas.
    ///
    /// ⚠️ Le pont MIDI (tache 5) aura besoin de l'etat COMPLET a la
    /// connexion d'un client, pas seulement des changements. Ce sera un
    /// accesseur sur cette structure, pas un rejeu d'evenements.
    pub fn new() -> Decodeur {
        Decodeur {
            boutons: [false; NB_BOUTONS],
            analogiques: [0; NB_ANALOGIQUES],
            encodeurs: [0; NB_ENCODEURS],
            jogs: [0; 2],
        }
    }

    /// Repart de zero : apres un debranchement, l'etat retenu ne vaut plus rien.
    pub fn oublier(&mut self) {
        *self = Decodeur::new();
    }

    /// Decoupe le paquet en blocs de 16 o et produit les evenements de
    /// **changement**. Un controle immobile n'emet rien — le boitier envoie
    /// ~600 blocs/s pendant un geste, les republier tous n'aurait aucun sens.
    pub fn absorber(&mut self, paquet: &[u8], sortie: &mut Vec<Evenement>) {
        let mut off = 0;
        while off + TAILLE_BLOC <= paquet.len() {
            self.bloc(&paquet[off..off + TAILLE_BLOC], sortie);
            off += TAILLE_BLOC;
        }
    }

    fn bloc(&mut self, bloc: &[u8], sortie: &mut Vec<Evenement>) {
        let id = ((bloc[0] as u16) << 8) | bloc[1] as u16;
        match id {
            0 => self.boutons(bloc, sortie),
            1 => self.jogs_et_encodeurs(bloc, sortie),
            2..=7 => {
                for &(offset, axe) in AXES_PAR_BLOC[(id - 2) as usize] {
                    let v = mot(bloc, offset);
                    if self.analogiques[axe as usize] != v {
                        self.analogiques[axe as usize] = v;
                        sortie.push(Evenement::Analogique {
                            index: axe,
                            valeur: v,
                        });
                    }
                }
            }
            // Bloc inconnu : caiaq n'en decode que 8 et journalise le reste.
            _ => {}
        }
    }

    fn boutons(&mut self, bloc: &[u8], sortie: &mut Vec<Evenement>) {
        for i in 0..NB_BOUTONS {
            let enfonce = ((bloc[4 + i / 8] >> (i % 8)) & 1) == 1;
            if self.boutons[i] != enfonce {
                self.boutons[i] = enfonce;
                sortie.push(Evenement::Bouton {
                    index: i as u8,
                    enfonce,
                });
            }
        }
    }

    fn jogs_et_encodeurs(&mut self, bloc: &[u8], sortie: &mut Vec<Evenement>) {
        // Chaque jog occupe 4 octets : { poids fort + drapeaux, position,
        // horodatage x2 }. caiaq ne lit que 10 bits de position.
        let jogs = [
            (
                Cote::Gauche,
                0usize,
                bloc[9] as u16 | ((bloc[8] as u16 & 0x3) << 8),
                ((bloc[10] as u16) << 8) | bloc[11] as u16,
            ),
            (
                Cote::Droit,
                1usize,
                bloc[13] as u16 | ((bloc[12] as u16 & 0x3) << 8),
                ((bloc[14] as u16) << 8) | bloc[15] as u16,
            ),
        ];
        for (cote, i, position, horodatage) in jogs {
            if self.jogs[i] != position {
                self.jogs[i] = position;
                sortie.push(Evenement::Jog {
                    cote,
                    position,
                    horodatage,
                });
            }
        }

        let crans = [
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
        for (i, &cran) in crans.iter().enumerate() {
            if self.encodeurs[i] != cran {
                self.encodeurs[i] = cran;
                sortie.push(Evenement::Encodeur {
                    index: i as u8,
                    cran,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Blocs reels, releves sur le boitier du PO le 2026-08-23.
    const BLOC0: [u8; 16] = [
        0x00, 0x00, 0xc0, 0x00, 0x00, 0x00, 0x90, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0,
        0x00,
    ];
    const BLOC1: [u8; 16] = [
        0x00, 0x01, 0xa9, 0x00, 0xf0, 0xff, 0xff, 0xff, 0xc4, 0x9d, 0xee, 0xb8, 0x00, 0x01, 0x00,
        0x04,
    ];
    const BLOC2: [u8; 16] = [
        0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x05, 0x00, 0x00, 0x03,
        0x32,
    ];
    const BLOC4: [u8; 16] = [
        0x00, 0x04, 0x0b, 0xc7, 0x0c, 0x14, 0x05, 0xaa, 0x08, 0x2a, 0x08, 0x23, 0x08, 0x05, 0x00,
        0x00,
    ];

    fn evts(d: &mut Decodeur, bloc: &[u8]) -> Vec<Evenement> {
        let mut v = Vec::new();
        d.absorber(bloc, &mut v);
        v
    }

    #[test]
    fn boutons_au_repos_les_bits_86_et_87_sont_bien_la() {
        // Le boitier declare 78 entrees numeriques, mais les bits 86 et 87
        // sont a 1 au repos : tronquer a 78 perdrait deux controles.
        let mut d = Decodeur::new();
        let enfonces: Vec<u8> = evts(&mut d, &BLOC0)
            .into_iter()
            .filter_map(|e| match e {
                Evenement::Bouton { index, enfonce: true } => Some(index),
                _ => None,
            })
            .collect();
        assert_eq!(enfonces, vec![20, 23, 86, 87]);
    }

    #[test]
    fn le_premier_passage_n_emet_que_ce_qui_differe_du_repos() {
        // Pas de vidage complet : les 92 boutons relaches n'apprennent rien,
        // et l'etat initial du decodeur les decrit deja.
        let mut d = Decodeur::new();
        let n = evts(&mut d, &BLOC0)
            .iter()
            .filter(|e| matches!(e, Evenement::Bouton { .. }))
            .count();
        assert_eq!(n, 4);
    }

    #[test]
    fn un_bouton_relache_emet_bien_son_relachement() {
        // Le silence sur le relachement serait un piege : un bouton reste
        // enfonce pour toujours du point de vue du consommateur.
        let mut d = Decodeur::new();
        evts(&mut d, &BLOC0);
        let mut relache = BLOC0;
        relache[6] = 0x80; // le bit 20 retombe, le 23 reste
        let v = evts(&mut d, &relache);
        assert_eq!(
            v,
            vec![Evenement::Bouton {
                index: 20,
                enfonce: false
            }]
        );
    }

    #[test]
    fn un_bloc_identique_n_emet_plus_rien() {
        let mut d = Decodeur::new();
        evts(&mut d, &BLOC2);
        assert!(evts(&mut d, &BLOC2).is_empty());
    }

    #[test]
    fn bloc_2_le_crossfader_est_a_l_offset_7() {
        let mut d = Decodeur::new();
        let v = evts(&mut d, &BLOC2);
        assert!(v.contains(&Evenement::Analogique {
            index: 7,
            valeur: 0x0332
        }));
        // Les quatre faders de volume sont a zero, donc identiques au repos :
        // rien a signaler. Restent le crossfader et le potard de boucle.
        assert_eq!(v.len(), 2);
    }

    /// 🔴 Barriere sur le SEUL ecart assume vis-a-vis de `caiaq`.
    ///
    /// `caiaq` lit l'axe 4 a l'offset 6 du bloc 2. Mesure du 2026-08-24 : cet
    /// offset vaut 0 et rien d'autre, alors que l'offset 5 parcourt 9 → 4095
    /// quand on tourne le potard du Loop Recorder, seul, boitier au repos.
    ///
    /// Le fixture porte **2053 a l'offset 5 et 0 a l'offset 6** : revenir au
    /// mapping de `caiaq` fait disparaitre l'evenement, et ce test rougit.
    #[test]
    fn bloc_2_le_potard_de_boucle_est_a_l_offset_5_pas_6() {
        let mut d = Decodeur::new();
        let v = evts(&mut d, &BLOC2);
        assert!(
            v.contains(&Evenement::Analogique {
                index: 4,
                valeur: 0x0805
            }),
            "l'axe 4 doit lire l'offset 5 (2053), pas l'offset 6 (0) : {v:?}"
        );
    }

    #[test]
    fn bloc_4_les_eq_du_canal_d_sont_au_cran_central() {
        let mut d = Decodeur::new();
        let v = evts(&mut d, &BLOC4);
        // ~2050 sur 4095 : le detent central des potards.
        for (axe, attendu) in [(13u8, 2090u16), (14, 2083), (15, 2053)] {
            assert!(v.contains(&Evenement::Analogique {
                index: axe,
                valeur: attendu
            }));
        }
        // Capteurs de proximite des jogs, main posee dessus.
        assert!(v.contains(&Evenement::Analogique {
            index: 10,
            valeur: 3015
        }));
    }

    #[test]
    fn bloc_1_position_des_jogs_et_horodatage_conserve() {
        let mut d = Decodeur::new();
        let v = evts(&mut d, &BLOC1);
        assert!(v.contains(&Evenement::Jog {
            cote: Cote::Gauche,
            position: 157,
            horodatage: 0xeeb8
        }));
        assert!(v.contains(&Evenement::Jog {
            cote: Cote::Droit,
            position: 1,
            horodatage: 0x0004
        }));
    }

    #[test]
    fn bloc_1_les_encodeurs_sortent_par_demi_octets() {
        let mut d = Decodeur::new();
        let v = evts(&mut d, &BLOC1);
        // buf[3] = 0x00 -> encodeur 0 = 0, identique au repos, silencieux.
        // buf[4] = 0xf0 -> encodeur 1 = 15 (emis), encodeur 2 = 0 (silencieux).
        assert!(v.contains(&Evenement::Encodeur { index: 1, cran: 15 }));
        assert!(!v
            .iter()
            .any(|e| matches!(e, Evenement::Encodeur { index: 0 | 2, .. })));
    }

    #[test]
    fn seul_l_axe_qui_bouge_est_reemis() {
        let mut d = Decodeur::new();
        evts(&mut d, &BLOC2);
        let mut modifie = BLOC2;
        modifie[15] = 0x33; // crossfader 0x0332 -> 0x0333
        let v = evts(&mut d, &modifie);
        assert_eq!(
            v,
            vec![Evenement::Analogique {
                index: 7,
                valeur: 0x0333
            }]
        );
    }

    #[test]
    fn un_paquet_porte_plusieurs_blocs() {
        let mut d = Decodeur::new();
        let mut paquet = Vec::new();
        paquet.extend_from_slice(&BLOC2);
        paquet.extend_from_slice(&BLOC4);
        let v = evts(&mut d, &paquet);
        assert!(v.iter().any(|e| matches!(
            e,
            Evenement::Analogique { index: 7, .. }
        )));
        assert!(v.iter().any(|e| matches!(
            e,
            Evenement::Analogique { index: 10, .. }
        )));
    }

    #[test]
    fn un_bloc_inconnu_est_ignore_sans_paniquer() {
        let mut d = Decodeur::new();
        let mut inconnu = [0u8; 16];
        inconnu[1] = 42;
        assert!(evts(&mut d, &inconnu).is_empty());
    }

    #[test]
    fn oublier_remet_la_reference_a_zero() {
        let mut d = Decodeur::new();
        evts(&mut d, &BLOC2);
        d.oublier();
        assert!(!evts(&mut d, &BLOC2).is_empty());
    }

    #[test]
    fn les_36_axes_sont_couverts_une_fois_et_une_seule() {
        // Barriere de completude : si un couple (offset, axe) est duplique ou
        // manquant dans AXES_PAR_BLOC, la table est fausse.
        let mut vus = [0u8; NB_ANALOGIQUES];
        for bloc in AXES_PAR_BLOC {
            for &(_, axe) in bloc {
                vus[axe as usize] += 1;
            }
        }
        assert!(
            vus.iter().all(|&n| n == 1),
            "axes mal couverts : {vus:?}"
        );
    }
}
