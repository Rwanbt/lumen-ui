# AI_CONTEXT — lumen-core

## Purpose
Foundation layer of lumen-ui. Owns design **tokens**, the ambient **Density/UiContext**, the
**`Theme` trait** with state-parameterized **recipes**, the minimal-motion helper, and the
`install()` entry point that wires a theme into an `egui::Context`. Everything else in the
workspace depends on this crate; it depends only on `egui`.

## Constraints
- The `Theme` trait is the **frozen core**: recipes take `(variant, state, ctx)` so adding
  states/variants later is NOT a breaking change. Any signature change needs an ADR
  (`docs/adr/`). See ADR-0002.
- All egui API contact is concentrated here (adaptation layer, ADR-0004). egui is pinned in
  the workspace; verify every egui signature by compilation before commit.
- The theme is stored as `Arc<dyn Theme>` in `egui::Context` persisted data under
  `Id("lumen_theme")`; UI context under `Id("lumen_ctx")`. Read via `UiThemeExt` (lock-free).
- `get_persisted` requires `ctx.data_mut(...)` (not `data`). `apply_to_ctx` uses
  `ctx.global_style_mut(...)` (renamed in egui 0.34).
- `install()` sets `max_passes = 2` so widgets can read frame-N-1 interaction state.

## Forbidden
- Never hard-code a color/spacing in a widget — go through a recipe.
- `#![forbid(unsafe_code)]` — no `unsafe` in this crate.
- No global mutable state / singletons; the theme lives in egui's data store, owner-tracked.

## Common patterns
```rust
use std::sync::Arc;
use lumen_core::{install, set_theme, DarkTheme, UiContext, UiThemeExt};

install(ctx, Arc::new(DarkTheme::new()), UiContext::default()); // once at startup
set_theme(ctx, Arc::new(DarkTheme::new()));                     // swap live
let recipe = ui.theme().button_recipe(variant, state, &ui.ui_ctx()); // inside a widget
```

## Modules
- `tokens.rs` — raw constants (Colors, Spacing, Radius, Typography, Elevation, Motion).
- `context.rs` — `Density` + `UiContext`.
- `recipe.rs` — `ButtonVariant`/`WidgetState`/`ButtonRecipe`, `TextRole`/`TextRecipe`,
  `CardRecipe`, `BadgeVariant`/`BadgeRecipe`, `ToggleRecipe`.
- `theme.rs` — `Theme` trait, `UiThemeExt`, `install`/`set_theme`.
- `dark.rs` — `DarkTheme` bootstrap theme (moves to `lumen-themes` in v0.7).
- `anim.rs` — minimal motion (`lerp_color`); swaps to `lumen-motion` in v0.5.
