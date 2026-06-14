# AI_CONTEXT — lumen-ui (façade)

## Purpose
The crate consumers actually depend on. Re-exports the internal crates behind **feature flags**
and exposes a `prelude`. No logic of its own — pure surface. Adding a new internal crate means
wiring its feature here in the same PR.

## Constraints
- Feature matrix (see ROADMAP.md §A): `tokens` → `theme` → `widgets`; `serde`; `full`.
  Features for not-yet-existing crates (`motion`, `layout`, `patterns`, `icons`, `themes`,
  `material`) are added here when their crate is created — do not declare a `dep:` on a crate
  that does not exist yet.
- Re-exports are `#[cfg(feature = ...)]`-gated so a minimal `tokens`-only build still compiles.
- `examples/` (e.g. `minimal.rs`) live here and use eframe 0.34. eframe 0.34's `App` requires
  `fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame)` (not `update`).

## Forbidden
- No business logic, no widgets, no theme code here — it belongs in the internal crates.

## Common patterns
```rust
use lumen_ui::prelude::*;   // install, set_theme, DarkTheme, UiContext, Button, …
```

## Modules
- `lib.rs` — feature-gated re-exports + `prelude`.
- `examples/minimal.rs` — v0.1 live-theming validation example.
- `examples/gallery.rs` — all v0.2 widgets + live Dark/Light theme switch (`set_theme`).
