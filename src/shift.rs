// SPDX-License-Identifier: GPL-2.0-only
//
// Couche SHIFT. Tache 6 de SPEC-S4-001.
//
// Module PUR : il ne connait ni le MIDI, ni l'USB, ni l'horloge. Il ne repond
// qu'a une question — « la couche est-elle armee ? » — et c'est ce qui la rend
// testable sans monter quoi que ce soit.
//
// D4 de la spec : la couche est resolue ICI, pas dans DekkR. `MidiEngine.ts`
// ignore les Note Off (« pour eviter le double-declenchement ») : un
// modificateur MAINTENU ne peut donc pas s'y exprimer. Le pont envoie deux
// notes differentes, une par couche, et le front n'a rien a savoir.

/// Les deux boutons SHIFT de la facade, releves le 2026-08-25 : bit 1 pour le
/// deck gauche, bit 57 pour le deck droit (voir `MAPPING.md`).
pub const BITS_SHIFT: [u8; 2] = [1, 57];

/// Etat des deux boutons SHIFT.
///
/// 🔴 Deux booleens et non un seul. La couche est armee des qu'**un** des deux
/// est maintenu (portee globale, validee PO le 2026-08-27) : avec un booleen
/// unique, relacher un SHIFT alors que l'autre est encore tenu ferait tomber la
/// couche en plein geste.
#[derive(Clone, Copy, Default)]
pub struct CoucheShift {
    /// Un booleen par entree de `BITS_SHIFT`, dans le meme ordre.
    maintenus: [bool; 2],
}

impl CoucheShift {
    pub fn new() -> CoucheShift {
        CoucheShift::default()
    }

    /// Enregistre l'etat d'un bit du bloc 0.
    ///
    /// Retourne `true` si ce bit **est** un SHIFT : l'appelant doit alors se
    /// taire, le bouton SHIFT lui-meme n'emet rien (SPEC-S4-001 §3).
    pub fn enregistrer(&mut self, index: u8, enfonce: bool) -> bool {
        match BITS_SHIFT.iter().position(|&bit| bit == index) {
            None => false,
            Some(i) => {
                self.maintenus[i] = enfonce;
                true
            }
        }
    }

    /// Vrai tant qu'au moins un SHIFT est maintenu.
    pub fn armee(&self) -> bool {
        self.maintenus.iter().any(|&m| m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_bit_ordinaire_n_est_pas_consomme() {
        let mut c = CoucheShift::new();
        assert!(
            !c.enregistrer(7, true),
            "le PLAY gauche a ete pris pour un SHIFT"
        );
        assert!(!c.armee());
    }

    #[test]
    fn chacun_des_deux_bits_shift_arme_la_couche() {
        // Portee GLOBALE : les deux SHIFT arment la meme et unique couche.
        for bit in BITS_SHIFT {
            let mut c = CoucheShift::new();
            assert!(
                c.enregistrer(bit, true),
                "le bit {bit} n'a pas ete consomme"
            );
            assert!(c.armee(), "le bit {bit} n'a pas arme la couche");
        }
    }

    #[test]
    fn le_relachement_desarme() {
        let mut c = CoucheShift::new();
        c.enregistrer(1, true);
        c.enregistrer(1, false);
        assert!(!c.armee());
    }

    #[test]
    fn la_couche_tient_tant_qu_un_des_deux_est_maintenu() {
        // 🔴 Le cas qu'un booleen unique aurait rate : les deux pouces sur les
        // deux SHIFT, on en leve un, la couche doit tenir.
        let mut c = CoucheShift::new();
        c.enregistrer(1, true);
        c.enregistrer(57, true);
        c.enregistrer(1, false);
        assert!(c.armee(), "relacher un SHIFT a fait tomber la couche");
        c.enregistrer(57, false);
        assert!(!c.armee(), "les deux relaches, la couche tient encore");
    }

    #[test]
    fn au_repos_la_couche_est_desarmee() {
        // L'etat de repos du decodeur est zero : aucun bloc recu ne doit
        // suffire a armer la couche.
        assert!(!CoucheShift::new().armee());
    }
}
