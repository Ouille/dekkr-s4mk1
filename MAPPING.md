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
| 4 | volume de boucle | **aucun geste ne l'a réveillé** | 🔴 voir plus bas |
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
| 16 – 19 | FX2 dry/wet, 1, 2, 3 | **2ᵉ panneau de FX balayé**, un des quatre potards | 🔶 groupe ✅, rôles ⬜ |
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
| 32 – 35 | FX1 dry/wet, 1, 2, 3 | **1ᵉʳ panneau de FX balayé**, un des quatre potards | 🔶 groupe ✅, rôles ⬜ |

**27 axes validés sur 36**, plus **8 groupés sans rôle attribué** (les FX), plus
**1 introuvable** (axe 4). Les 16 axes d'EQ sont tenus, ainsi que les deux faders
de pitch, le MIC, le CUE/MIX et les deux capteurs de proximité.

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

👉 **Confirmation à faire, et elle coûte un geste** : tourner **seul** le DRY/WET
du panneau de gauche. S'il répond **35**, tout ce qui précède est acquis ; s'il
répond 19, les panneaux sont inversés ; s'il répond 32, le balayage était de
droite à gauche. Tant que ce n'est pas fait, ces 8 lignes restent 🔶.

⚠️ **Ne pas corriger `ETIQUETTES` dans `decode.rs` avant cette confirmation** —
une étiquette fausse est moins coûteuse qu'une étiquette fausse *qu'on croit
validée*.

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

## 🔴 L'axe 4 ne répond à rien — et l'explication la plus probable est un TROU DE `caiaq`

Le PO a tourné le potard du Loop Recorder : **aucun événement**, pas même une ligne
au vidage initial (`derniers-axes.log`). L'axe 4 est le seul des 36 à n'avoir jamais
donné signe de vie.

⚠️ **« Le décodeur n'affiche rien » ne veut pas dire « le boîtier n'envoie rien ».**
Notre décodeur ne lit que les offsets que `snd_usb_caiaq_tks4_dispatch` lit, et
`caiaq` en **saute quatre** qui portent des valeurs vivantes :

| Bloc | Offsets non lus par `caiaq` |
| :-- | :--- |
| 2 | 5 — mesuré à **2053**, soit un cran central : ce n'est pas du vide |
| 3 | 1, 2, 5 |

`caiaq` mappe **exactement 36 axes**, et le boîtier en **déclare 36** : les valeurs
de ces quatre offsets sont donc des contrôles **hors du compte officiel**. Le potard
du Loop Recorder est un excellent candidat — c'est une fonction Traktor tardive, que
le pilote Linux de 2007 n'avait aucune raison d'exposer.

👉 **Le geste qui tranche : `--brut`, et ne toucher QUE ce potard.** Le mode brut
affiche les blocs octet par octet, encadrant ceux qui changent — il ne dépend
d'aucune table. Si un octet bouge, le contrôle est vivant et c'est notre table
qui est incomplète ; si rien ne bouge dans aucun bloc, alors seulement le contrôle
est muet côté matériel.

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
de la séance, a émis **cinq fois** entre 3032 et 3047. L'axe **11** de même. Ni
l'un ni l'autre n'a été manipulé : la cause n'est pas mécanique, c'est du **bruit
de conversion**.

**Amplitude mesurée sur les 26 axes restés au repos** (`fx-et-restants.log`) :
**14 à 27 pas sur 4095**, maximum sur les axes **10 et 11** — les capteurs de
proximité des jogs, capacitifs, donc les plus bruyants. Aucun axe au repos n'est
en dessous de 14.

🔑 **Le bruit se concentre dans le vidage initial, pas dans la durée.** Au premier
geste, le boîtier redonne l'état des 36 axes **trois fois en 60 ms**, et les trois
copies diffèrent du bruit. Ensuite, pendant les balayages, **aucun autre axe n'a
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
