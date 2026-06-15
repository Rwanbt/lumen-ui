# ADR-0007 : Nommage des crates publiées — namespace `lumen-ui-*`

**Date** : 2026-06-15 | **Statut** : Accepté

## Contexte

Préparation de la publication v1.0 sur crates.io. La préparation release (PR #28) a révélé que
le nom de package **`lumen-core` est déjà pris** sur crates.io (crate ML sans rapport, v0.5.0).
Impossible de publier le workspace tel quel.

Vérification de disponibilité au 2026-06-15 (`crates.io/api/v1/crates/<nom>`) :

| Nom | État |
|-----|------|
| `lumen` | PRIS (v2.30, 26k dls, sans rapport) |
| `lumen-core` | **PRIS** (v0.5, crate ML) |
| `lumen-ui` (façade) | **LIBRE** |
| `lumen-ui-core`, `lumen-ui-widgets`, … | **LIBRES** |
| `lumen-widgets`/`layout`/`motion`/`patterns`/`themes`/`icons` | tous LIBRES |

Seul `lumen-core` posait réellement problème : tous les autres `lumen-*` étaient libres.

## Décision

**Tous les crates internes adoptent le préfixe `lumen-ui-*`**, aligné sur le nom public de la
façade `lumen-ui` (qui reste inchangé) :

```
lumen-ui            façade publique (ce que l'utilisateur installe et importe)
lumen-ui-core       (était lumen-core)
lumen-ui-widgets    lumen-ui-layout    lumen-ui-motion
lumen-ui-patterns   lumen-ui-themes    lumen-ui-icons
lumen-ui-material   nom réservé (crate non créée — ADR-0005)
```

Le répertoire de chaque crate est aligné sur son nom de package (`crates/lumen-ui-core`, …) pour
éviter tout décalage dossier/package. L'outil binaire `lumen-theme-gen` n'est **pas** renommé
(ce n'est pas un crate de bibliothèque publié sous l'ombrelle, c'est un utilitaire de dev).

## Alternatives rejetées

- **Renommer uniquement `lumen-core`** (le seul en collision), garder les 6 satellites en `lumen-*` :
  rejeté car *amalgame*. Le namespace `lumen` est encombré (crate `lumen` v2.30 à 26k dls) ;
  publier `lumen-widgets`/`lumen-layout` en vrac laisse croire qu'ils appartiennent à ce projet
  tiers. Un préfixe `lumen-ui-*` uniforme rend l'appartenance au projet **visible** sur crates.io.
  Bénéfice « churn minimal » illusoire : `lumen-ui-core` étant importé par presque tous les crates,
  son renommage touche déjà tout le workspace — namespacer le reste n'ajoute qu'une réécriture
  mécanique de tokens, sans état intermédiaire à moitié renommé.
- **Rebrand complet** (`lumina`, `lume`, …) : inutile, `lumen-ui` (le nom public, déjà en usage
  dans le repo/README/bannière) est libre. Aucun coût de rebrand à payer.
- **Règle « `lumen-*` sauf core qui dévie »** : deux conventions au lieu d'une, plus difficile à
  expliquer dans la doc. Une règle uniforme (« tout est `lumen-ui-*` ») est plus simple.

## Conséquences

- **Aucun impact pour l'utilisateur final** : il installe `lumen-ui` et écrit
  `use lumen_ui::prelude::*;`. Les sous-crates ne sont quasi jamais référencés directement ;
  les chemins `lumen_ui_core::` sont internes au workspace.
- Sur crates.io, tous les crates du projet se regroupent sous `lumen-ui-*` — découverte et
  cohérence de marque, zéro confusion avec le crate tiers `lumen`.
- Les imports internes passent de `lumen_core::` à `lumen_ui_core::` (réécriture mécanique).
- `lumen-core` (le nom) **n'est plus à réserver** ; il appartient à un autre projet, ce qui est
  désormais sans conséquence.
- La collision crates.io documentée dans `RELEASING.md` (PR #28) est résolue : le workspace est
  publiable sous des noms tous libres.
