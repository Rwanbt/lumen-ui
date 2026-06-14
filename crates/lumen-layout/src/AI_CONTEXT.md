# AI_CONTEXT — lumen-layout

## Purpose
CSS-style flexbox layout + responsive breakpoints for lumen-ui, over `egui_taffy`
(the roadmap's highest-risk dependency, isolated here behind the façade `layout` feature).
Depends only on `egui` + `egui_taffy` — not on `lumen-core`.

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
use lumen_layout::{Flex, FlexUiExt, Justify, Align, responsive, Breakpoint};

Flex::row().gap(8.0).fill_width().justify(Justify::SpaceBetween).show(ui, "bar", |t| {
    t.item(|ui| { ui.label("left"); });
    t.item_grow(|ui| { ui.label("stretches"); });
});

let columns = responsive(ui, |bp| if bp <= Breakpoint::Sm { 1 } else { 3 });
```

## Modules
- `lib.rs` — `Flex` (row/column, gap/justify/align/fill_width), `FlexUiExt` (item/item_grow/nest),
  `Justify`/`Align`, `Breakpoint` + `responsive(ui, |bp| ...)`.
