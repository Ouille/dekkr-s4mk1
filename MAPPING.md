# Table de correspondance — Traktor Kontrol S4 MK1

Ce fichier ne contient que du **mesuré**. Les étiquettes de `caiaq` sont des
commentaires du pilote Linux, que personne n'avait confrontés au matériel : tant
qu'une ligne n'est pas marquée ✅, elle reste une hypothèse.

Méthode : bouger **un contrôle à la fois**, noter l'axe ou le bit qui réagit.
Relevés faits sur le boîtier du PO (firmware 2575), MacBook Pro 2015 / macOS 12.7.6.

---

## 🔑 L'ordre des voies est C — A — B — D

De gauche à droite sur la façade. **Confirmé cinq fois indépendamment** : par les
quatre faders de volume et les quatre potards LOW (2026-08-23), puis par les
rangées FILTER, MID et HI (2026-08-24). Les cinq rangées donnent la même
permutation.

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
| 12 | EQ D filtre | potard FILTER, **4ᵉ** | ✅ 2026-08-24 |
| 13 | EQ D bas | potard LOW, **4ᵉ** depuis la gauche | ✅ 2026-08-23 |
| 14 | EQ D medium | potard MID, **4ᵉ** | ✅ 2026-08-24 |
| 15 | EQ D haut | potard HI, **4ᵉ** | ✅ 2026-08-24 |
| 16 | FX2 dry/wet | — | ⬜ au repos : 0 |
| 17 | FX2 1 | — | ⬜ au repos : 2063 |
| 18 | FX2 2 | — | ⬜ au repos : 1965 |
| 19 | FX2 3 | — | ⬜ au repos : 2931 |
| 20 | EQ B filtre | potard FILTER, **3ᵉ** | ✅ 2026-08-24 |
| 21 | EQ B bas | potard LOW, **3ᵉ** | ✅ 2026-08-23 |
| 22 | EQ B medium | potard MID, **3ᵉ** | ✅ 2026-08-24 |
| 23 | EQ B haut | potard HI, **3ᵉ** | ✅ 2026-08-24 |
| 24 | EQ A filtre | potard FILTER, **2ᵉ** | ✅ 2026-08-24 |
| 25 | EQ A bas | potard LOW, **2ᵉ** | ✅ 2026-08-23 |
| 26 | EQ A medium | potard MID, **2ᵉ** | ✅ 2026-08-24 |
| 27 | EQ A haut | potard HI, **2ᵉ** | ✅ 2026-08-24 |
| 28 | EQ C filtre | potard FILTER, **1ᵉʳ** (le plus à gauche) | ✅ 2026-08-24 |
| 29 | EQ C bas | potard LOW, **1ᵉʳ** | ✅ 2026-08-23 |
| 30 | EQ C medium | potard MID, **1ᵉʳ** | ✅ 2026-08-24 |
| 31 | EQ C haut | potard HI, **1ᵉʳ** | ✅ 2026-08-24 |
| 32 | FX1 dry/wet | — | ⬜ au repos : 600 |
| 33 | FX1 1 | — | ⬜ au repos : 1248 |
| 34 | FX1 2 | — | ⬜ au repos : 3047 |
| 35 | FX1 3 | — | ⬜ au repos : 1345 |

**22 axes validés sur 36.** Aucun désaccord avec `caiaq` à ce jour. Les 16 axes
d'EQ sont tous tenus ; les 14 restants ne dépendent d'aucune hypothèse d'ordre.

### Comment les 12 potards d'EQ ont été départagés

Relevé `releves/eq.log` (2026-08-24), trois balayages de gauche à droite :
FILTER, puis MID, puis HI. Dépouillement par étendue de valeurs :

> **Exactement 12 axes** parcourent plus de 4000 pas sur 4095 — et ce sont
> exactement les 12 prédits. Les quatre axes LOW (13, 21, 25, 29) n'émettent
> qu'une ligne, celle du vidage initial : ils n'ont pas été touchés.
> Aucun axe hors des 12 n'a été balayé.

C'est cette double exclusion qui fait la preuve : non seulement chaque potard
attendu a répondu, mais **aucun autre** n'a bougé. Un balayage qui aurait aussi
réveillé un axe voisin aurait signalé un couplage ou une étiquette décalée.

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

**🔴 Bruit de fond sur les axes au repos — ce n'est PAS une dérive de relâchement.**

Noté le 2026-08-23 : l'axe 29 s'était stabilisé à 2089 puis avait émis 2075 deux
secondes plus tard ; l'axe 21, de 2097 à 2086. J'en avais conclu qu'un potard
cranté se détend après qu'on l'a lâché. **C'était faux, et l'explication tenait
au seul fait que je venais de les toucher.**

Le relevé du 2026-08-24 tranche : l'axe **34** (FX1 2), que personne n'a approché
de la séance, a émis **cinq fois** entre 3032 et 3047 — 15 pas d'amplitude. L'axe
**11** (proximité jog D) de même, 11 pas. Ni l'un ni l'autre n'a été manipulé. La
cause n'est donc pas mécanique : c'est du **bruit de conversion**, présent en
permanence sur tous les axes, à hauteur de **10 à 15 pas sur 4095**.

Conséquence directe pour le pont MIDI (tâche 5) : un pont qui réémet à chaque
changement de valeur brute **enverra des CC sur des potards que personne ne
touche**. Un pas de CC 7 bits vaut 4095/127 ≈ **32 pas bruts**, donc le bruit
seul ne franchit pas un pas — sauf quand la position de repos tombe près d'une
frontière, où le CC oscillera indéfiniment entre deux valeurs voisines.

**Il faut une hystérésis**, pas seulement un « n'émettre que si le 7 bits
change » : exiger que la valeur brute dépasse la frontière d'au moins **16 pas**
avant de basculer. À vérifier au banc, l'amplitude mesurée ici ne vaut que pour
ce boîtier et cette séance.

**`GET_DEVICE_INFO` est resté sans réponse le 2026-08-24**, alors qu'il avait
répondu la veille (firmware 2575). Même boîtier, même binaire. C'est donc
**intermittent**, et non « ça marche » ou « ça ne marche pas ». Sans effet ici —
l'armement et la lecture se passent de cette réponse — mais la SPEC-S4-004
(audio) a besoin du champ `alignement` qu'elle seule fournit : il faudra soit
réessayer, soit figer la valeur lue une fois pour toutes.

**L'octet 2 du bloc 0 change sans qu'aucun bouton bouge** (`c0`, `40`, `80` selon
les relevés). Il est hors du masque des boutons, que `caiaq` lit à partir de
l'octet 4. Compteur ou drapeau d'état, non identifié, sans effet connu.
