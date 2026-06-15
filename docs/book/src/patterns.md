# Patterns

`lumen-ui-patterns` (feature `patterns`) assembles widgets into app-shell building blocks over
egui 0.34's `Panel` API. These are opinionated layouts, not new theme surface — they reuse the
atomic widgets and the theme tokens.

## DashboardLayout

A full app shell: optional toolbar, status bar, left sidebar, right inspector, and a central area.

```rust,ignore
use lumen_ui::prelude::*;

DashboardLayout::new()
    .toolbar(|ui| { ui.add(Heading::new("My App")); })
    .sidebar(|ui| { ui.add(Label::new("nav")); })
    .inspector(|ui| { ui.add(Label::new("properties")); })
    .status_bar(|ui| { ui.add(Label::muted("Ready")); })
    .show(ui, |ui| {
        // central content
        ui.add(Label::new("main view"));
    });
```

## Sidebar navigation

Bound to a `&mut usize` (the selected index):

```rust,ignore
Sidebar::new(&mut self.section)
    .item("Home")
    .item("Settings")
    .item("About")
    .show(ui);
```

## Property rows & containers

```rust,ignore
// A label + control row:
property_row(ui, "Volume", |ui| { ui.add(Slider::new(&mut volume, 0.0..=1.0)); });

// Titled scroll/inspector containers:
SettingsPage::new("Preferences").show(ui, |ui| { /* property_rows … */ });
InspectorPanel::new("Inspector").show(ui, |ui| { /* … */ });
```

## Bars

```rust,ignore
Toolbar::new().show(ui, |ui| { ui.add(Button::ghost("File")); });
StatusBar::new().show(ui, |ui| { ui.add(Label::muted("Ln 1, Col 1")); });
```

## LogPanel

A scrollable, leveled log with severity badges and stick-to-bottom. You own the `Vec<LogEntry>`:

```rust,ignore
let entries = vec![
    LogEntry::info("started"),
    LogEntry::warn("low memory"),
    LogEntry::error("connection lost"),
];
LogPanel::new().show(ui, &entries);
```

## CommandPalette

A searchable command overlay (open state + query in `ctx.data`); returns the chosen index.

```rust,ignore
open_command_palette(ctx, "cmdk");   // e.g. on Ctrl+K
if let Some(i) = CommandPalette::new("cmdk")
    .command("Open File")
    .command("Save")
    .command("Toggle Theme")
    .show(ctx)
{
    // run command i
}
```

See `cargo run -p lumen-ui --example dashboard`. Next: [Accessibility](accessibility.md).
