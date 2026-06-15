# Getting started

This page gets a themed egui app on screen in under 15 minutes.

## 1. Add the dependency

```toml
[dependencies]
lumen-ui = { git = "https://github.com/Rwanbt/lumen-ui" }
eframe = "0.34"
```

The default features are `theme`, `widgets`, `themes`. Add more as you need them:

```toml
lumen-ui = { git = "https://github.com/Rwanbt/lumen-ui", features = ["full"] }
```

| Feature | Pulls in |
|---------|----------|
| `theme` | tokens + the `Theme` trait (always on via `widgets`) |
| `widgets` | Button, TextField, Switch, Checkbox, Slider, Card, Badge, Tabs, Modal, Toast… |
| `layout` | `Flex`/`Grid`/`responsive` (pulls `egui_taffy`) |
| `motion` | `Spring`/`ease`/`fade` |
| `patterns` | `DashboardLayout`, `Sidebar`, `LogPanel`, `CommandPalette` |
| `themes` | `audio_dark()`, `high_contrast()` |
| `icons` | `Icon` widget |
| `serde` | (de)serialize tokens/themes |
| `full` | everything above |

## 2. Install a theme once, at startup

```rust,ignore
use eframe::egui;
use lumen_ui::prelude::*;
use std::sync::Arc;

struct App;

impl eframe::App for App {
    // eframe 0.34: implement `ui`, not `update`.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.add(Button::primary("Save"));
        ui.add(Button::ghost("Cancel"));
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "demo",
        Default::default(),
        Box::new(|cc| {
            // One call wires the theme + UI context into egui.
            install(&cc.egui_ctx, Arc::new(DarkTheme::new()), UiContext::default());
            Ok(Box::new(App))
        }),
    )
}
```

That's it — `install()` maps the tokens onto egui's `Style` and stores the theme so every lumen
widget can resolve its recipe. Stock egui widgets follow the theme too.

## 3. Swap the theme live

```rust,ignore
use lumen_ui::prelude::*;
use std::sync::Arc;

// Anywhere you have the `egui::Context`:
set_theme(ctx, Arc::new(LightTheme::new()));
set_theme(ctx, Arc::new(audio_dark()));      // from the `themes` feature
```

The whole app restyles on the next frame. No widget code changes.

## Run the examples

```bash
cargo run -p lumen-ui --example minimal       # live theming end-to-end
cargo run -p lumen-ui --example gallery        # every widget + 4-theme switch
cargo run -p lumen-ui --example dashboard      # app-shell pattern
cargo run -p lumen-ui --example responsive     # breakpoint-driven layout
cargo run -p lumen-ui --example motion         # spring + fade
cargo run -p lumen-ui --example icons          # icon set
```

Next: [Core concepts](concepts.md).
