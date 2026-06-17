# AI_CONTEXT — lumen-ui-layout

## Purpose
CSS-style flexbox layout + responsive breakpoints for lumen-ui, over `egui_taffy`
(the roadmap's highest-risk dependency, isolated here behind the façade `layout` feature).
Depends only on `egui` + `egui_taffy` — not on `lumen-ui-core`.

## Constraints
- Requires `lumen_ui::install(..)` to have run: it sets `max_passes = 2`, which taffy needs to
  resolve sizes before painting (otherwise the first frame mismeasures).
- `egui_taffy 0.12` is pinned via the workspace and verified against egui 0.34.3. A bump must be
  re-checked for compat (see ROADMAP.md §E).
- `Flex::show`'s closure receives `&mut egui_taffy::Tui`; add children through the `FlexUiExt`
  trait (`item`, `item_grow`, `nest`) — don't call raw taffy APIs from widget code.

## Forbidden
- `#![forbid(unsafe_code)]`. No panics in layout paths.
- Don't leak `taffy::Style` into public lumen APIs beyond what `Flex` exposes.

## Common patterns
```rust
use lumen_ui_layout::{Flex, FlexUiExt, Justify, Align, responsive, Breakpoint};

Flex::row().gap(8.0).fill_width().justify(Justify::SpaceBetween).show(ui, "bar", |t| {
    t.item(|ui| { ui.label("left"); });
    t.item_grow(|ui| { ui.label("stretches"); });
});

let columns = responsive(ui, |bp| if bp <= Breakpoint::Sm { 1 } else { 3 });
```

## Modules
- `lib.rs` — `Flex` (row/column, gap/justify/align/fill_width), `Grid` (N equal columns),
  `FlexUiExt` (item/item_grow/nest), `Justify`/`Align`, `Breakpoint` + `responsive(ui, |bp| ...)`.
  v1.7 sizing primitives (**pure egui, no taffy**): `Container::new(max_width)` (CSS max-width +
  auto margins, via `set_max_width`) and `AspectRatio::{new,widescreen,square}` (ratio-sized box
  via `allocate_ui`). Their layout math is in pure helpers (`container_layout`, `aspect_box`) with
  unit tests — no recipe (structural, not themed).
  `ResizableSplit::{horizontal,vertical}` — two panes with a draggable divider; the first pane's
  fraction persists in egui memory (keyed by id). Divider color comes from egui visuals (the crate
  has no `lumen-ui-core` dep), drag cursor via `CursorIcon::Resize*`. `clamp_fraction` (pure, tested)
  keeps both panes above `min_fraction`. Fills the available size — constrain the cross axis.
