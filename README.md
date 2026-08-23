# sonde-s4mk1

Sonde de diagnostic USB pour le **Traktor Kontrol S4 MK1** (`17cc:baff`).
Elle ne pilote rien : elle **mesure**, et surtout elle sait dire *pourquoi* elle échoue.

> ⚠️ **Dépôt séparé, volontairement hors de DekkR.** Ce code est un portage du
> protocole de `sound/usb/caiaq` (noyau Linux), donc un dérivé **GPL-2.0-or-later**.
> Le front DekkR ne doit jamais en dépendre ni l'héberger.

---

## Prérequis (macOS 12 Monterey, Intel)

1. Outils de compilation Apple :
   ```
   xcode-select --install
   ```
2. Rust :
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

Aucun `brew install libusb` n'est nécessaire : la dépendance `rusb` est compilée
avec la fonctionnalité `vendored`, qui construit libusb depuis ses sources.

## Lancer

S4 branché en USB. Le bloc secteur **n'est pas obligatoire** — le S4 MK1 fonctionne
sur l'alimentation du bus ; le bloc ne fait que rendre les LED plus lumineuses et le
casque un peu plus fort. Le mode diagnostic affiche ce que le boîtier **déclare**
consommer (`bMaxPower`) : au-delà de 500 mA, brancher le bloc devient pertinent.

```
cargo build --release
sudo ./target/release/sonde-s4mk1
```

`sudo` sur le **binaire déjà compilé**, jamais `sudo cargo`, qui recompilerait en root
et laisserait des fichiers inaccessibles dans `target/`.

### Deux modes

| Commande | Ce qu'elle fait |
|---|---|
| `sonde-s4mk1` | **surveillance** : attend le boîtier, l'arme, affiche les contrôles, encaisse les débranchements. Ne s'arrête jamais seule. |
| `sonde-s4mk1 --diagnostic` | dump du descripteur USB — interfaces, endpoints, alimentation — puis sort. |

Le mode surveillance **ne dépend pas de l'ordre de branchement** : on peut le lancer
avant le S4, débrancher, rebrancher — il se réarme seul. `Ctrl-C` rend l'interface
proprement au lieu de la laisser au noyau, ce qui compte pour le programme suivant
qui voudra la réserver.

---

## Ce que chaque étape établit

| Étape | Ce qu'elle prouve |
|---|---|
| **Inventaire complet** | liste **non filtrée** : si d'autres appareils apparaissent, libusb fonctionne et l'absence du S4 est un fait sur le S4. Si la liste est vide, la sonde ne dit rien du S4 — c'est libusb ou les droits. |
| **Descripteur / interfaces** | tranche la question restée ouverte sous Windows : **existe-t-il une interface HID (classe `0x03`) dans le descripteur ?** Windows ne pouvait pas répondre, macOS oui. Affiche aussi l'alimentation déclarée et le courant demandé. |
| **Endpoints** | vérifie que `0x01 OUT`, `0x81 IN` et `0x84 IN` existent, et que `0x84` est bien **bulk** (l'isochrone ne concerne que l'audio). |
| **GET_DEVICE_INFO** | le boîtier **dialogue-t-il** ? Renvoie la version de firmware et les compteurs (audio in/out, MIDI in/out) **lus dans le boîtier**. |
| **AUTO_MSG** | met le S4 en émission. **Sans elle il reste muet** — un silence avant cette commande ne signifierait pas « matériel mort ». |
| **Écoute ep 0x84** | la surface répond-elle ? Un bloc par groupe de contrôles, seuls les blocs **modifiés** sont affichés. |

## Lire la sortie

```
[   3.42s] bloc   2   00  02  07 ff [08][00] 00  00 ...
                     o1=2047 o2=2048 o3=0 ...
```

- `bloc N` — identifiant du groupe, `(buf[0] << 8) | buf[1]`.
- `[xx]` — octet **modifié** depuis le dernier affichage de ce bloc.
- `o1..o7` — valeurs 16 bits gros-boutiste aux offsets pairs. Les faders du S4
  sont sur **12 bits** : ils vont de `0` à `4095`.
- Bloc `0` — les 96 boutons, affichés par **numéro de bit** enfoncé.

Méthode : bouger **un seul contrôle à la fois** et noter quel bloc / quel offset
bouge. C'est ainsi qu'on construit la table de correspondance, sans la deviner.

## Licence — GPL-2.0 (sans « or later »)

Ce programme est une **œuvre dérivée** de `sound/usb/caiaq`, pilote du noyau Linux :

> Copyright (c) 2007 Daniel Mack &lt;daniel@caiaq.de&gt;, Karsten Wiese &lt;fzu@wemgehoertderstaat.de&gt;

Le texte complet est dans [COPYING](COPYING).

⚠️ **Pourquoi GPL-2.0 et pas GPL-2.0-or-later.** Les trois fichiers `.c` de `caiaq`
(`device.c`, `input.c`, `audio.c`) portent `SPDX-License-Identifier: GPL-2.0-or-later`,
mais **`device.h` porte `GPL-2.0` seul** — et c'est de lui que viennent les constantes
reprises ici : `EP1_CMD_*`, `EP1_BUFSIZE`, `EP4_BUFSIZE`, les identifiants USB et la
disposition de `caiaq_device_spec`. Le terme le plus strict s'impose à l'ensemble.

Le fait qu'une poignée de constantes et une disposition de structure soient ou non
protégeables est discutable — de l'information d'interface l'est rarement. La position
retenue ici est la prudente, pas la minimale.

**Dépendances** : `rusb` (MIT), `libusb` (LGPL-2.1, lié dynamiquement ou compilé via la
fonctionnalité `vendored`).

⛔ **Ce dépôt reste séparé de DekkR.** Le front est propriétaire ; aucun de ses fichiers
ne doit dépendre de ce code, et aucune ligne d'ici ne doit y être recopiée. Les échanges
passent exclusivement par un **port MIDI virtuel**.
