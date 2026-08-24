// SPDX-License-Identifier: GPL-2.0-only
//
// Journal : tout ce qui part a la console part aussi dans un fichier.
//
// Les seances de relevé produisent des milliers de lignes ; les recuperer
// depuis le defilement du terminal est ingerable.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Mutex;

static JOURNAL: Mutex<Option<BufWriter<File>>> = Mutex::new(None);

/// Ouvre (et TRONQUE) le fichier de journal.
pub fn ouvrir(chemin: &str) -> std::io::Result<()> {
    let fichier = File::create(chemin)?;
    *JOURNAL.lock().unwrap() = Some(BufWriter::new(fichier));
    Ok(())
}

pub fn ecrire(ligne: &str) {
    if let Ok(mut verrou) = JOURNAL.lock() {
        if let Some(sortie) = verrou.as_mut() {
            let _ = writeln!(sortie, "{ligne}");
        }
    }
}

/// Force l'ecriture sur le disque.
///
/// 🔴 Appele au battement (toutes les 3 s) et a l'arret, PAS a chaque ligne :
/// le boitier produit ~600 blocs/s, un vidage par ligne ferait travailler le
/// disque en vain. Un arret brutal peut donc coûter les 3 dernieres secondes.
pub fn vider() {
    if let Ok(mut verrou) = JOURNAL.lock() {
        if let Some(sortie) = verrou.as_mut() {
            let _ = sortie.flush();
        }
    }
}

/// Comme `println!`, mais la ligne va aussi dans le journal.
#[macro_export]
macro_rules! dire {
    () => {{
        println!();
        $crate::journal::ecrire("");
    }};
    ($($arg:tt)*) => {{
        let ligne = format!($($arg)*);
        println!("{ligne}");
        $crate::journal::ecrire(&ligne);
    }};
}
