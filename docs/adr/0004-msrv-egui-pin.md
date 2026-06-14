# ADR-0004 : MSRV Rust 1.92 et pin strict d'egui dans une couche d'adaptation unique

**Date** : 2026-06-14 | **Statut** : Accepté

## Contexte

egui publie ~3 versions mineures par an, souvent avec des breaking changes. Le code généré
par IA lors de la conception ciblait des méthodes egui inexistantes (`Button::padding`,
`Button::shadow`) ou renommées (`Context::style_mut` → `global_style_mut`, `data` →
`data_mut` pour `get_persisted`). Sans confinement, chaque bump egui se propagerait dans tout
le codebase.

## Décision

- **MSRV** : Rust **1.92** (cible egui 0.34.3, juin 2026), déclaré via `rust-version` dans
  `[workspace.package]`. Pas de `rust-toolchain.toml` épinglant un patch exact (forcerait un
  téléchargement de toolchain inutile pour les contributeurs) — la MSRV est vérifiée en CI sur
  un job dédié, le développement local utilise la stable installée.
- **Pin egui** : version unique déclarée dans `[workspace.dependencies]` (`egui = "0.34.3"`).
- **Couche d'adaptation unique** : tout contact avec l'API egui est concentré dans
  `lumen-core` (et les impls minces de widgets). Un bump egui se traite à un seul endroit,
  avec vérification par compilation + clippy `-D warnings` avant commit.

## Alternatives rejetées

- **Suivre egui en `*` / dernière version** : casserait silencieusement à chaque bump amont.
- **Dépendance egui dispersée dans chaque crate sans couche d'adaptation** : multiplie les
  points de rupture.

## Conséquences

- ✅ Un breaking change egui se corrige à un seul endroit ; la CI casse tôt.
- ✅ Matrice de compatibilité egui maintenable (objectif v1.0).
- ⚠️ Bump egui = action délibérée avec ADR de suivi, pas automatique.
