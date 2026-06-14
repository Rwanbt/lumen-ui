# AI_CONTEXT — lumen-widgets

## Purpose
Themed egui widgets that consume `lumen-core` recipes. v0.1 ships `Button`; v0.2 adds the
foundational set (Input, Card, Badge, Switch, Checkbox, RadioGroup, Slider, Label, Heading).
Widgets are the only place that calls egui's drawing API besides `lumen-core::theme`.

## Constraints
- A widget MUST resolve its style from `ui.theme().<widget>_recipe(variant, state, &ui.ui_ctx())`.
  Never hard-code a color, padding, or radius.
- Interaction state is read from the **previous frame** via `ctx.read_response(id)` — egui only
  knows hover/active after allocation. `install()` (in core) sets `max_passes = 2` for this.
- egui 0.34 reality: `egui::Button` has no `.padding()`/`.shadow()`. Padding + shadow come from
  wrapping the button in an `egui::Frame` (`inner_margin`, `shadow`, `corner_radius`, `fill`).
- Widgets are **stateless**: no app-owned booleans. From v0.3, composed components keep their
  open/visible state in `ctx.data` keyed by `Id`.

## Forbidden
- No `unwrap()`/`expect()` outside tests. `#![forbid(unsafe_code)]`.
- Do not store an `egui::Button` (it borrows its content atoms `Button<'a>`); build and `add`
  it inline at draw time.

## Common patterns
```rust
use lumen_widgets::Button;

if ui.add(Button::primary("Save")).clicked() { /* ... */ }
ui.add(Button::ghost("Cancel").enabled(false));
```

## Modules
- `button.rs` — the `Button` widget (Primary/Secondary/Ghost/Danger), Frame+Button technique.
  v0.2: fill interpolates via `lumen_core::anim::lerp_color` (minimal motion).
- `text.rs` — `Label` (+ `muted`) and `Heading` (+ `display`); resolve color/size from
  `Theme::text_recipe(role, ctx)`.
- `card.rs` — `Card` themed surface container; exposes `show(ui, |ui| …)` (not a `Widget`).
- `badge.rs` — `Badge` status label (Neutral/Primary/Success/Warning/Danger).
- `switch.rs` — `Switch` (`&mut bool`), animated knob; custom painter (rect+circle).
- `checkbox.rs` — `Checkbox` (`&mut bool` + label); custom box + check mark.
  Both use `Theme::toggle_recipe`.
