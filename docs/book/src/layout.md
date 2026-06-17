# Layout

`lumen-ui-layout` (feature `layout`) wraps [`egui_taffy`](https://crates.io/crates/egui_taffy) to
give egui CSS-style flexbox and grid, plus responsive breakpoints. `install()` already enables the
two-pass layout these need.

## Flex

```rust,ignore
use lumen_ui::prelude::*;   // Flex, Grid, FlexUiExt, Justify, Align

Flex::row()
    .gap(8.0)
    .justify(Justify::SpaceBetween)
    .align(Align::Center)
    .fill_width()
    .show(ui, "toolbar", |t| {
        t.item(|ui| { ui.add(Heading::new("Title")); });
        t.item_grow(|ui| {});                 // spacer that eats remaining width
        t.item(|ui| { ui.add(Button::primary("Action")); });
    });

Flex::column().gap(12.0).show(ui, "stack", |t| {
    t.item(|ui| { ui.add(Label::new("row 1")); });
    t.item(|ui| { ui.add(Label::new("row 2")); });
});
```

`FlexUiExt` adds three methods inside the closure:

| Method | Effect |
|--------|--------|
| `item(\|ui\| …)` | an item sized to its content |
| `item_grow(\|ui\| …)` | an item that grows to fill remaining space (`flex-grow: 1`) |
| `nest(flex, \|t\| …)` | a nested flex container |

## Grid

Equal columns, with a gap:

```rust,ignore
Grid::new(3).gap(8.0).fill_width().show(ui, "cards", |t| {
    for card in &cards {
        t.item(|ui| { Card::new().show(ui, |ui| { ui.add(Label::new(&card.name)); }); });
    }
});
```

## Responsive

`responsive` resolves a [`Breakpoint`] (`Xs`/`Sm`/`Md`/`Lg`/`Xl`) from the available width, so the
same code adapts to the window size:

```rust,ignore
let columns = responsive(ui, |bp| match bp {
    Breakpoint::Xs | Breakpoint::Sm => 1,
    Breakpoint::Md => 2,
    _ => 3,
});
Grid::new(columns).gap(8.0).fill_width().show(ui, "grid", |t| { /* … */ });
```

See `cargo run -p lumen-ui --example responsive`.

## v2 primitives (pure egui)

Beyond Flex/Grid, lumen-ui-layout adds sizing and structure primitives that need no taffy:

```rust,ignore
use lumen_ui::prelude::*;

// Container — CSS max-width + auto margins (centers above the cap, fills below).
Container::new(960.0).show(ui, |ui| { /* page content */ });

// AspectRatio — a ratio-sized box for media/previews.
AspectRatio::widescreen().show(ui, |ui| { /* 16:9 */ });

// ResizableSplit — two panes with a draggable divider (fraction persisted in egui memory).
ResizableSplit::horizontal("split").show(ui, |left| { /* … */ }, |right| { /* … */ });

// Stack — a plain list with a uniform gap or themed separators between items.
Stack::vertical().gap(8.0).separators(true).show(ui, |s| {
    s.item(|ui| { ui.add(Label::new("one")); });
    s.item(|ui| { ui.add(Label::new("two")); });
});

// Scroll — an ergonomic ScrollArea wrapper (fills the space; optional max size).
Scroll::vertical().max_height(240.0).show(ui, |ui| { /* long content */ });

// GridTemplate — explicit, mixed column tracks (CSS grid-template-columns).
GridTemplate::columns([Track::Px(120.0), Track::Fr(1.0), Track::Auto])
    .gap(8.0)
    .show(ui, "grid", |t| { /* cells, row-major */ });
```

Next: [Audio](audio.md).
