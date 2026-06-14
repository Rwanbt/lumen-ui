<div align="center">

# lumen-ui

**A token-driven, themeable design system for [egui](https://github.com/emilk/egui).**

[![CI](https://github.com/Rwanbt/lumen-ui/actions/workflows/ci.yml/badge.svg)](https://github.com/Rwanbt/lumen-ui/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![egui 0.34](https://img.shields.io/badge/egui-0.34-orange.svg)](https://github.com/emilk/egui)

</div>

> ⚠️ **Status: v0.1 (foundation).** API is unstable until v1.0. See [ROADMAP.md](ROADMAP.md).

`lumen-ui` separates *what a widget is* from *how it looks*. Widgets read a **recipe**
resolved by a **theme** from a set of **design tokens** for a given
`(variant, state, density)`. Swap the theme and the whole app restyles — without touching
any widget code.

## Why

- **One source of visual truth** — tokens, not scattered `Color32::from_rgb(...)` calls.
- **Live theming** — `lumen_ui::set_theme(ctx, …)` restyles everything, instantly.
- **State-parameterized recipes from v0.1** — adding interaction states later is *not* a
  breaking change to the foundation trait.
- **Built on the real egui 0.34 API** — every signature is verified by compilation, not
  hallucinated (see [ROADMAP.md §Corrections d'API](ROADMAP.md)).

## Quick start

```toml
[dependencies]
lumen-ui = { git = "https://github.com/Rwanbt/lumen-ui" }   # crates.io at v1.0
eframe = "0.34"
```

```rust
use std::sync::Arc;
use eframe::egui;
use lumen_ui::prelude::*;

// once, at startup:
install(&cc.egui_ctx, Arc::new(DarkTheme::new()), UiContext::default());

// anywhere in your UI:
if ui.add(Button::primary("Save")).clicked() {
    // ...
}
```

Run the validation example:

```bash
cargo run -p lumen-ui --example minimal
```

## Workspace layout

| Crate | Role | Lands in |
|-------|------|----------|
| `lumen-core` | Tokens, density/context, `Theme` trait, recipes, `install()` | v0.1 ✅ |
| `lumen-widgets` | Themed widgets (`Button`, …) | v0.1 ✅ |
| `lumen-ui` | Façade: re-exports, prelude, feature flags | v0.1 ✅ |
| `lumen-motion` | Springs, easings, timelines | v0.5 |
| `lumen-layout` | `egui_taffy` flex/grid + breakpoints | v0.4 |
| `lumen-patterns` | Dashboard, Sidebar, Inspector… | v0.6 |
| `lumen-icons` / `lumen-themes` / `lumen-material` | Icons, theme family, Material adapters | v0.7–v0.8 |

We build a **workspace from day zero** but only create crates as their version arrives —
no "big split" later. See [docs/adr/0001-workspace-from-day-zero.md](docs/adr/0001-workspace-from-day-zero.md).

## Documentation

- [ROADMAP.md](ROADMAP.md) — the locked plan, version by version.
- [ARCHITECTURE.md](ARCHITECTURE.md) — layers, data flow, ownership, red zones.
- [CONTRIBUTING.md](CONTRIBUTING.md) — conventions and PR checklist.
- [docs/glossary.md](docs/glossary.md) — domain vocabulary (token, recipe, density…).
- [docs/adr/](docs/adr/) — architecture decision records.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option. Unless you explicitly state otherwise, any
contribution intentionally submitted for inclusion in this crate by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
