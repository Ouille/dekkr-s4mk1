// SPDX-License-Identifier: GPL-2.0-only
//
// Port MIDI virtuel. Cette couche ne DECIDE rien : elle ouvre le port et
// pose les octets que `midi.rs` a produits. Tout ce qui se teste est la-bas.

use crate::midi::MessageMidi;

/// D8 : nom fixe. `useMidi.ts:44` cherche le profil par nom EXACT du
/// peripherique — un nom variable (numero de serie, index) casserait
/// l'appariement avec `s4mk1.json`.
pub const NOM_PORT: &str = "DekkR S4 MK1";

pub struct PortMidi {
    #[cfg(unix)]
    connexion: midir::MidiOutputConnection,
}

impl PortMidi {
    /// Ouvre le port virtuel. En cas d'echec, le pont doit continuer a
    /// fonctionner en sonde : perdre le MIDI ne justifie pas de perdre aussi
    /// le diagnostic.
    #[cfg(unix)]
    pub fn ouvrir() -> Result<PortMidi, String> {
        use midir::os::unix::VirtualOutput;

        let sortie = midir::MidiOutput::new("dekkr-s4mk1").map_err(|e| e.to_string())?;
        let connexion = sortie
            .create_virtual(NOM_PORT)
            .map_err(|e| format!("port virtuel « {NOM_PORT} » refuse : {e}"))?;
        Ok(PortMidi { connexion })
    }

    /// ⚠️ CoreMIDI (macOS) et ALSA (Linux) savent creer un port virtuel ;
    /// l'API MIDI de Windows, non — il y faudra un pilote tiers (loopMIDI).
    /// Windows est explicitement le dernier de la file (SPEC-S4-001), mais le
    /// projet doit rester COMPILABLE ailleurs que sur le Mac.
    #[cfg(not(unix))]
    pub fn ouvrir() -> Result<PortMidi, String> {
        Err("port MIDI virtuel indisponible sur cette plateforme (Windows : pilote tiers requis)"
            .to_string())
    }

    #[cfg(unix)]
    pub fn emettre(&mut self, m: &MessageMidi) {
        let octets = m.octets();
        if let Err(e) = self.connexion.send(&octets) {
            // Ne pas mourir pour un message perdu : le boitier continue de
            // parler, et l'utilisateur doit pouvoir le voir.
            crate::dire!("⚠️ MIDI : {e}");
        }
    }

    #[cfg(not(unix))]
    pub fn emettre(&mut self, _m: &MessageMidi) {}
}
