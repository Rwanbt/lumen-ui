# Glossary — lumen-ui

Operational definitions of the vocabulary used in the code. A contributor without a design-
system background can introduce subtle bugs by misreading these terms.

| Term | Definition (in this project) | Implemented in |
|------|------------------------------|----------------|
| **Token** | A raw, named visual constant (a color, a spacing step, a corner radius). Tokens carry no logic and no per-state variation. The single source of visual truth. | [`lumen-core/src/tokens.rs`](../crates/lumen-core/src/tokens.rs) |
| **Recipe** | The fully *resolved*, ready-to-draw style for **one widget in one state** — e.g. a primary button while hovered. Produced by a theme from tokens + `(variant, state, density)`. Widgets consume recipes, never tokens. | [`recipe.rs`](../crates/lumen-core/src/recipe.rs) |
| **Theme** | An object that owns a `Tokens` set and knows how to turn `(variant, state, ctx)` into a recipe, plus how to map tokens onto egui's global `Style`/`Visuals`. Lives behind `Arc<dyn Theme>` in egui's data store. | [`theme.rs`](../crates/lumen-core/src/theme.rs), [`dark.rs`](../crates/lumen-core/src/dark.rs) |
| **Variant** | A semantic flavor of a widget independent of interaction — e.g. a button's `Primary` / `Secondary` / `Ghost` / `Danger`. | [`recipe.rs`](../crates/lumen-core/src/recipe.rs) |
| **State** | The interaction state of a widget at a moment: `Normal` / `Hovered` / `Active` / `Disabled`. Read from the **previous** frame because egui only knows hover after allocation. | [`recipe.rs`](../crates/lumen-core/src/recipe.rs) |
| **Density** | Ambient display compactness: `Compact` / `Comfortable` / `Touch`. Scales paddings and hit-targets. Wired in from v0.1 to avoid a future breaking change. | [`context.rs`](../crates/lumen-core/src/context.rs) |
| **UiContext** | The ambient, theme-independent parameters passed to every recipe (currently just `density`; extensible to high-contrast, etc.). | [`context.rs`](../crates/lumen-core/src/context.rs) |
| **Install** | Wiring a theme + UI context into an `egui::Context` (once at startup, or again to swap live). Also sets `max_passes = 2`. | [`theme.rs`](../crates/lumen-core/src/theme.rs) |
| **Minimal motion** | v0.2 interpolation of color/opacity via egui's `animate_value_with_time`, no heavy dependency. Swaps transparently to the spring solver in v0.5. | [`anim.rs`](../crates/lumen-core/src/anim.rs) |
| **Frame-N-1 state** | The technique of reading interaction state from the response produced on the previous frame (`ctx.read_response(id)`), because the current frame's hover isn't known until after layout. | [`button.rs`](../crates/lumen-widgets/src/button.rs) |
| **Façade** | The `lumen-ui` crate: it re-exports the internal crates behind feature flags and exposes a `prelude`. Consumers depend on it, not on the internal crates. | [`lumen-ui/src/lib.rs`](../crates/lumen-ui/src/lib.rs) |
