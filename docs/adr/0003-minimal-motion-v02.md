# ADR-0003 : Motion minimal en v0.2, springs en v0.5 — bascule transparente

**Date** : 2026-06-14 | **Statut** : Accepté

## Contexte

Le motion avancé (springs, easings, timelines) est un chantier à risque et coût élevés. Mais
le repousser entièrement en v0.5 obligerait à re-câbler tous les widgets à ce moment-là pour
brancher l'animation.

## Décision

Intégrer un **motion minimal dès v0.2** : interpolation couleur/opacité via le
`animate_value_with_time` natif d'egui, exposée par `lumen_core::anim::lerp_color`, **sans
dépendance lourde**. En v0.5, `lumen-motion` (springs/easings/timelines) remplace
l'implémentation derrière la même signature de helper — **aucun changement d'API publique des
widgets**.

## Alternatives rejetées

- **Aucun motion avant v0.5** : re-câblage de tous les widgets en v0.5.
- **Springs complets dès v0.2** : risque et coût trop élevés pour le socle.

## Conséquences

- ✅ Transitions hover/focus fluides dès v0.2 sans dette d'API.
- ✅ v0.5 = changement d'implémentation interne, pas de breaking change.
- 🔗 Repose sur les recettes paramétrées par état ([ADR-0002](0002-recipes-parameterized-by-state.md))
  et sur la lecture d'état en frame N-1.
