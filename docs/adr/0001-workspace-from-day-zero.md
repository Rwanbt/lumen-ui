# ADR-0001 : Workspace cargo dès le jalon 0

**Date** : 2026-06-14 | **Statut** : Accepté

## Contexte

Plusieurs IA consultées en review recommandaient un monorepo plat (un seul `src/`) à
découper en workspace multi-crates plus tard (v0.4), pour démarrer plus vite.

## Décision

Démarrer en **workspace cargo dès le jalon 0**, avec `lumen-core`, `lumen-widgets` et la
façade `lumen-ui`. Les autres crates (`lumen-motion`, `lumen-layout`, …) naissent quand leur
version de la roadmap arrive — pas de crates vides spéculatives, mais pas de « grand split »
ultérieur non plus.

## Alternatives rejetées

- **Monorepo plat à splitter en v0.4** : empile deux refactos risqués au même moment (le
  découpage en crates **et** l'intégration d'`egui_taffy`, la dépendance la plus instable).

## Conséquences

- ✅ Le compilateur impose la clarté des frontières (`pub` vs `pub(crate)`) dès le début —
  atout pour un design system.
- ✅ Aucun coût administratif tant qu'on ne publie pas (rien avant v0.9). Le `Cargo.toml`
  racine s'écrit une fois.
- ⚠️ Légère surcharge de structure dès le départ — jugée négligeable.
