# ADR-0011 : Gestion des dates pour DatePicker / TimePicker / Calendar

**Date** : 2026-06-17 | **Statut** : **Accepté — Option A (maison, zéro dépendance)** | Auteur : session v1.4

> **Décision actée** : implémenter une arithmétique calendaire **maison** dans `lumen-ui-core`
> (module `date`), exposant des types valeurs `Date` / `Time` et les helpers civils nécessaires
> (`is_leap_year`, `days_in_month`, `day_of_week`). **Aucune dépendance dates** (`chrono`/`time`).

## Contexte

La v1.4 (inputs) et la v1.3 (Calendar) requièrent trois widgets liés par une même brique : la
représentation et l'arithmétique d'une date civile. egui **n'expose aucun calendrier natif**. Avant
d'écrire DatePicker / TimePicker / Calendar, il faut trancher : **dépend-on d'une crate de dates
(`chrono` ou `time`), ou code-t-on l'arithmétique calendaire à la main ?**

### Besoins réels (minimaux)
- Un type valeur `Date { year, month, day }` et `Time { hour, minute }`, bornés/validés.
- Pour rendre une grille de mois : **nombre de jours dans un mois** (avec années bissextiles) et
  **jour de semaine du 1er du mois** (décalage de la grille).
- Navigation mois précédent / suivant (avec passage d'année).
- **Pas besoin** de fuseaux horaires, parsing de formats localisés, durées, calendriers non
  grégoriens, ni instants — ce sont précisément les fonctionnalités lourdes de `chrono`/`time`.

### État des dépendances
- Le workspace n'a **aucune** dépendance dates. La philosophie du projet ([icons/lib.rs], ADR-0008)
  est « **no runtime dep** quand c'est raisonnable » : le pipeline d'icônes a refusé `resvg` runtime
  au profit d'un codegen maison pour la même raison.
- [ADR-0004] pin `egui = 0.34.3` strictement ; chaque dépendance ajoute une surface de maintenance.

## Décision — Option A : arithmétique maison dans le core (RECOMMANDÉE, retenue)

- Module `lumen_ui_core::date` (pur, `#![forbid(unsafe_code)]`, **sans dépendance**) :
  - `pub struct Date { year: i32, month: u32 /* 1..=12 */, day: u32 /* 1..=31 */ }` + `Time`.
  - `is_leap_year(year) -> bool` (règle grégorienne : ÷4 sauf ÷100 sauf ÷400).
  - `days_in_month(year, month) -> u32`.
  - `day_of_week(date) -> u32` (0 = lundi … 6 = dimanche) via l'algorithme de Sakamoto.
  - `Date::saturating_*` pour clamp, navigation mois ±1.
- Logique **pure et déterministe** → testable sans hardware ni horloge (tests unitaires : bissextiles,
  jours/mois, jour de semaine de dates connues). Aligne avec `a11y` (math pure déjà dans le core).
- Les widgets (`Calendar`, `DatePicker`, `TimePicker`) consomment `lumen_ui_core::{Date, Time}` —
  respecte la direction des dépendances (UI → Core → Types : un type valeur partagé vit dans le core).

*Pourquoi recommandée* : le besoin est minuscule (~60 LOC d'arithmétique grégorienne triviale et
stable depuis 1582), entièrement couvrable par des tests. Ajouter `chrono` (≈ gros graphe, features
`std`/`clock`) ou `time` pour calculer « combien de jours en février » est disproportionné et élargit
la surface de maintenance/CVE. Choix **réversible** : si un besoin lourd émerge (fuseaux, parsing
i18n), on pourra adopter `time` derrière une feature sans casser l'API `Date` publique.

## Alternatives rejetées

**Option B — dépendre de `chrono`.** ✅ Arithmétique éprouvée, formats, fuseaux. ❌ Graphe de
dépendances lourd, surface CVE (déjà des avis RustSec historiques), features `std`/`clock` à cadrer
pour le wasm ; surdimensionné pour « jours dans un mois ».

**Option C — dépendre de `time`.** ✅ Plus léger que `chrono`, API soignée. ❌ Reste une dépendance
liée à une lib externe pour un besoin que 60 LOC testées couvrent ; à reconsidérer **seulement** si
le parsing/format localisé devient un objectif produit.

## Conséquences

- +0 dépendance. Module `date` pur, testé, dans le core (nouvelle responsabilité « types valeurs +
  math calendaire », cohérente avec `a11y`).
- `Date`/`Time` deviennent des types **publics** de `lumen-ui-core` re-exportés par la façade ; tout
  changement de leur forme est un changement d'API (SemVer) — gelés avec soin (champs simples).
- Pas de i18n des noms de mois/jours au départ : libellés anglais (`Mon`, `Jan`) — l'i18n des thèmes
  et libellés est un chantier distinct (backlog).
- Si un besoin de fuseaux/durées/parsing émerge : adopter `time` derrière une feature, en préservant
  `Date`/`Time` comme façade — décision réversible.

## Délai de review : tranché en séance (2026-06-17), implémentation immédiate.
