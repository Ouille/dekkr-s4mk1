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
| 4 | volume de boucle | potard du **Loop Recorder** — ⚠️ bloc 2 **offset 5**, pas 6 | ✅ 2026-08-24 |
| 5 | tempo gauche | fader de pitch, deck **gauche** | ✅ 2026-08-24 |
| 6 | tempo droit | fader de pitch, deck **droit** | ✅ 2026-08-24 |
| 7 | crossfader | crossfader | ✅ 2026-08-23 |
| 8 | volume micro | potard **MIC** | ✅ 2026-08-24 |
| 9 | cue mix | potard **CUE/MIX** du casque | ✅ 2026-08-24 |
| 10 | proximité jog G | capteur de proximité du jog gauche | ✅ 2026-08-23 |
| 11 | proximité jog D | capteur de proximité du jog **droit** — main posée, étendue 819 | ✅ 2026-08-24 |
| 12 | EQ D filtre | potard FILTER, **4ᵉ** | ✅ 2026-08-24 |
| 13 | EQ D bas | potard LOW, **4ᵉ** depuis la gauche | ✅ 2026-08-23 |
| 14 | EQ D medium | potard MID, **4ᵉ** | ✅ 2026-08-24 |
| 15 | EQ D haut | potard HI, **4ᵉ** | ✅ 2026-08-24 |
| 16 | ~~FX2 dry/wet~~ **FX D 3** | panneau de FX **droit**, 4ᵉ potard | ✅ 2026-08-24 |
| 17 | ~~FX2 1~~ **FX D 2** | panneau de FX droit, 3ᵉ | ✅ 2026-08-24 |
| 18 | ~~FX2 2~~ **FX D 1** | panneau de FX droit, 2ᵉ | ✅ 2026-08-24 |
| 19 | ~~FX2 3~~ **FX D dry/wet** | panneau de FX droit, **1ᵉʳ** (le plus à gauche) | ✅ 2026-08-24 |
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
| 32 | ~~FX1 dry/wet~~ **FX G 3** | panneau de FX **gauche**, 4ᵉ potard | ✅ 2026-08-24 |
| 33 | ~~FX1 1~~ **FX G 2** | panneau de FX gauche, 3ᵉ | ✅ 2026-08-24 |
| 34 | ~~FX1 2~~ **FX G 1** | panneau de FX gauche, 2ᵉ | ✅ 2026-08-24 |
| 35 | ~~FX1 3~~ **FX G dry/wet** | panneau de FX gauche, **1ᵉʳ** — 🎯 mesure directe | ✅ 2026-08-24 |

# ✅ **36 axes sur 36.** Tous confrontés au boîtier, un contrôle à la fois.

**Deux désaccords avec `caiaq`**, tous deux mesurés : le **groupe des FX numéroté
à l'envers** (étiquettes), et l'**axe 4 lu au mauvais offset** (table de dispatch —
le plus grave des deux, car il rendait un contrôle invisible).

## 🔑 Deux règles d'orientation, et chacune a son domaine

| Famille de contrôles | Sens des numéros | Vérifié sur |
| :--- | :--- | :--- |
| **rangées horizontales du mixeur** | **décroissent** de gauche à droite | volumes 3/2/1/0 · EQ 29/25/21/13 · FX 35/34/33/32 |
| **paires gauche/droite des platines** | **croissent** de gauche à droite | jogs 10/11 · faders de pitch 5/6 |

🔴 **Ne pas transposer l'une à l'autre.** La règle de décroissance appliquée aux
faders de pitch aurait prédit `6 = gauche`, contre `caiaq` — et c'est `caiaq` qui
a raison : le balayage du 2026-08-24 donne l'axe 5 de 7,6 s à 12 s (fader gauche,
balayé en premier) puis l'axe 6 de 15,4 s à 23,7 s. *Une règle vérifiée trois fois
inspire assez confiance pour être dégainée hors de son domaine.*

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

### 🔶 Les 8 potards de FX : groupés, pas encore attribués

Relevé `releves/fx-et-restants.log` (2026-08-24). Huit balayages nets, ~4 s chacun,
sans chevauchement, dans cet ordre chronologique :

```
35 → 34 → 33 → 32     puis     19 → 18 → 17 → 16
```

**Ce que ça établit** : les deux panneaux de FX correspondent bien aux deux groupes
`caiaq` **{32, 33, 34, 35}** et **{16, 17, 18, 19}** — aucun axe ne traverse d'un
groupe à l'autre. La partition FX1 / FX2 est réelle.

**Le PO a levé l'ambiguïté (2026-08-24) : le DRY/WET est le potard le plus à
gauche du panneau de FX.** Le balayage ayant été fait de gauche à droite, le
DRY/WET est donc le **premier** axe sorti de chaque groupe — soit **35** et
**19**, et non 32 et 16.

🔴 **Ce sont les étiquettes de `caiaq` qui sont en défaut sur ce groupe**, pas
le relevé. `decode.rs` annonce `32 = FX1 dry/wet … 35 = FX1 3` ; l'ordre réel est
l'inverse. Attribution provisoire :

| Axe | Rôle réel (provisoire) | | Axe | Rôle réel (provisoire) |
| :-- | :--- | --- | :-- | :--- |
| 35 | FX gauche — **dry/wet** | | 19 | FX droit — **dry/wet** |
| 34 | FX gauche — 1 | | 18 | FX droit — 1 |
| 33 | FX gauche — 2 | | 17 | FX droit — 2 |
| 32 | FX gauche — 3 | | 16 | FX droit — 3 |

## 🔑 Les numéros d'axe DÉCROISSENT de gauche à droite

C'est ce qui rend l'attribution ci-dessus plus qu'une supposition : la même
régularité s'observe maintenant sur **trois familles de contrôles indépendantes**.

| Famille | De gauche à droite |
| :--- | :--- |
| faders de volume | 3, 2, 1, 0 |
| potards d'EQ (chaque rangée) | 29, 25, 21, 13 |
| potards d'un panneau de FX | 35, 34, 33, 32 |

Le boîtier numérote ses entrées analogiques **de la droite vers la gauche**, par
groupe. C'est aussi ce qui désigne le groupe {32-35} comme le panneau de
**gauche** (numéros hauts) et {16-19} comme celui de droite.

### ✅ Confirmé le 2026-08-24, en mode brut

Le DRY/WET du panneau de gauche tourné **seul**, sur un relevé où rien d'autre n'a
bougé (`releves/brut-boucle.log`) : **un seul offset de tout le boîtier a réagi**,
le **bloc 7 offset 5** — soit l'axe **35**. Les 41 autres offsets, les 96 bits de
boutons et les 11 encodeurs sont restés au niveau du bruit.

C'est la mesure la plus propre du dossier : une seule variable, une seule réponse.
`ETIQUETTES` est corrigé dans `decode.rs` — **seul désaccord avec `caiaq` sur les
35 axes vérifiés**.

## Jogs — 10 bits (0 – 1023), circulaires

| Contrôle | Statut |
| :--- | :--- |
| jog gauche | ✅ 2026-08-23 — compte en montant, reboucle proprement à 1023 → 0 |
| jog droit | ⬜ |

## Boutons — bitmask du bloc 0

| Bit | Contrôle physique | Statut |
| :-- | :--- | :--- |
| 0 | hot cue **1**, deck gauche | ✅ 2026-08-25 |
| 1 | **SHIFT**, deck gauche | ✅ 2026-08-25 |
| 2 | hot cue **2**, deck gauche | ✅ 2026-08-25 |
| 3 | **SYNC**, deck gauche | ✅ 2026-08-25 |
| 4 | hot cue **3**, deck gauche | ✅ 2026-08-25 |
| 5 | **CUE**, deck gauche | ✅ 2026-08-25 |
| 6 | hot cue **4**, deck gauche | ✅ 2026-08-25 |
| 7 | **PLAY**, deck gauche | ✅ 2026-08-23, reconfirmé le 2026-08-25 |
| 8 | bascule de deck **A / C**, gauche | ✅ 2026-08-25 |
| 9 | sample **1**, deck gauche | ✅ 2026-08-25 |
| 10 | **LOOP IN**, deck gauche | ✅ 2026-08-25 |
| 11 | sample **2**, deck gauche | ✅ 2026-08-25 |
| 12 | **LOOP OUT**, deck gauche | ✅ 2026-08-25 |
| 13 | sample **3**, deck gauche | ✅ 2026-08-25 |
| 14 | **LOAD**, deck gauche | ✅ 2026-08-25 |
| 15 | sample **4**, deck gauche | ✅ 2026-08-25 |
| 16 | **offset bas**, deck gauche | ✅ 2026-08-25 |
| 17 | **offset haut**, deck gauche | ✅ 2026-08-25 |
| 24 | **SIZE** (Loop Recorder) | ✅ 2026-08-25 |
| 25 | **REC** (Loop Recorder) | ✅ 2026-08-25 |
| 26 | **UNDO** (Loop Recorder) | ✅ 2026-08-25 |
| 27 | **PLAY** (Loop Recorder) | ✅ 2026-08-25 |
| 28 | **BROWSE** | ✅ 2026-08-25 |
| 29 | **SNAP** | ✅ 2026-08-25 |
| 30 | **MASTER** | ✅ 2026-08-25 |
| 31 | **QUANT** | ✅ 2026-08-25 |
| 32 | **CUE** casque, **1ʳᵉ** voie depuis la gauche (canal C) | ✅ 2026-08-25 |
| 33 | **CUE** casque, **2ᵉ** voie (canal A) | ✅ 2026-08-25 |
| 34 | **CUE** casque, **3ᵉ** voie (canal B) | ✅ 2026-08-25 |
| 35 | **CUE** casque, **4ᵉ** voie (canal D) | ✅ 2026-08-25 |
| 40 | **offset bas**, deck droit | ✅ 2026-08-25 |
| 41 | **offset haut**, deck droit | ✅ 2026-08-25 |
| 48 | bascule de deck **B / D**, droit | ✅ 2026-08-25 |
| 49 | sample **1**, deck droit | ✅ 2026-08-25 |
| 50 | **LOOP IN**, deck droit | ✅ 2026-08-25 |
| 51 | sample **2**, deck droit | ✅ 2026-08-25 |
| 52 | **LOOP OUT**, deck droit | ✅ 2026-08-25 |
| 53 | sample **3**, deck droit | ✅ 2026-08-25 |
| 54 | **LOAD**, deck droit | ✅ 2026-08-25 |
| 55 | sample **4**, deck droit | ✅ 2026-08-25 |
| 56 | hot cue **1**, deck droit | ✅ 2026-08-25 |
| 57 | **SHIFT**, deck droit | ✅ 2026-08-25 |
| 58 | hot cue **2**, deck droit | ✅ 2026-08-25 |
| 59 | **SYNC**, deck droit | ✅ 2026-08-25 |
| 60 | hot cue **3**, deck droit | ✅ 2026-08-25 |
| 61 | **CUE**, deck droit | ✅ 2026-08-25 |
| 62 | hot cue **4**, deck droit | ✅ 2026-08-25 |
| 63 | **PLAY**, deck droit | ✅ 2026-08-25 |
| 65 | FX **gauche** — bouton du 1ᵉʳ potard (le plus à gauche) | ✅ 2026-08-25 |
| 66 | FX **gauche** — bouton du 2ᵉ potard | ✅ 2026-08-25 |
| 67 | FX **gauche** — bouton du 3ᵉ potard | ✅ 2026-08-25 |
| 68 | FX **gauche** — bouton du 4ᵉ potard | ✅ 2026-08-25 |
| 69 | FX **gauche** — **MODE** | ✅ 2026-08-25 |
| 72, 73 | assignation FX de la **1ʳᵉ** voie (canal C) | ⚠️ voie sûre, **ordre de la paire à confirmer** |
| 74, 75 | assignation FX de la **2ᵉ** voie (canal A) | ⚠️ idem |
| 76, 77 | assignation FX de la **3ᵉ** voie (canal B) | ⚠️ idem |
| 78, 79 | assignation FX de la **4ᵉ** voie (canal D) | ⚠️ idem |
| 89 | FX **droit** — bouton du 1ᵉʳ potard (le plus à gauche) | ✅ 2026-08-25 |
| 90 | FX **droit** — bouton du 2ᵉ potard | ✅ 2026-08-25 |
| 91 | FX **droit** — bouton du 3ᵉ potard | ✅ 2026-08-25 |
| 92 | FX **droit** — bouton du 4ᵉ potard | ✅ 2026-08-25 |
| 93 | FX **droit** — **MODE** | ✅ 2026-08-25 |
| 20, 23, 86, 87 | interrupteurs, **fermés au repos** — position, pas appui | 🔎 à identifier |

**66 boutons nommés sur 96**, dont 8 (les assignations FX) dont la **voie** est
sûre mais dont l'ordre **à l'intérieur de la paire** attend une précision du PO :
les deux boutons FX d'un même gain sont-ils côte à côte ou l'un au-dessus de
l'autre, et lequel a été pressé en premier ? *La voie est acquise parce que les
quatre paires se suivent dans l'ordre du balayage ; le rang dans la paire ne se
déduit d'aucune mesure.*

⚠️ **Sur le deck droit, LOAD et la bascule de deck sont en miroir** de ceux du
deck gauche : LOAD est à gauche de la bascule. L'attribution ne repose donc pas
sur le seul ordre d'appui — **deux arguments indépendants convergent** : le PO a
appuyé 54 puis 48 en allant de gauche à droite sur cette rangée inversée, *et*
l'octet 10 réplique la structure de l'octet 5, où la bascule occupe le bit bas
et LOAD le bit haut. `caiaq` n'en nomme aucun : cette colonne est
entièrement à construire, il n'y a rien à porter.

### ✅ Rangée transport du deck gauche, 2026-08-25

`releves/boutons-transport-gauche.log`. Quatre appuis annoncés **avant** le geste,
de gauche à droite : SHIFT, SYNC, CUE, PLAY. Le journal donne quatre bits neufs
dans cet ordre — **1, 3, 5, 7** — et **aucun autre bit ni axe n'a bougé**.

🔑 **La rangée contenait le seul bouton déjà nommé, et il est tombé juste** :
PLAY = bit 7, mesuré indépendamment le 2026-08-23. Une passe dont on connaît
déjà une réponse vaut mieux qu'une passe entièrement inconnue — le témoin valide
la méthode avant qu'on lui fasse confiance sur les 92 restants.

⚠️ **Le bit 1 arrive dans le même paquet que le vidage initial** et aurait pu
passer pour un cinquième interrupteur fermé au repos. Il est **relâché** 0,3 s
plus tard, et `decode.rs` fige la liste des fermés au repos à exactement
`[20, 23, 86, 87]` : c'est bien un appui, celui qui a réveillé le flux.

### ✅ Rangée transport du deck droit, 2026-08-25

`releves/boutons-transport-droit.log`. Même rangée, **même ordre** sur la façade
(pas de miroir, confirmé par le PO avant le geste) : SHIFT, SYNC, CUE, PLAY.
Bits neufs, dans cet ordre : **57, 59, 61, 63**. Rien d'autre n'a bougé. Même
détail qu'à gauche — le premier bit arrive dans le paquet du vidage initial,
c'est l'appui qui réveille le flux, et il est relâché 0,35 s plus tard.

### ⛔ Hypothèse d'entrelacement gauche/droite — RÉFUTÉE par la mesure

J'avais prédit **{0, 2, 4, 6}** pour le deck droit, sur la seule foi de la parité
des quatre bits de gauche. **C'est faux** : les deux rangées sont impaires et
espacées de 2, `1,3,5,7` d'un côté, `57,59,61,63` de l'autre. Les bits pairs
{0, 2, 4, 6} ne sont pas le deck droit et **restent à trouver**.

🔑 **Et la règle des platines n'était pas menacée** : gauche = bits **bas**,
droite = bits **hauts**, donc les paires **croissent**, comme les jogs (10/11) et
les faders de pitch (5/6). La contradiction n'existait que dans mon hypothèse ;
la règle gagne une confirmation de plus au lieu d'une exception.

⚠️ *Coût évité* : inscrite sans test, cette hypothèse aurait posé les quatre
boutons du deck droit sur `0, 2, 4, 6` — faux sur les quatre, et invisible
jusqu'au premier mix. **La parité d'une suite de quatre nombres ne dit rien de
la structure ; deux relevés le disent.**

### ✅ Les 19 boutons restants du deck gauche, 2026-08-25

`releves/boutons-deck-gauche.log`. Six rangées annoncées **avant** le geste avec
leur compte (5 / 2 / 2 / 2 / 4 / 4), une **pause de 5 s entre chaque** : le
journal imprime alors « plus aucun bloc ne parvient », ce qui **découpe les
rangées sans ambiguïté**. Résultat : **19 bits neufs, 19 attendus**, répartis
exactement selon l'annonce, et **aucun autre bit n'a bougé**.

🔑 *Le séparateur ne coûte rien et il est déjà là.* Une passe de 19 boutons est
aussi sûre qu'une passe de 4 dès lors qu'on peut situer chaque rangée dans le
journal — sans les silences, une erreur au 3ᵉ bouton décalerait les 16 suivants
sans laisser de trace.

### ✅ Le corps du deck est un damier — bits 0 à 17, sans trou

| Octet | Bits pairs | Bits impairs |
| :--- | :--- | :--- |
| 4 | hot cues 1-4 (0, 2, 4, 6) | transport SHIFT/SYNC/CUE/PLAY (1, 3, 5, 7) |
| 5 | A/C, LOOP IN, LOOP OUT, LOAD (8, 10, 12, 14) | samples 1-4 (9, 11, 13, 15) |
| 6 | offset bas (16) | offset haut (17) |

C'est l'explication des `1, 3, 5, 7` du transport : la rangée voisine occupait
les bits pairs depuis le début. **Deux rangées se partagent un octet en damier**,
elles ne s'empilent pas octet par octet.

### ⛔ Deux prédictions du 2026-08-25, toutes deux prises en défaut

**① « La bascule A/C est un interrupteur à accrochage »** — faux. Le bit 8 est
**relâché** 0,15 s après l'appui : bouton ordinaire. **Les quatre bits fermés au
repos (20, 23, 86, 87) restent entiers et non identifiés.**

**② « Les 19 bits tomberont dans les bits bas »** — faux : la rangée FX sort à
**65-69**, au-dessus du transport droit. 🔑 *L'erreur vient du découpage, pas du
boîtier* : j'ai rangé le panneau FX avec le deck **parce que le PO me l'a décrit
en haut du deck**, alors que le boîtier le classe avec le **mixeur**. Or la règle
du mixeur était déjà mesurée le 24/08 sur les axes — **décroissante de gauche à
droite** (FX gauche = 32-35, FX droit = 16-19). Les deux règles d'orientation du
24/08 valent donc aussi pour les boutons : **corps de platine croissant, mixeur
décroissant**.
👉 Conséquence testable : les 5 boutons du panneau FX **droit** doivent sortir à
des bits **strictement inférieurs à 65**. — ⛔ **RÉFUTÉ le jour même : 89-93.**
Voir « la règle décroissante du mixeur ne vaut que pour les axes », plus bas.

🔴 **L'écart de +56 ne se généralise pas.** Vérifié sur la seule rangée de
transport (1,3,5,7 → 57,59,61,63). Appliqué au deck entier, il placerait la
bascule A/C droite à 64 et LOOP IN droit à 66 — **or 65 à 69 appartiennent au FX
gauche**. *Une régularité vérifiée sur une rangée ne dit rien de la rangée
voisine.*

### ✅ Les 19 boutons du deck droit, 2026-08-25

`releves/boutons-deck-droit.log`. Même protocole, mêmes six rangées, mêmes
pauses : **19 bits neufs, 19 attendus**, aux six bonnes coupures, rien d'autre.

🎯 **Prédiction confirmée au bit près : hot cues droits = 56, 58, 60, 62.**
Annoncée avant le geste, à partir du seul damier de l'octet 4. **Le damier est
une loi du boîtier**, pas une particularité du deck gauche.

⛔ **Prédiction réfutée : « le FX droit sortira sous 65 ».** Il sort à **89-93**,
donc *au-dessus* du FX gauche (65-69). Les boutons sont **croissants de gauche à
droite partout** — corps de platine, transport, panneaux FX.

🔴 **La règle décroissante du mixeur ne vaut que pour les AXES.** Même panneau FX
physique : ses potards sont numérotés décroissants de gauche à droite (32-35 à
gauche, 16-19 à droite), ses boutons croissants (65-69 à gauche, 89-93 à droite).
*Une règle d'orientation appartient à la **famille de contrôle**, pas à la zone de
la façade.* Troisième transposition abusive d'une orientation dans ce dossier —
les deux premières le 2026-08-24.

### 🔑 Le masque est un miroir — et il désigne où chercher le mixeur

| Rangée | Deck gauche | Deck droit | Somme |
| :--- | :--- | :--- | :--- |
| hot cues + transport | octet **4** | octet **11** | 15 |
| bascule/IN/OUT/LOAD + samples | octet **5** | octet **10** | 15 |
| offsets | octet **6** | octet **9** | 15 |
| panneau FX | octet **12** | octet **15** | — |

Les deux decks sont **symétriques autour du milieu du masque**, et les positions
*à l'intérieur* de l'octet sont **identiques** des deux côtés (hot cue 1 = bit 0
de son octet, à gauche comme à droite). Ce n'est donc pas un décalage constant :
l'écart gauche→droite vaut +56 sur les hot cues, +40 sur les samples, +24 sur les
offsets. **C'est l'ordre des octets qui s'inverse, pas les bits.**

👉 **Prédiction pour la passe du mixeur** : il reste entre les deux miroirs les
**octets 7 et 8, soit les bits 24 à 39**. C'est là que les boutons du mixeur
doivent tomber. Les bits 18, 19, 21, 22 (fin de l'octet 6) et 42-47 (fin de
l'octet 9) restent également libres.

### ✅ Les 20 boutons du mixeur, 2026-08-25

`releves/boutons-mixeur.log`. Six rangées annoncées (8 / 3 / 2 / 2 / 1 / 4),
**20 bits neufs, 20 attendus**, rien d'autre.

🎯 **Prédiction confirmée** : les 8 boutons du centre (SNAP, MASTER, QUANT, SIZE,
UNDO, REC, PLAY, BROWSE) tombent tous dans les bits **24-39** que le miroir
laissait libres — précisément **24-31**.

⛔ **Prédiction réfutée sur le débordement.** J'avais annoncé que les boutons
*par voie* (8 FX + 4 CUE) iraient dans les places libres autour des decks (18,
19, 21, 22 et 42-47). **Aucune n'a bougé** : les 4 CUE restent avec le centre
(**32-35**) et ce sont les **8 assignations FX** qui partent à **72-79**.

🔑 **L'octet 13 n'était pas vide, il attendait sa mesure.** Ce fichier écrivait
le matin même que l'octet symétrique 13 « ne porte rien de connu ». Il porte les
assignations FX. La zone FX est donc un bloc cohérent de quatre octets :
**panneau gauche (12), assignations (13), 86/87 + libre (14), panneau droit (15)**.

🔑 **Le damier n'est pas propre aux platines, c'est la grammaire du boîtier** :
SIZE/UNDO occupent les pairs **24, 26** et REC/PLAY les impairs **25, 27** — deux
rangées voisines qui se partagent un octet en alternance, comme les hot cues et
le transport.

### 🔢 Ce qui reste — et un décompte qui ne tombe pas juste

**66 boutons nommés + 4 interrupteurs fermés au repos = 70.** Le boîtier déclare
**78** entrées numériques (`caiaq` en alloue 96). **Il en manque donc 8.**

Candidats évidents : les **clics d'encodeurs**. Le boîtier en porte 9 — 4 GAIN,
1 BROWSE, et 2 par platine autour de LOOP IN/OUT. S'ils cliquent tous, on
dépasserait 78 d'une unité ; s'il en manque un, on tomberait juste. ⚠️ **Ne pas
bâtir sur ce compte** : le « 78 » vient de la déclaration du boîtier, que le
dossier a déjà prise en défaut (les bits 86 et 87 existent au-delà des 78).

Bits encore libres : 18, 19, 21, 22 · 36 à 39 · 42 à 47 · 64, 70, 71 · 80 à 85 ·
88, 94, 95.

### 🔎 Structure du masque — constaté, non expliqué

L'écart entre les deux rangées de transport est de **56 bits, soit exactement 7
octets**, à position identique dans l'octet : le masque commençant à `buf[4]`, la
rangée gauche occupe l'octet **4**, la droite l'octet **11**.

⛔ **La lecture de travail « bits bas = moitié gauche, bits hauts = moitié
droite » est FAUSSE**, et le relevé du deck gauche l'a montrée le jour même : le
panneau FX **gauche** occupe les bits **65-69**. Elle est remplacée par la
symétrie en miroir des octets, décrite plus haut — qui rend compte des deux decks
*et* des deux panneaux FX. Les deux questions qu'elle laissait ouvertes sont
tranchées : les rangées **s'entrelacent en damier dans un même octet**, et
l'écart gauche→droite **n'est pas constant**.

⚠️ Les quatre interrupteurs fermés au repos, eux, **ne suivent pas le miroir** :
20 et 23 tombent dans l'octet 6 (celui des offsets gauches), 86 et 87 dans
l'octet 14, dont l'octet symétrique 13 ne porte rien de connu. Aucun des quatre
n'a été identifié à ce jour.

---

## ✅ L'axe 4 était muet parce que `caiaq` le lit au mauvais endroit

Le potard du Loop Recorder ne produisait **aucun** événement, pas même au vidage
initial — seul des 36 dans ce cas. Ce n'était pas le boîtier qui se taisait.

**Mesure du 2026-08-24** (`releves/boucle-seule.log`, mode brut, potard tourné seul,
rien d'autre touché) :

| Bloc 2 | Valeurs observées | Lu par `caiaq` ? |
| :--- | :--- | :--- |
| **offset 5** | **9 → 4095**, course complète | ❌ non |
| offset 6 | **0, et rien d'autre** | ✅ oui — c'est là que `caiaq` met l'axe 4 |

Aucun autre offset, aucun bit de bouton, aucun encodeur n'a bougé. `AXES_PAR_BLOC`
lit désormais l'**offset 5** pour l'axe 4, et un test dédié rougit si quelqu'un
revient au mapping de `caiaq` — le fixture porte justement 2053 à l'offset 5 et 0
à l'offset 6.

🔑 **C'est le désaccord le plus grave des deux** : une étiquette fausse se corrige
à la lecture, un offset faux rend un contrôle **invisible**. Et il serait passé
inaperçu sans le mode brut, qui ne dépend d'aucune table.

### Ce qui reste des trous de `caiaq`

⚠️ **« Le décodeur n'affiche rien » ne veut pas dire « le boîtier n'envoie rien ».**
Notre décodeur ne lit que les offsets que lit `snd_usb_caiaq_tks4_dispatch`, et
`caiaq` en **saute six** : bloc 2 offset 5, bloc 3 offsets 1, 2, 5, bloc 7 offsets
6 et 7.

**Un seul de ces six porte une valeur vivante : le bloc 2 offset 5**, mesuré entre
**2037 et 2057** — un cran central, donc un vrai contrôle mis de côté par le pilote
Linux. Les cinq autres sont **exactement à zéro** dans tout le relevé brut.

🔴 *Correction.* J'avais écrit que **quatre** de ces trous portaient des valeurs
vivantes. C'était une généralisation de la seule mesure dont je disposais : un
relevé brut les montre tous à zéro sauf un. **Une mesure ne se propage pas à ses
voisins** — même famille d'erreur que la « dérive de relâchement ».

Il reste donc **cinq offsets non lus, tous à zéro en permanence** : bloc 3 offsets
1, 2 et 5, bloc 7 offsets 6 et 7, auxquels s'ajoute le bloc 2 offset 6 abandonné
par la correction ci-dessus. Aucun contrôle physique connu ne leur correspond, et
les 36 axes déclarés par le boîtier sont désormais tous attribués.

⚠️ Un emplacement à zéro reste indiscernable d'un contrôle au repos à zéro. Si un
contrôle non identifié apparaît un jour, ce sont les six premiers endroits à
regarder — en `--brut`.

## Observations à ne pas perdre

**🔴 Le démarrage du flux n'est pas déterministe — trois relevés, deux
comportements.** Les deux relevés de transport (2026-08-25) ne reçoivent **rien**
pendant 7 à 9 s après l'armement, puis **16 blocs en tout** pour 4 appuis : les 8
fronts du bloc 0 et quelques blocs de jog, **aucun bloc d'axe de toute la
séance**. Le relevé du deck gauche, même binaire et même boîtier, reçoit à
**2,80 s un état complet non sollicité** — 36 axes, jogs, encodeurs, interrupteurs
— **avant tout geste**.

Aucune des deux formes n'est « la » bonne, et la cause n'est pas identifiée
(état laissé par la séance précédente ?). ⚠️ **Conséquence pour le pont MIDI
(tâche 5) : ne présumer ni l'une ni l'autre.** Un pont qui attend un état complet
au démarrage attendrait indéfiniment dans le premier cas ; un pont qui suppose
n'avoir rien reçu recevrait 36 axes d'un coup dans le second.

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
de la séance, a émis **cinq fois** entre 3032 et 3047. L'axe **11** de même. Ni
l'un ni l'autre n'a été manipulé : la cause n'est pas mécanique, c'est du **bruit
de conversion**.

**Amplitude mesurée sur les 26 axes restés au repos** (`fx-et-restants.log`) :
**14 à 27 pas sur 4095**, maximum sur les axes **10 et 11** — les capteurs de
proximité des jogs, capacitifs, donc les plus bruyants. Aucun axe au repos n'est
en dessous de 14.

🔑 **Le bruit se concentre dans le vidage initial, pas dans la durée.** Au premier
geste, le boîtier redonne l'état des 36 axes **trois fois en 60 ms**, et les trois
copies diffèrent du bruit.

🔴 **Correction du 2026-08-25 — ce paragraphe a été nuancé à tort dans la journée.**
Après le relevé du transport (4 appuis, 4 secondes, **aucun bloc d'axe**), j'avais
écrit que la rafale était déclenchée par le premier geste *analogique*. Le relevé
du deck gauche, trois heures plus tard, la montre arrivée **à 2,80 s, complète,
sans qu'aucun geste n'ait eu lieu** — et **une seule fois**, pas trois. Trois
séances, deux comportements, cause non identifiée. *J'ai remplacé une
généralisation par une autre, à partir d'un seul relevé à chaque fois.* Ensuite, pendant les balayages, **aucun autre axe n'a
émis** : le boîtier applique son propre seuil quand il est déjà en émission. Le
problème est donc **borné** — une rafale au démarrage, plus quelques émissions
isolées, et non un bruit de fond continu.

Conséquence pour le pont MIDI (tâche 5) : un pont qui réémet à chaque changement
de valeur brute **enverra une centaine de CC sur des potards que personne ne
touche**, à chaque reprise d'émission. Un pas de CC 7 bits vaut 4095/127 ≈ **32
pas bruts**, à peine au-dessus des 27 mesurés — la marge est trop mince pour se
contenter d'un « n'émettre que si la valeur 7 bits change ».

**Il faut une hystérésis d'un pas de CC entier (32 pas bruts)** : n'émettre que
lorsque la valeur brute s'est écartée d'au moins 32 de **la dernière valeur
émise**, pas de la frontière la plus proche. Coût pour le DJ : 0,8 % de course
morte, imperceptible. ⚠️ Les 27 pas valent pour ce boîtier et ces deux séances —
si un autre exemplaire mesure davantage, c'est le seuil qu'il faut relever, pas
la règle.

**`GET_DEVICE_INFO` est resté sans réponse le 2026-08-24**, alors qu'il avait
répondu la veille (firmware 2575). Même boîtier, même binaire. C'est donc
**intermittent**, et non « ça marche » ou « ça ne marche pas ». Sans effet ici —
l'armement et la lecture se passent de cette réponse — mais la SPEC-S4-004
(audio) a besoin du champ `alignement` qu'elle seule fournit : il faudra soit
réessayer, soit figer la valeur lue une fois pour toutes.

**L'octet 2 du bloc 0 change sans qu'aucun bouton bouge** (`c0`, `40`, `80` selon
les relevés). Il est hors du masque des boutons, que `caiaq` lit à partir de
l'octet 4. Compteur ou drapeau d'état, non identifié, sans effet connu.
