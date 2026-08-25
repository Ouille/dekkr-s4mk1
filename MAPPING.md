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
| 1 | **SHIFT**, deck gauche | ✅ 2026-08-25 |
| 3 | **SYNC**, deck gauche | ✅ 2026-08-25 |
| 5 | **CUE**, deck gauche | ✅ 2026-08-25 |
| 7 | **PLAY**, deck gauche | ✅ 2026-08-23, reconfirmé le 2026-08-25 |
| 57 | **SHIFT**, deck droit | ✅ 2026-08-25 |
| 59 | **SYNC**, deck droit | ✅ 2026-08-25 |
| 61 | **CUE**, deck droit | ✅ 2026-08-25 |
| 63 | **PLAY**, deck droit | ✅ 2026-08-25 |
| 20, 23, 86, 87 | interrupteurs, **fermés au repos** — position, pas appui | 🔎 à identifier |

**8 boutons nommés sur 96.** `caiaq` n'en nomme aucun : cette colonne est
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

### 🔎 Structure du masque — constaté, non expliqué

L'écart entre les deux rangées est de **56 bits, soit exactement 7 octets**, à
position identique dans l'octet : le masque commençant à `buf[4]`, la rangée
gauche occupe l'octet **4**, la droite l'octet **11**. Les quatre interrupteurs
fermés au repos se répartissent dans le même sens — **20 et 23** dans la moitié
basse, **86 et 87** dans la moitié haute.

👉 Lecture de travail : **bits bas = moitié gauche, bits hauts = moitié droite**.
Deux relevés la soutiennent, aucun ne la teste. Prochaine passe discriminante :
une **seconde rangée du deck gauche**, qui doit tomber dans les bits bas — et
qui dira si les rangées s'entrelacent dans le même octet (bits pairs 0, 2, 4, 6)
ou si chaque rangée a son octet (bits 8 à 15).

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

**Le boîtier ne parle qu'en cas de changement, et bloc par bloc.** Relevé du
2026-08-25 : rien pendant les 9 premières secondes (« plus aucun bloc ne
parvient »), puis **16 blocs en tout** pour 4 appuis — soit les 8 fronts du
bloc 0 plus quelques blocs de jog. **Aucun bloc d'axe n'est arrivé de toute la
séance.** Conséquence pour le pont MIDI : l'état initial d'un contrôle n'est
connu qu'après son premier mouvement, et un pont qui attend un état complet au
démarrage attendrait indéfiniment.

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
geste **analogique**, le boîtier redonne l'état des 36 axes **trois fois en 60 ms**,
et les trois copies diffèrent du bruit. *(Nuance apportée le 2026-08-25 : j'avais
écrit « au premier geste ». Le relevé des boutons montre 4 appuis sur 4 secondes
sans qu'un seul bloc d'axe n'arrive — la rafale est déclenchée par le bloc
concerné, pas par le réveil du flux.)* Ensuite, pendant les balayages, **aucun autre axe n'a
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
