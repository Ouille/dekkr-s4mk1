# Table de correspondance — Traktor Kontrol S4 MK1

Ce fichier ne contient que du **mesuré**. Les étiquettes de `caiaq` sont des
commentaires du pilote Linux, que personne n'avait confrontés au matériel : tant
qu'une ligne n'est pas marquée ✅, elle reste une hypothèse.

Méthode : bouger **un contrôle à la fois**, noter l'axe ou le bit qui réagit.
Relevés faits sur le boîtier du PO (firmware 2575), MacBook Pro 2015 / macOS 12.7.6.

---

## 🔑 L'ordre des voies est C — A — B — D

De gauche à droite sur la façade. **Confirmé deux fois indépendamment** le
2026-08-23 : par les quatre faders de volume, puis par les quatre potards LOW,
qui donnent la même permutation.

C'est le piège principal de cette table. Un mapping écrit sous l'hypothèse
« la voie la plus à gauche est la A » serait faux sur les quatre voies à la fois,
et l'erreur ne se verrait qu'au premier mix.

---

## Contrôles analogiques — 12 bits (0 – 4095)

| Axe | Étiquette `caiaq` | Contrôle physique | Statut |
| :-- | :--- | :--- | :--- |
| 0 | volume canal D | fader de volume, **4ᵉ** depuis la gauche | ✅ 2026-08-23 |
| 1 | volume canal B | fader de volume, **3ᵉ** | ✅ 2026-08-23 |
| 2 | volume canal A | fader de volume, **2ᵉ** | ✅ 2026-08-23 |
| 3 | volume canal C | fader de volume, **1ᵉʳ** (le plus à gauche) | ✅ 2026-08-23 |
| 4 | volume de boucle | — | ⬜ au repos : 0 |
| 5 | tempo gauche | — | ⬜ au repos : 2411 |
| 6 | tempo droit | — | ⬜ au repos : 2056 |
| 7 | crossfader | crossfader | ✅ 2026-08-23 |
| 8 | volume micro | — | ⬜ au repos : 394 |
| 9 | cue mix | — | ⬜ au repos : 0 |
| 10 | proximité jog G | capteur de proximité du jog gauche | ✅ 2026-08-23 |
| 11 | proximité jog D | — | ⬜ au repos : 3094 |
| 12 | EQ D filtre | — | ⬜ |
| 13 | EQ D bas | potard LOW, **4ᵉ** depuis la gauche | ✅ 2026-08-23 |
| 14 | EQ D medium | — | ⬜ |
| 15 | EQ D haut | — | ⬜ |
| 16 | FX2 dry/wet | — | ⬜ au repos : 0 |
| 17 – 19 | FX2 1 / 2 / 3 | — | ⬜ |
| 20 | EQ B filtre | — | ⬜ |
| 21 | EQ B bas | potard LOW, **3ᵉ** | ✅ 2026-08-23 |
| 22 | EQ B medium | — | ⬜ |
| 23 | EQ B haut | — | ⬜ |
| 24 | EQ A filtre | — | ⬜ |
| 25 | EQ A bas | potard LOW, **2ᵉ** | ✅ 2026-08-23 |
| 26 | EQ A medium | — | ⬜ |
| 27 | EQ A haut | — | ⬜ |
| 28 | EQ C filtre | — | ⬜ |
| 29 | EQ C bas | potard LOW, **1ᵉʳ** (le plus à gauche) | ✅ 2026-08-23 |
| 30 | EQ C medium | — | ⬜ |
| 31 | EQ C haut | — | ⬜ |
| 32 – 35 | FX1 dry/wet, 1, 2, 3 | — | ⬜ |

**10 axes validés sur 36.** Aucun désaccord avec `caiaq` à ce jour.

## Jogs — 10 bits (0 – 1023), circulaires

| Contrôle | Statut |
| :--- | :--- |
| jog gauche | ✅ 2026-08-23 — compte en montant, reboucle proprement à 1023 → 0 |
| jog droit | ⬜ |

## Boutons — bitmask du bloc 0

| Bit | Contrôle physique | Statut |
| :-- | :--- | :--- |
| 7 | **PLAY, deck gauche** | ✅ 2026-08-23 |
| 20, 23, 86, 87 | interrupteurs, **fermés au repos** — position, pas appui | 🔎 à identifier |

**1 bouton nommé sur 96.** `caiaq` ne nomme aucun bouton : cette colonne est
entièrement à construire.

---

## Observations à ne pas perdre

**Les bits 86 et 87 existent alors que le boîtier déclare 78 entrées numériques.**
Ils sont à 1 au repos. Le décodeur garde donc les 96 bits alloués par `caiaq` :
tronquer à 78 supprimerait des contrôles réels.

**Quatre axes reposent à 0** — 2 (volume A), 4 (volume de boucle), 9 (cue mix),
16 (FX2 dry/wet). Ils n'apparaissent pas dans le vidage initial, ce qui est
normal : l'état de repos du décodeur est zéro, donc il n'y a rien à signaler.

**Dérive après relâchement d'un potard cranté.** Mesuré : l'axe 29 s'est stabilisé
à 2089 puis a émis 2075 deux secondes plus tard ; l'axe 21, de 2097 à 2086. Soit
10 à 15 pas sur 4095 — **0,3 %**, moins d'un demi-pas en CC 7 bits. Sans
conséquence pour le pont MIDI, mais à revoir si un jour on passe ces axes en
haute résolution.

**L'octet 2 du bloc 0 change sans qu'aucun bouton bouge** (`c0`, `40`, `80` selon
les relevés). Il est hors du masque des boutons, que `caiaq` lit à partir de
l'octet 4. Compteur ou drapeau d'état, non identifié, sans effet connu.
