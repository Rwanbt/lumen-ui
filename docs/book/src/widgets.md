# Widgets

All widgets live in `lumen-ui-widgets` (feature `widgets`) and re-export from the façade. They're
**stateless**: bind them to your data with `&mut`, or — for composed components — let them keep
their open/visible state in `ctx.data` keyed by an `Id`. None of them require an app-owned boolean.

## Atoms

```rust,ignore
use lumen_ui::prelude::*;

// Button — Primary / Secondary / Ghost / Danger, with .enabled(bool)
if ui.add(Button::primary("Save")).clicked() { /* … */ }
ui.add(Button::danger("Delete").enabled(can_delete));

// Text — Label (+ muted) and Heading (+ display)
ui.add(Label::new("body text"));
ui.add(Label::muted("secondary"));
ui.add(Heading::new("Section"));

// Inputs bound to your state
ui.add(TextField::new(&mut name).hint("Your name"));
ui.add(TextField::new(&mut password).password(true));
ui.add(Switch::new(&mut enabled));
ui.add(Checkbox::new(&mut agree, "I agree"));
ui.add(Slider::new(&mut volume, 0.0..=1.0));

// Single selection — RadioGroup is a Widget; it mutates the bound `&mut T` in place,
// so read `selected` after adding it.
ui.add(
    RadioGroup::new(&mut selected)
        .option(Mode::A, "Mode A")
        .option(Mode::B, "Mode B"),
);

// Containers / status
Card::new().show(ui, |ui| { ui.add(Label::new("inside a card")); });
ui.add(Badge::success("OK"));
```

## Composed components

```rust,ignore
// Tabs — selection persisted in ctx.data, returns the active index
let active = Tabs::new("main-tabs").tab("Files").tab("Search").show(ui);

// Accordion — themed collapsible
Accordion::new("advanced").show(ui, |ui| { /* … */ });

// Select<T> dropdown bound to &mut T (first arg is an id source)
Select::new("mode", &mut current).option(A, "A").option(B, "B").show(ui);

// Modal — open state in ctx.data, no external bool. Esc / backdrop closes it.
// `show` takes the egui Context (not a Ui), since the modal renders at the top level.
open_modal(ctx, "confirm");
Modal::new("confirm").show(ctx, |ui| {
    ui.add(Label::new("Are you sure?"));
    if ui.add(Button::primary("Yes")).clicked() { close_modal(ctx, "confirm"); }
});

// Toasts — push from anywhere, render once per frame
toast_success(ctx, "Saved");
toast_error(ctx, "Failed");
show_toasts(ctx);    // call once per frame, e.g. at the end of your top panel
```

## Overlays

```rust,ignore
let resp = ui.add(Button::ghost("hover me"));
popover(&resp, |ui| { ui.add(Label::new("popover body")); });
context_menu(&resp, |ui| { if ui.add(Button::ghost("Copy")).clicked() {} });
tooltip(resp, "extra info"); // takes the Response by value — call it last
```

Every one of these resolves its styling from the installed theme — swap the theme and they all
restyle. They also follow density and show a focus ring on keyboard focus (see
[Accessibility](accessibility.md)).

Next: [Layout](layout.md).
