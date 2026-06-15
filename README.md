<div align="center">

# lumen-ui

**A token-driven, themeable design system for [egui](https://github.com/emilk/egui).**

[![CI](https://github.com/Rwanbt/lumen-ui/actions/workflows/ci.yml/badge.svg)](https://github.com/Rwanbt/lumen-ui/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![egui 0.34](https://img.shields.io/badge/egui-0.34-orange.svg)](https://github.com/emilk/egui)

</div>

> **Status: v0.9 (API-freeze candidate).** All planned crates exist and are feature-complete;
> the public API is frozen pending the v1.0 release. See [ROADMAP.md](ROADMAP.md) and
> [docs/api-freeze.md](docs/api-freeze.md). Full guide: **[the book](docs/book/)**
> (`mdbook serve docs/book`).

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

| Crate | Role | Feature | Status |
|-------|------|---------|--------|
| `lumen-core` | Tokens, density/context, `Theme` trait, recipes, `install()`, a11y audit | `theme` | ✅ |
| `lumen-widgets` | Themed widgets (Button, TextField, Switch, Slider, Card, Tabs, Modal, Toast…) | `widgets` | ✅ |
| `lumen-layout` | `egui_taffy` flex/grid + responsive breakpoints | `layout` | ✅ |
| `lumen-motion` | Springs, easings, fade transitions | `motion` | ✅ |
| `lumen-patterns` | DashboardLayout, Sidebar, LogPanel, CommandPalette… | `patterns` | ✅ |
| `lumen-themes` | Theme family (`audio_dark`, `high_contrast`) | `themes` | ✅ |
| `lumen-icons` | Painter-drawn icon set | `icons` | ✅ |
| `lumen-ui` | Façade: re-exports, prelude, feature flags | — | ✅ |
| `tools/lumen-theme-gen` | CLI: RON palette ⇄ Rust `PaletteTheme` | — | ✅ |

A **workspace from day zero** (no "big split" later);
[ADR-0001](docs/adr/0001-workspace-from-day-zero.md). Enable everything with `features = ["full"]`.

> A `material` adapter (egui-material3) was planned but **deferred** — that crate targets egui 0.33
> and pulls ~465 transitive deps; [ADR-0005](docs/adr/0005-defer-material-adapter.md).

## Accessibility

Every built-in theme is **WCAG 2.1 AA audited in CI**. Widgets are keyboard-navigable with a
visible focus ring and 44 px touch targets in `Density::Touch`. See
[the a11y chapter](docs/book/src/accessibility.md).

## Documentation

- **[The book](docs/book/)** — complete guide (getting started → patterns). `mdbook serve docs/book`.
- [ROADMAP.md](ROADMAP.md) — the locked plan, version by version.
- [ARCHITECTURE.md](ARCHITECTURE.md) — layers, data flow, ownership, red zones.
- [docs/api-freeze.md](docs/api-freeze.md) — the v1.0 frozen public surface.
- [docs/performance.md](docs/performance.md) — hot-path budgets + measured numbers.
- [CONTRIBUTING.md](CONTRIBUTING.md) · [docs/glossary.md](docs/glossary.md) · [docs/adr/](docs/adr/)

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option. Unless you explicitly state otherwise, any
contribution intentionally submitted for inclusion in this crate by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
