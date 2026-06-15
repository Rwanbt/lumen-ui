# AI_CONTEXT — lumen-ui-core

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
use lumen_ui_core::{install, set_theme, DarkTheme, UiContext, UiThemeExt};

install(ctx, Arc::new(DarkTheme::new()), UiContext::default()); // once at startup
set_theme(ctx, Arc::new(DarkTheme::new()));                     // swap live
let recipe = ui.theme().button_recipe(variant, state, &ui.ui_ctx()); // inside a widget
```

## Modules
- `tokens.rs` — raw constants (Colors, Spacing, Radius, Typography, Elevation, Motion).
- `context.rs` — `Density` + `UiContext`.
- `recipe.rs` — `ButtonVariant`/`WidgetState`/`ButtonRecipe`, `TextRole`/`TextRecipe`,
  `CardRecipe`, `BadgeVariant`/`BadgeRecipe`, `ToggleRecipe`, `SliderRecipe`, `TextFieldRecipe`.
  `WidgetState` includes `Focused` (text input).
- `theme.rs` — `Theme` trait, `UiThemeExt`, `install`/`set_theme`.
- `builder.rs` — shared recipe-resolution logic (pure fns over `&Tokens` + an emphasis fn);
  themes delegate here so the rules live in one place (DRY).
- `dark.rs` / `light.rs` — `DarkTheme` / `LightTheme`: a palette + delegation to `builder`.
- `palette.rs` — `PaletteTheme` (public) + `ThemeMode`: a theme = `Tokens` palette + mode; the
  generic basis for `lumen-ui-themes`. New themes use this, never re-implement recipes.
- `anim.rs` — minimal motion (`lerp_color`); swaps to `lumen-ui-motion` in v0.5.
- `a11y.rs` — WCAG 2.1 contrast math (`relative_luminance`, `contrast_ratio`, `meets`/`meets_aa`,
  `ContrastLevel`). Pure fns; the theme layer audits its color pairs against these (v0.8).
