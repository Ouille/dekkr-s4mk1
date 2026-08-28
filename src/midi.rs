// SPDX-License-Identifier: GPL-2.0-only
//
// Traduction des evenements du S4 en messages MIDI. Tache 5 de SPEC-S4-001.
//
// Module PUR : aucun acces USB, aucun port MIDI, aucune horloge. Le temps
// entre par parametre (`ms`). C'est ce qui rend chaque decision testable —
// et dans ce depot, une decision logee dans un module non montable n'a
// aucune barriere.
//
// L'ouverture du port virtuel vit dans `sortie.rs`, qui ne decide rien.

use crate::decode::{Cote, Evenement, NB_ANALOGIQUES, NB_BOUTONS, NB_ENCODEURS};
use crate::shift::CoucheShift;

// ── Vocabulaire MIDI (SPEC-S4-001 §3) ────────────────────────────────────────

/// Les 96 bits du bloc 0, couche normale.
pub const CANAL_BOUTONS: u8 = 0;
/// Les 96 memes bits, couche SHIFT. Portee GLOBALE : n'importe lequel des deux
/// boutons SHIFT bascule la facade entiere (decision PO du 2026-08-27).
///
/// ⚠️ La couche ne concerne que les BOUTONS. Les axes (canal 2), les jogs et
/// les faders de tempo (3-6) et la rotation des encodeurs (canal 7) n'ont pas
/// d'equivalent shifte dans le vocabulaire de la spec : un BROWSE shifte
/// demanderait de nouveaux canaux, il est hors du perimetre de la tache 6.
pub const CANAL_BOUTONS_SHIFT: u8 = 1;
/// Les 36 axes, 12 bits ramenes a 7.
pub const CANAL_ANALOGIQUES: u8 = 2;
pub const CANAL_JOG_G: u8 = 3;
pub const CANAL_JOG_D: u8 = 4;
pub const CANAL_TEMPO_G: u8 = 5;
pub const CANAL_TEMPO_D: u8 = 6;
/// 🔴 Absent de la spec, ajoute apres mesure : les 9 encodeurs tournent, et
/// `MidiEngine.ts` ne sait pas lire un CC relatif — il n'indexe le declencheur
/// que sur le NUMERO de CC, jamais sur sa valeur. Deux sens = deux notes.
pub const CANAL_ENCODEURS: u8 = 7;

/// 🔴 Le boitier vient d.etre (re)arme — signal a DekkR, qui doit alors REARMER sa
/// prise en douceur : les positions physiques qu.il retenait ne valent plus rien.
///
/// Pourquoi un message dedie : DekkR rearmait sur `MIDIAccess.onstatechange`, en
/// croyant que cela valait « a chaque reconnexion du boitier ». Faux — ce port
/// virtuel est ouvert AVANT le boitier et garde ouvert pour toute la vie du
/// processus (voir `main.rs`), donc debrancher le S4 ne ferme rien et aucun
/// evenement ne part. Recette du 2026-08-28 §8.3 : au rebranchement, le fader
/// SAUTAIT a sa position physique.
///
/// Canal 15 : hors des canaux 0-7 que ce pont emet, donc sans collision possible.
/// Pas de SysEx — DekkR demande `requestMIDIAccess({ sysex: false })`.
pub const CANAL_REARMEMENT: u8 = 15;
pub const CC_REARMEMENT: u8 = 0;

/// Les deux faders de tempo, traites en pitch bend (D7) et non en CC.
pub const AXE_TEMPO_G: u8 = 5;
pub const AXE_TEMPO_D: u8 = 6;

/// 🔴 Bits qui ne doivent JAMAIS produire de note.
///
/// - **20 et 23** : fermes au repos, aucun controle physique correspondant
///   apres balayage complet de la facade (2026-08-25).
/// - **85, 86, 87** : interrupteurs du panneau arriere (MIDI thru, line/phono
///   des decks C et D). Ce sont des reglages materiels, pas des gestes.
///
/// Sans cette liste, **chaque connexion enverrait quatre Note On** : le
/// decodeur signale 20, 23, 86 et 87 comme « enfonces » des le premier bloc,
/// puisque son etat de repos est zero.
pub const BITS_MUETS: [u8; 5] = [20, 23, 85, 86, 87];

// ── Seuils, tous mesures ─────────────────────────────────────────────────────

/// Un pas de CC 7 bits vaut 4095/127 ≈ 32 pas bruts. Le bruit de conversion
/// mesure au repos va de 14 a 27 pas sur 4095 (26 axes, 2026-08-24) : un
/// filtre « la valeur 7 bits a change » laisserait passer une centaine de CC
/// sur des potards que personne ne touche.
pub const HYSTERESIS: u16 = 32;

/// Anti-rebond des entrees numeriques. Mesure le 2026-08-25 : une bascule
/// arriere a produit relache/enfonce/relache dans le meme instant. Les blocs
/// arrivent a ~600/s, soit 1,6 ms : 10 ms couvrent le rebond sans qu'aucun
/// geste humain ne puisse etre avale.
pub const ANTI_REBOND_MS: u64 = 10;

/// Duree pendant laquelle un fader de tempo reste « en mouvement » apres sa
/// derniere emission. Voir `analogique()` : c'est ce qui rend les 12 bits
/// reellement accessibles sans reemettre le bruit au repos.
pub const REPOS_MS: u64 = 200;

// ── Messages ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageMidi {
    NoteOn { canal: u8, note: u8 },
    ControlChange { canal: u8, cc: u8, valeur: u8 },
    /// `valeur` sur **14 bits** (0..16383).
    PitchBend { canal: u8, valeur: u16 },
}

impl MessageMidi {
    /// Le signal « boitier (re)arme » : DekkR doit rearmer sa prise en douceur.
    ///
    /// 🔴 A emettre AVANT toute reannonce d.axe. La tache 9 fait annoncer la
    /// position de tous les axes des la connexion ; si le rearmement arrivait
    /// apres, ces annonces seraient traitees avec l.etat de prise PRECEDENT et
    /// pourraient « prendre » le controle sur un croisement imaginaire — c.est
    /// exactement le defaut que le signal existe pour empecher.
    pub fn rearmement() -> MessageMidi {
        MessageMidi::ControlChange {
            canal: CANAL_REARMEMENT,
            cc: CC_REARMEMENT,
            valeur: 127,
        }
    }

    /// Les trois octets a poser sur le fil.
    pub fn octets(&self) -> [u8; 3] {
        match *self {
            MessageMidi::NoteOn { canal, note } => [0x90 | (canal & 0x0f), note & 0x7f, 127],
            MessageMidi::ControlChange { canal, cc, valeur } => {
                [0xb0 | (canal & 0x0f), cc & 0x7f, valeur & 0x7f]
            }
            // LSB puis MSB, comme l'exige le standard.
            MessageMidi::PitchBend { canal, valeur } => [
                0xe0 | (canal & 0x0f),
                (valeur & 0x7f) as u8,
                ((valeur >> 7) & 0x7f) as u8,
            ],
        }
    }
}

// ── Traducteur ───────────────────────────────────────────────────────────────

/// Etat d'un axe analogique : la derniere valeur **emise** (et non la derniere
/// recue), plus l'instant de la derniere emission pour le regime de mouvement.
#[derive(Clone, Copy, Default)]
struct Axe {
    dernier_emis: Option<u16>,
    derniere_emission_ms: u64,
}

pub struct Traducteur {
    axes: [Axe; NB_ANALOGIQUES],
    dernier_appui_ms: [Option<u64>; NB_BOUTONS],
    dernier_cran: [Option<u8>; NB_ENCODEURS],
    dernier_jog: [Option<u16>; 2],
    shift: CoucheShift,
}

impl Default for Traducteur {
    fn default() -> Self {
        Self::new()
    }
}

impl Traducteur {
    pub fn new() -> Traducteur {
        Traducteur {
            axes: [Axe::default(); NB_ANALOGIQUES],
            dernier_appui_ms: [None; NB_BOUTONS],
            dernier_cran: [None; NB_ENCODEURS],
            dernier_jog: [None; 2],
            shift: CoucheShift::new(),
        }
    }

    /// Repartir de zero apres une reconnexion : les valeurs du boitier
    /// precedent n'ont plus cours, un encodeur dont on ignore le cran de
    /// depart ne doit surtout pas produire un sens de rotation invente, et une
    /// couche SHIFT restee armee enverrait toute la facade sur le canal 1.
    pub fn oublier(&mut self) {
        *self = Traducteur::new();
    }

    /// Traduit un evenement. `ms` = millisecondes depuis le lancement.
    pub fn traduire(&mut self, e: &Evenement, ms: u64, sortie: &mut Vec<MessageMidi>) {
        match *e {
            Evenement::Bouton { index, enfonce } => self.bouton(index, enfonce, ms, sortie),
            Evenement::Analogique { index, valeur } => self.analogique(index, valeur, ms, sortie),
            Evenement::Encodeur { index, cran } => self.encodeur(index, cran, sortie),
            Evenement::Jog { cote, position, .. } => self.jog(cote, position, sortie),
        }
    }

    fn bouton(&mut self, index: u8, enfonce: bool, ms: u64, sortie: &mut Vec<MessageMidi>) {
        // 🔴 AVANT toute autre garde, et en particulier avant la sortie sur
        // `!enfonce` juste dessous. Un SHIFT est un modificateur MAINTENU : son
        // relachement est la moitie de l'information. Lu apres cette sortie, il
        // ne serait JAMAIS vu et la couche resterait armee a vie des le premier
        // appui — un defaut qu'aucun test de la tache 5 n'aurait rougi.
        // Le bouton SHIFT lui-meme n'emet rien : il change la couche (§3).
        if self.shift.enregistrer(index, enfonce) {
            return;
        }
        if BITS_MUETS.contains(&index) {
            return;
        }
        // 🔴 `MidiEngine.ts:145` ne retient que `0x90` avec velocite > 0 : les
        // Note Off sont ignores cote DekkR. Emettre un relachement serait du
        // trafic que personne ne lit — et c'est pourquoi la couche SHIFT est
        // resolue ici et non la-bas (D4).
        if !enfonce {
            return;
        }
        let i = index as usize;
        if let Some(precedent) = self.dernier_appui_ms[i] {
            if ms.saturating_sub(precedent) < ANTI_REBOND_MS {
                return;
            }
        }
        self.dernier_appui_ms[i] = Some(ms);
        // 🔑 La note reste le NUMERO DE BIT dans les deux couches : seul le
        // canal change. Aucune table de conversion, donc aucun endroit ou se
        // tromper — c'est exactement ce qui a rendu la tache 5 sure.
        sortie.push(MessageMidi::NoteOn {
            canal: if self.shift.armee() {
                CANAL_BOUTONS_SHIFT
            } else {
                CANAL_BOUTONS
            },
            note: index,
        });
    }

    fn analogique(&mut self, index: u8, valeur: u16, ms: u64, sortie: &mut Vec<MessageMidi>) {
        let i = index as usize;
        if i >= NB_ANALOGIQUES {
            return;
        }
        let axe = self.axes[i];
        let ecart = match axe.dernier_emis {
            None => u16::MAX,
            Some(p) => valeur.abs_diff(p),
        };
        let franchit = ecart >= HYSTERESIS;

        let canal_tempo = match index {
            AXE_TEMPO_G => Some(CANAL_TEMPO_G),
            AXE_TEMPO_D => Some(CANAL_TEMPO_D),
            _ => None,
        };

        let emettre = match canal_tempo {
            // Les 34 axes ordinaires : hysteresis seche. Sous 32 pas bruts, la
            // valeur 7 bits ne bougerait de toute facon que d'un cran.
            None => franchit,
            // 🔴 Les deux faders de tempo ne peuvent pas se contenter de la
            // meme regle. En 14 bits, une hysteresis de 32 pas plafonnerait la
            // resolution a 128 valeurs utiles — exactement ce que D7 cherchait
            // a depasser. D'ou deux regimes : au REPOS il faut franchir les 32
            // pas (le bruit mesure sur ces deux axes, 14 et 20 pas, ne le peut
            // pas) ; EN MOUVEMENT, tout changement passe, et les 12 bits sont
            // reellement disponibles. On retombe au repos apres REPOS_MS sans
            // emission.
            Some(_) => {
                let en_mouvement = axe.dernier_emis.is_some()
                    && ms.saturating_sub(axe.derniere_emission_ms) < REPOS_MS;
                franchit || (en_mouvement && ecart > 0)
            }
        };

        if !emettre {
            return;
        }
        self.axes[i].dernier_emis = Some(valeur);
        self.axes[i].derniere_emission_ms = ms;

        match canal_tempo {
            Some(canal) => sortie.push(MessageMidi::PitchBend {
                canal,
                // 12 bits -> 14 bits, sans perte.
                valeur: valeur << 2,
            }),
            None => sortie.push(MessageMidi::ControlChange {
                canal: CANAL_ANALOGIQUES,
                cc: index,
                // 4095 >> 5 = 127 : la plage entiere est atteinte aux deux bouts.
                valeur: (valeur >> 5) as u8,
            }),
        }
    }

    /// Le jog est un **compteur de position**, pas une conversion analogique :
    /// il ne bruite pas, donc aucune hysteresis. Chaque cran compte.
    fn jog(&mut self, cote: Cote, position: u16, sortie: &mut Vec<MessageMidi>) {
        let (i, canal) = match cote {
            Cote::Gauche => (0usize, CANAL_JOG_G),
            Cote::Droit => (1usize, CANAL_JOG_D),
        };
        if self.dernier_jog[i] == Some(position) {
            return;
        }
        self.dernier_jog[i] = Some(position);
        sortie.push(MessageMidi::PitchBend {
            canal,
            // 10 bits -> 14 bits.
            valeur: position << 4,
        });
    }

    /// Compteur circulaire sur 4 bits : c'est l'ECART qui porte le sens, pas
    /// la valeur. Deux notes par encodeur, une par sens.
    fn encodeur(&mut self, index: u8, cran: u8, sortie: &mut Vec<MessageMidi>) {
        let i = index as usize;
        if i >= NB_ENCODEURS {
            return;
        }
        let precedent = match self.dernier_cran[i] {
            // 🔴 Premier cran vu : on l'enregistre et on se tait. Le vidage
            // initial donne le cran courant des 9 encodeurs — en tirer un sens
            // de rotation inventerait un geste que personne n'a fait.
            None => {
                self.dernier_cran[i] = Some(cran & 0x0f);
                return;
            }
            Some(p) => p,
        };
        self.dernier_cran[i] = Some(cran & 0x0f);
        let ecart = (cran & 0x0f).wrapping_sub(precedent) & 0x0f;
        // 0 = rien ; 8 = indecidable (autant de crans dans les deux sens).
        if ecart == 0 || ecart == 8 {
            return;
        }
        let (note, pas) = if ecart < 8 {
            (index * 2, ecart)
        } else {
            (index * 2 + 1, 16 - ecart)
        };
        for _ in 0..pas {
            sortie.push(MessageMidi::NoteOn {
                canal: CANAL_ENCODEURS,
                note,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msgs(t: &mut Traducteur, e: Evenement, ms: u64) -> Vec<MessageMidi> {
        let mut v = Vec::new();
        t.traduire(&e, ms, &mut v);
        v
    }

    fn appui(index: u8) -> Evenement {
        Evenement::Bouton {
            index,
            enfonce: true,
        }
    }

    // ── Boutons ──────────────────────────────────────────────────────────────

    #[test]
    fn un_appui_emet_une_note_de_meme_numero_que_le_bit() {
        let mut t = Traducteur::new();
        assert_eq!(
            msgs(&mut t, appui(7), 0),
            vec![MessageMidi::NoteOn {
                canal: 0,
                note: 7
            }]
        );
    }

    #[test]
    fn les_bits_muets_n_emettent_rien_meme_enfonces() {
        // Sans cette regle, CHAQUE connexion enverrait quatre Note On : le
        // decodeur signale 20, 23, 86 et 87 des le premier bloc.
        let mut t = Traducteur::new();
        for bit in BITS_MUETS {
            assert!(
                msgs(&mut t, appui(bit), 0).is_empty(),
                "le bit {bit} a emis"
            );
        }
    }

    #[test]
    fn un_relachement_n_emet_rien() {
        // MidiEngine.ts ignore les Note Off : les emettre serait du trafic
        // que personne ne lit.
        let mut t = Traducteur::new();
        msgs(&mut t, appui(7), 0);
        let e = Evenement::Bouton {
            index: 7,
            enfonce: false,
        };
        assert!(msgs(&mut t, e, 100).is_empty());
    }

    #[test]
    fn le_rebond_d_un_interrupteur_n_emet_qu_une_note() {
        let mut t = Traducteur::new();
        assert_eq!(msgs(&mut t, appui(30), 1000).len(), 1);
        // Rebond mesure : quelques millisecondes.
        assert!(msgs(&mut t, appui(30), 1003).is_empty());
        assert!(msgs(&mut t, appui(30), 1009).is_empty());
        // Un vrai second appui, lui, doit passer.
        assert_eq!(msgs(&mut t, appui(30), 1011).len(), 1);
    }

    #[test]
    fn l_anti_rebond_est_par_bouton_et_non_global() {
        let mut t = Traducteur::new();
        assert_eq!(msgs(&mut t, appui(30), 1000).len(), 1);
        // Un autre bouton dans la meme milliseconde n'a rien a voir.
        assert_eq!(msgs(&mut t, appui(31), 1000).len(), 1);
    }

    // ── Couche SHIFT (tache 6) ───────────────────────────────────────────────

    fn relache(index: u8) -> Evenement {
        Evenement::Bouton {
            index,
            enfonce: false,
        }
    }

    /// SHIFT gauche maintenu.
    fn shift_g() -> Evenement {
        appui(crate::shift::BITS_SHIFT[0])
    }

    #[test]
    fn shift_maintenu_le_bouton_part_sur_la_couche_shiftee() {
        let mut t = Traducteur::new();
        msgs(&mut t, shift_g(), 0);
        assert_eq!(
            msgs(&mut t, appui(7), 10),
            vec![MessageMidi::NoteOn {
                canal: CANAL_BOUTONS_SHIFT,
                note: 7
            }]
        );
    }

    #[test]
    fn shift_relache_le_bouton_repart_sur_la_couche_normale() {
        // 🔴 LE test de la tache 6. Il rougit si l'etat SHIFT est lu apres la
        // sortie sur `!enfonce` : le relachement passerait inapercu et la
        // couche resterait armee a vie. Tous les tests de la tache 5
        // resteraient verts pendant ce temps.
        let mut t = Traducteur::new();
        msgs(&mut t, shift_g(), 0);
        msgs(&mut t, appui(7), 10);
        msgs(&mut t, relache(7), 20);
        msgs(&mut t, relache(crate::shift::BITS_SHIFT[0]), 30);
        assert_eq!(
            msgs(&mut t, appui(7), 40),
            vec![MessageMidi::NoteOn {
                canal: CANAL_BOUTONS,
                note: 7
            }],
            "la couche SHIFT est restee armee apres le relachement"
        );
    }

    #[test]
    fn les_bits_shift_n_emettent_jamais_rien() {
        // Le bouton SHIFT change la couche, il n'est pas une action.
        let mut t = Traducteur::new();
        for bit in crate::shift::BITS_SHIFT {
            assert!(
                msgs(&mut t, appui(bit), 0).is_empty(),
                "le bit {bit} a emis"
            );
            assert!(
                msgs(&mut t, relache(bit), 100).is_empty(),
                "le relachement du bit {bit} a emis"
            );
        }
    }

    #[test]
    fn le_shift_droit_arme_aussi_un_bouton_du_deck_gauche() {
        // Portee GLOBALE (decision PO du 2026-08-27) : il n'y a qu'une couche,
        // et le SHIFT d'un deck vaut pour toute la facade.
        let mut t = Traducteur::new();
        msgs(&mut t, appui(crate::shift::BITS_SHIFT[1]), 0);
        assert_eq!(
            msgs(&mut t, appui(7), 10),
            vec![MessageMidi::NoteOn {
                canal: CANAL_BOUTONS_SHIFT,
                note: 7
            }]
        );
    }

    #[test]
    fn un_seul_des_deux_shift_relache_ne_desarme_pas_la_couche() {
        let mut t = Traducteur::new();
        msgs(&mut t, appui(crate::shift::BITS_SHIFT[0]), 0);
        msgs(&mut t, appui(crate::shift::BITS_SHIFT[1]), 10);
        msgs(&mut t, relache(crate::shift::BITS_SHIFT[0]), 20);
        assert_eq!(
            msgs(&mut t, appui(7), 30),
            vec![MessageMidi::NoteOn {
                canal: CANAL_BOUTONS_SHIFT,
                note: 7
            }],
            "relacher un SHIFT a fait tomber la couche alors que l'autre tenait"
        );
    }

    #[test]
    fn un_bit_muet_reste_muet_sous_shift() {
        // La couche ne ressuscite pas les bits exclus : sinon chaque connexion
        // SHIFT maintenu enverrait quatre Note On sur le canal 1.
        let mut t = Traducteur::new();
        msgs(&mut t, shift_g(), 0);
        for bit in BITS_MUETS {
            assert!(
                msgs(&mut t, appui(bit), 10).is_empty(),
                "le bit muet {bit} a emis sous SHIFT"
            );
        }
    }

    #[test]
    fn oublier_desarme_la_couche_shift() {
        // Une reconnexion SHIFT maintenu laisserait toute la facade sur le
        // canal 1 : le boitier ne renverra jamais le relachement d'avant.
        let mut t = Traducteur::new();
        msgs(&mut t, shift_g(), 0);
        t.oublier();
        assert_eq!(
            msgs(&mut t, appui(7), 100),
            vec![MessageMidi::NoteOn {
                canal: CANAL_BOUTONS,
                note: 7
            }]
        );
    }

    #[test]
    fn l_anti_rebond_ne_distingue_pas_les_couches() {
        // C'est le meme bouton physique : changer de couche entre deux appuis
        // ne doit pas rouvrir la fenetre d'anti-rebond.
        let mut t = Traducteur::new();
        assert_eq!(msgs(&mut t, appui(30), 1000).len(), 1);
        msgs(&mut t, shift_g(), 1002);
        assert!(msgs(&mut t, appui(30), 1004).is_empty());
        assert_eq!(msgs(&mut t, appui(30), 1011).len(), 1);
    }

    // ── Axes ordinaires ──────────────────────────────────────────────────────

    fn axe(index: u8, valeur: u16) -> Evenement {
        Evenement::Analogique { index, valeur }
    }

    #[test]
    fn la_premiere_valeur_d_un_axe_est_toujours_emise() {
        let mut t = Traducteur::new();
        assert_eq!(
            msgs(&mut t, axe(13, 2048), 0),
            vec![MessageMidi::ControlChange {
                canal: 2,
                cc: 13,
                valeur: 64
            }]
        );
    }

    #[test]
    fn sous_le_seuil_rien_ne_part_au_dela_il_part() {
        let mut t = Traducteur::new();
        msgs(&mut t, axe(13, 2000), 0);
        assert!(msgs(&mut t, axe(13, 2031), 10).is_empty(), "31 pas");
        assert_eq!(msgs(&mut t, axe(13, 2032), 20).len(), 1, "32 pas");
    }

    #[test]
    fn le_bruit_au_repos_ne_produit_aucun_message() {
        // Amplitude mesuree le 2026-08-24 sur 26 axes laisses au repos :
        // 14 a 27 pas. Le seuil doit tous les avaler.
        let mut t = Traducteur::new();
        msgs(&mut t, axe(29, 2089), 0);
        for (i, v) in [2075u16, 2096, 2062, 2089, 2102].iter().enumerate() {
            assert!(
                msgs(&mut t, axe(29, *v), (i as u64 + 1) * 500).is_empty(),
                "le bruit {v} a emis"
            );
        }
    }

    #[test]
    fn l_ecart_se_compte_depuis_la_derniere_valeur_emise() {
        // 🔴 Le piege : compter depuis la derniere valeur RECUE laisserait une
        // derive lente franchir le seuil un pas a la fois sans jamais l'atteindre.
        let mut t = Traducteur::new();
        msgs(&mut t, axe(13, 2000), 0);
        for v in [2010u16, 2020, 2029] {
            assert!(msgs(&mut t, axe(13, v), 10).is_empty());
        }
        // 2029 -> 2032 : trois pas depuis la derniere RECUE, 32 depuis l'EMISE.
        assert_eq!(msgs(&mut t, axe(13, 2032), 20).len(), 1);
    }

    #[test]
    fn les_deux_extremes_sont_atteignables() {
        let mut t = Traducteur::new();
        let bas = msgs(&mut t, axe(13, 0), 0);
        assert_eq!(
            bas,
            vec![MessageMidi::ControlChange {
                canal: 2,
                cc: 13,
                valeur: 0
            }]
        );
        let haut = msgs(&mut t, axe(13, 4095), 10);
        assert_eq!(
            haut,
            vec![MessageMidi::ControlChange {
                canal: 2,
                cc: 13,
                valeur: 127
            }]
        );
    }

    // ── Faders de tempo ──────────────────────────────────────────────────────

    #[test]
    fn un_fader_de_tempo_part_en_pitch_bend_sur_son_canal() {
        let mut t = Traducteur::new();
        assert_eq!(
            msgs(&mut t, axe(AXE_TEMPO_G, 2048), 0),
            vec![MessageMidi::PitchBend {
                canal: CANAL_TEMPO_G,
                valeur: 2048 << 2
            }]
        );
        assert_eq!(
            msgs(&mut t, axe(AXE_TEMPO_D, 2048), 0),
            vec![MessageMidi::PitchBend {
                canal: CANAL_TEMPO_D,
                valeur: 2048 << 2
            }]
        );
    }

    #[test]
    fn en_mouvement_le_tempo_emet_ses_petits_pas() {
        // C'est tout l'objet du second regime : sans lui, les 14 bits de D7
        // ne vaudraient pas mieux qu'un CC.
        let mut t = Traducteur::new();
        msgs(&mut t, axe(AXE_TEMPO_G, 2000), 0);
        // Un grand pas ouvre le regime de mouvement.
        assert_eq!(msgs(&mut t, axe(AXE_TEMPO_G, 2040), 10).len(), 1);
        // Puis les pas fins passent.
        assert_eq!(msgs(&mut t, axe(AXE_TEMPO_G, 2043), 30).len(), 1);
        assert_eq!(msgs(&mut t, axe(AXE_TEMPO_G, 2044), 50).len(), 1);
    }

    #[test]
    fn au_repos_le_tempo_redevient_sourd_au_bruit() {
        let mut t = Traducteur::new();
        msgs(&mut t, axe(AXE_TEMPO_G, 2000), 0);
        msgs(&mut t, axe(AXE_TEMPO_G, 2040), 10);
        // Plus rien pendant REPOS_MS : le fader est lache.
        assert!(
            msgs(&mut t, axe(AXE_TEMPO_G, 2050), 10 + REPOS_MS + 1).is_empty(),
            "le bruit d'apres-mouvement a emis"
        );
    }

    // ── Jogs ─────────────────────────────────────────────────────────────────

    #[test]
    fn chaque_cran_de_jog_est_emis_sans_hysteresis() {
        let mut t = Traducteur::new();
        let e = Evenement::Jog {
            cote: Cote::Gauche,
            position: 100,
            horodatage: 0,
        };
        assert_eq!(
            msgs(&mut t, e, 0),
            vec![MessageMidi::PitchBend {
                canal: CANAL_JOG_G,
                valeur: 100 << 4
            }]
        );
        // Un seul cran d'ecart : un fader analogique l'aurait avale.
        let e = Evenement::Jog {
            cote: Cote::Gauche,
            position: 101,
            horodatage: 0,
        };
        assert_eq!(msgs(&mut t, e, 1).len(), 1);
    }

    #[test]
    fn un_jog_immobile_n_emet_pas_deux_fois() {
        let mut t = Traducteur::new();
        let e = Evenement::Jog {
            cote: Cote::Droit,
            position: 512,
            horodatage: 0,
        };
        assert_eq!(msgs(&mut t, e, 0).len(), 1);
        assert!(msgs(&mut t, e, 5).is_empty());
    }

    // ── Encodeurs ────────────────────────────────────────────────────────────

    fn enc(index: u8, cran: u8) -> Evenement {
        Evenement::Encodeur { index, cran }
    }

    #[test]
    fn le_premier_cran_vu_n_invente_aucun_sens() {
        // Le vidage initial donne le cran des 9 encodeurs sans que personne
        // n'ait tourne quoi que ce soit.
        let mut t = Traducteur::new();
        assert!(msgs(&mut t, enc(4, 15), 0).is_empty());
    }

    #[test]
    fn un_cran_dans_chaque_sens_donne_deux_notes_differentes() {
        let mut t = Traducteur::new();
        msgs(&mut t, enc(4, 8), 0);
        assert_eq!(
            msgs(&mut t, enc(4, 9), 10),
            vec![MessageMidi::NoteOn {
                canal: CANAL_ENCODEURS,
                note: 8
            }]
        );
        assert_eq!(
            msgs(&mut t, enc(4, 8), 20),
            vec![MessageMidi::NoteOn {
                canal: CANAL_ENCODEURS,
                note: 9
            }]
        );
    }

    #[test]
    fn le_compteur_reboucle_sans_inverser_le_sens() {
        // 15 -> 0 est un cran vers le haut, pas quinze vers le bas.
        let mut t = Traducteur::new();
        msgs(&mut t, enc(0, 15), 0);
        assert_eq!(
            msgs(&mut t, enc(0, 0), 10),
            vec![MessageMidi::NoteOn {
                canal: CANAL_ENCODEURS,
                note: 0
            }]
        );
        // Et 0 -> 15 est bien un cran vers le bas.
        assert_eq!(
            msgs(&mut t, enc(0, 15), 20),
            vec![MessageMidi::NoteOn {
                canal: CANAL_ENCODEURS,
                note: 1
            }]
        );
    }

    #[test]
    fn plusieurs_crans_d_un_coup_donnent_autant_de_notes() {
        let mut t = Traducteur::new();
        msgs(&mut t, enc(1, 0), 0);
        assert_eq!(msgs(&mut t, enc(1, 3), 10).len(), 3);
    }

    #[test]
    fn un_ecart_de_huit_est_indecidable_donc_ignore() {
        let mut t = Traducteur::new();
        msgs(&mut t, enc(1, 0), 0);
        assert!(msgs(&mut t, enc(1, 8), 10).is_empty());
    }

    // ── Encodage sur le fil ──────────────────────────────────────────────────

    #[test]
    fn le_pitch_bend_pose_le_lsb_avant_le_msb() {
        let m = MessageMidi::PitchBend {
            canal: 3,
            valeur: 0x2000,
        };
        assert_eq!(m.octets(), [0xe3, 0x00, 0x40]);
    }

    #[test]
    fn une_note_part_avec_une_velocite_non_nulle() {
        // Velocite 0 vaut Note Off pour MidiEngine.ts : la note serait ignoree.
        let m = MessageMidi::NoteOn { canal: 0, note: 7 };
        assert_eq!(m.octets(), [0x90, 7, 127]);
        assert!(m.octets()[2] > 0);
    }

    // ── Reconnexion ──────────────────────────────────────────────────────────

    #[test]
    fn oublier_remet_les_encodeurs_dans_l_ignorance() {
        let mut t = Traducteur::new();
        msgs(&mut t, enc(4, 8), 0);
        t.oublier();
        // Apres reconnexion, le premier cran redevient une simple prise de
        // reference : sinon on inventerait une rotation au rebranchement.
        assert!(msgs(&mut t, enc(4, 12), 100).is_empty());
    }
}

#[cfg(test)]
mod tests_rearmement {
    use super::*;

    // SPEC-S4-001 §8.3 — recette du 2026-08-28 : au rebranchement du boitier, le fader de
    // volume SAUTAIT a sa position physique au lieu de redemander un croisement. DekkR
    // rearmait sur `onstatechange`, qui ne part jamais puisque ce pont garde son port
    // virtuel ouvert. D'ou ce signal explicite.

    #[test]
    fn le_signal_de_rearmement_est_un_cc_sur_le_canal_reserve() {
        let octets = MessageMidi::rearmement().octets();
        assert_eq!(octets[0], 0xb0 | CANAL_REARMEMENT, "doit etre un Control Change sur 15");
        assert_eq!(octets[1], CC_REARMEMENT, "numero de CC reserve");
    }

    #[test]
    fn le_canal_de_rearmement_n_entre_en_collision_avec_aucun_canal_emis() {
        // 🔴 Le garde-fou du choix de conception. Les sept canaux ci-dessous portent des
        // gestes reels ; si le signal descendait dans cette plage, une rotation d'encodeur ou
        // un mouvement de fader pourrait passer pour une reconnexion et rearmer en plein mix.
        for canal in [
            CANAL_BOUTONS,
            CANAL_BOUTONS_SHIFT,
            CANAL_ANALOGIQUES,
            CANAL_JOG_G,
            CANAL_JOG_D,
            CANAL_TEMPO_G,
            CANAL_TEMPO_D,
            CANAL_ENCODEURS,
        ] {
            assert_ne!(canal, CANAL_REARMEMENT, "collision de canal");
        }
    }

    #[test]
    fn le_signal_tient_dans_les_sept_bits_du_midi() {
        let octets = MessageMidi::rearmement().octets();
        assert!(octets[1] <= 0x7f);
        assert!(octets[2] <= 0x7f);
    }
}
