# ADR-0002 : Recettes paramétrées par `(variant, state, ctx)` dès v0.1

**Date** : 2026-06-14 | **Statut** : Accepté

## Contexte

Le trait `Theme` est la couche la plus fondamentale de lumen-ui : tout widget et tout thème
en dépend. Toute modification de sa signature est un breaking change massif. La question :
faut-il introduire les états d'interaction (`Hovered`, `Active`, …) et la densité dans les
recettes maintenant, ou plus tard quand les widgets en auront besoin ?

## Décision

Les recettes sont paramétrées par `(variant, state, ctx)` **dès v0.1**, même si seuls
`Normal`/`Hovered`/`Active`/`Disabled` et `Density::Comfortable` sont pleinement exploités.
Idem pour `Density`, câblé dans `UiContext` dès le départ.

## Alternatives rejetées

- **Recettes par variant seul, états ajoutés en v0.2** : aurait forcé un changement de
  signature du trait `Theme` à la version suivante → re-câblage de tous les widgets et thèmes.

## Conséquences

- ✅ Ajouter un état ou une variante plus tard n'est **pas** un breaking change du trait.
- ✅ Le motion minimal de v0.2 et les springs de v0.5 se branchent sans toucher l'API publique
  des widgets (voir [ADR-0003](0003-minimal-motion-v02.md)).
- ⚠️ Un peu de code « en avance sur le besoin » en v0.1 — coût marginal, bénéfice structurel.
