# ADR-0005 : Report de la feature `material` (adaptateurs egui-material3) au backlog post-1.0

**Date** : 2026-06-15 | **Statut** : Accepté

## Contexte

La roadmap v0.8 prévoyait une feature `material` exposant des adaptateurs vers
[`egui-material3`](https://crates.io/crates/egui-material3) pour offrir des composants
Material Design 3 stylés par les tokens lumen-ui.

Vérification de compatibilité avant engagement (réflexe « ne jamais halluciner l'API egui » +
validation empirique d'une dépendance, comme fait pour `egui_taffy`) :

- `egui-material3` est en **v0.0.9** (pré-release 0.0.x, API instable, pas de garantie SemVer).
- Il dépend de **egui 0.33.3**, alors que lumen-ui épingle **egui 0.34.3** (ADR-0004). Les deux
  versions cohabiteraient dans l'arbre : `egui::Ui` 0.33 et 0.34 sont des types **distincts et
  incompatibles** — les widgets material3 n'accepteraient pas notre `&mut egui::Ui`.
- Il tire **~465 crates transitives** (`duckdb`, `tokio`, `resvg`/`usvg`/`tiny-skia`, `csv`…),
  en contradiction directe avec la philosophie légère de lumen-ui (egui + `egui_taffy` seulement).

## Décision

**Reporter la feature `material` au backlog post-1.0.** Elle ne sera reconsidérée que si
`egui-material3` (ou une alternative) publie une version stable ciblant la même version d'egui
que lumen-ui.

La valeur réelle du jalon v0.8 — **l'accessibilité WCAG 2.1 AA sur tous les composants** — est
livrée intégralement (module `lumen-ui-core::a11y`, audit automatisé en test sur les 4 thèmes
intégrés, focus visible, navigation clavier, cibles tactiles ≥ 44 px).

## Alternatives rejetées

- **Ajouter `egui-material3` 0.0.9 quand même** : casse l'arbre de dépendances (double egui),
  introduit une dette massive (465 crates) et une dépendance pré-release sur un chemin critique.
- **Vendoriser/forker egui-material3 vers egui 0.34** : coût de maintenance disproportionné pour
  une feature optionnelle, hors scope d'une v1.0.
- **Réimplémenter Material 3 from scratch** : c'est un design system complet à lui seul ; lumen-ui
  est volontairement neutre. Un thème « material-like » via `PaletteTheme` reste possible sans dep.

## Conséquences

- La feature `material` n'apparaît pas dans la façade en v1.0 ; le tableau de features est
  documenté sans elle. Pas de promesse non tenue dans la doc publique.
- Aucune dette technique introduite. La réintroduction future serait une feature additive
  (non-breaking).
- L'effort v0.8 est entièrement réinvesti dans l'accessibilité, qui est un critère de sortie
  mesurable et testé en CI.
