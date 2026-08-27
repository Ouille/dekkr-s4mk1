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

### Les modes

| Commande | Ce qu'elle fait |
|---|---|
| `sonde-s4mk1` | **pont MIDI** : ouvre le port virtuel `DekkR S4 MK1`, attend le boîtier, l'arme, traduit les contrôles et les affiche. Ne s'arrête jamais seule. |
| `sonde-s4mk1 --sans-midi` | la même surveillance, **sans ouvrir de port MIDI** — c'est le mode des relevés. |
| `sonde-s4mk1 --brut` | blocs hexadécimaux de 16 octets, octets modifiés encadrés. **Jamais de MIDI** : ce mode n'assemble aucun événement. |
| `sonde-s4mk1 --diagnostic` | dump du descripteur USB — interfaces, endpoints, alimentation — puis sort. |
| `--journal <fichier>` | où écrire le journal (défaut : `sonde.log`) |
| `--sans-journal` | console seule |

⚠️ Le port virtuel est ouvert **avant** d'attendre le boîtier : Chrome énumère le MIDI
à l'ouverture de la page, et un port apparu après coup resterait invisible jusqu'au
rechargement. ⚠️ **macOS et Linux uniquement** — l'API MIDI de Windows ne sait pas créer
de port virtuel ; l'outil y retombe proprement en sonde seule.

### Ce qui part sur le fil

| Canal | Type | Contenu |
|---|---|---|
| 0 | Note On, note = **numéro de bit** | les boutons — voir `MAPPING.md` |
| 1 | Note On, **même note** | les mêmes boutons, **couche SHIFT** |
| 2 | CC, cc = numéro d'axe | les 34 axes ordinaires, 12 bits → 7 |
| 3 / 4 | Pitch bend 14 bits | jog gauche / droit |
| 5 / 6 | Pitch bend 14 bits | fader de tempo gauche / droit |
| 7 | Note On, deux notes par encodeur | rotation des 9 encodeurs : `index×2` vers le haut, `index×2+1` vers le bas |

Un **relâchement n'émet rien** : `MidiEngine.ts` ignore les Note Off. Et cinq bits sont
**muets par construction** — 20 et 23 (fermés au repos, sans contrôle physique connu),
85, 86 et 87 (interrupteurs du panneau arrière). Sans cette exclusion, chaque connexion
enverrait quatre Note On que personne n'a demandées.

### La couche SHIFT

Les deux boutons SHIFT (**bit 1** à gauche, **bit 57** à droite) n'émettent **jamais rien** :
ils font passer les boutons du canal 0 au canal 1, **à note identique**. La portée est
**globale** — n'importe lequel des deux bascule la façade entière, et la couche tient tant
qu'au moins un des deux est maintenu.

C'est le pont qui résout la couche, pas DekkR : `MidiEngine.ts` ignore les Note Off, donc
un modificateur *maintenu* ne peut pas s'y exprimer. Le pont envoie deux notes différentes,
le front n'a rien à savoir.

⚠️ La couche ne concerne que les **boutons**. Les axes, les jogs, les faders de tempo et la
rotation des encodeurs n'ont pas d'équivalent shifté.

### Le journal

Tout ce qui s'affiche part **aussi** dans un fichier — une séance de relevé produit
des milliers de lignes, impossibles à récupérer depuis le défilement du terminal.

⚠️ `sonde.log` est **écrasé à chaque lancement**. Pour conserver une séance, la nommer :

```
sudo ./target/release/sonde-s4mk1 --journal faders-volume.log
```

Le fichier est écrit sur le disque **au battement, toutes les 3 secondes**, pas à
chaque ligne : à ~600 blocs/s, un vidage par ligne ferait travailler le disque pour
rien. Un `kill -9` peut donc coûter les 3 dernières secondes ; un `Ctrl-C` non, il
passe par l'arrêt propre.

En mode décodé, chaque geste produit une ligne lisible :

```
[  12.31s] axe  7 crossfader        818 / 4095
[  12.44s] jog G  position  157 / 1023  (horodatage 0xeeb8)
[  12.51s] bouton 20  enfonce
```

Les étiquettes (`crossfader`, `EQ A bas`…) viennent des **commentaires de `caiaq`** et
ne sont **pas validées physiquement** — c'est l'objet de la tâche 4. L'ordre des canaux
du S4 étant C-A-B-D de gauche à droite, les intuitions sont trompeuses : utiliser
`--brut` ou les numéros d'axe pour établir la correspondance réelle.

Un contrôle qui ne bouge pas n'émet rien, et l'état de repos (tout à zéro, boutons
relâchés) sert de référence : au démarrage, seuls les contrôles **déjà hors position
neutre** se signalent.

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
