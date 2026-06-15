# Core concepts

Four ideas carry the whole design system. Understand these and the rest is mechanical.

## Tokens

**Tokens** are the raw visual constants — the single source of truth. A widget never reads them
directly.

```rust,ignore
pub struct Tokens {
    pub colors: Colors,        // semantic roles: background, surface, primary, on_primary, …
    pub spacing: Spacing,      // xs / sm / md / lg / xl
    pub radius: Radius,        // sm / md / lg / full
    pub typography: Typography,// body / label / heading / display
    pub elevation: Elevation,  // none / low / high (egui Shadow)
    pub motion: Motion,        // fast / base / slow (seconds)
}
```

`Colors` are **semantic** (`primary`, `on_primary`, `danger`, `text_muted`…), not a raw swatch
palette. `on_*` is the foreground meant to sit on top of its fill — and those pairs are
WCAG-audited (see [Accessibility](accessibility.md)).

## Recipes

A **recipe** is the fully-resolved styling a widget needs to paint *one* state — colors, stroke,
corner radius, shadow, inner margin. It carries no logic.

```rust,ignore
pub struct ButtonRecipe {
    pub fill: Color32,
    pub text_color: Color32,
    pub stroke: Stroke,
    pub corner_radius: CornerRadius,
    pub shadow: Shadow,
    pub inner_margin: Vec2,
}
```

There is a recipe per widget family: `ButtonRecipe`, `TextRecipe`, `CardRecipe`, `BadgeRecipe`,
`ToggleRecipe`, `SliderRecipe`, `TextFieldRecipe`.

## The Theme trait

A **theme** owns tokens and maps `(variant, state, context)` → recipe. This is the *frozen core*:
recipe methods are parameterized by state from day one, so adding states/variants later is not a
breaking change.

```rust,ignore
pub trait Theme: Send + Sync {
    fn tokens(&self) -> &Tokens;
    fn button_recipe(&self, variant: ButtonVariant, state: WidgetState, ctx: &UiContext) -> ButtonRecipe;
    fn text_recipe(&self, role: TextRole, ctx: &UiContext) -> TextRecipe;
    // … card / badge / toggle / slider / text_field …
    fn apply_to_ctx(&self, ctx: &egui::Context); // maps tokens onto egui Style
}
```

`WidgetState` is `Normal | Hovered | Active | Focused | Disabled`. egui only knows hover/active
*after* allocation, so widgets read the **previous frame's** response — `install()` sets
`max_passes = 2` so this is stable.

## Density / UiContext

`UiContext` is ambient, theme-independent display state. Today it carries **density**:

```rust,ignore
pub enum Density { Compact, Comfortable, Touch }
```

Recipes receive the context, so one theme adapts its paddings and **hit targets** to the device:
`UiContext::min_interactive_size()` returns 44 px in `Touch` (WCAG 2.5.5). `UiContext` is
extensible (e.g. a future high-contrast flag) without touching the `Theme` trait.

---

The flow each frame, per widget:

```text
ui.theme()                       // the installed Arc<dyn Theme>
  .button_recipe(variant, state, &ui.ui_ctx())
  → ButtonRecipe                 // resolved styling
  → widget paints with it
```

Next: [Theming](theming.md).
