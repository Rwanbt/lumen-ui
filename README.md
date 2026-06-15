<div align="center">

<img src="assets/banner.png" alt="lumen-ui — token-driven design system for egui" width="100%">

# lumen-ui

**A token-driven, themeable design system for [egui](https://github.com/emilk/egui).**

[![CI](https://github.com/Rwanbt/lumen-ui/actions/workflows/ci.yml/badge.svg)](https://github.com/Rwanbt/lumen-ui/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![egui 0.34](https://img.shields.io/badge/egui-0.34-orange.svg)](https://github.com/emilk/egui)

</div>

<div align="center">

  **[ 🇬🇧 English &nbsp;|&nbsp; 🇫🇷 [Lire en Français](README.fr.md) ]**

</div>

> 🚧 **Work in progress — not yet released on crates.io.** The code is feature-complete and the
> public API is frozen for a `1.0.0` release candidate, but it has not been published or battle-tested
> in production yet. Use it via a git dependency and expect rough edges. Feedback welcome.

## What is this?

[**egui**](https://github.com/emilk/egui) is a popular immediate-mode GUI library for Rust — used
for desktop apps, dev tools, game UIs, and audio-plugin interfaces. It's great, but its styling is
**one global, imperative `Style`**: you mutate a single struct for the whole context, colors and
spacings end up hard-coded across your widgets, and there's no built-in notion of a *theme* you can
design once and swap.

**lumen-ui is the design-system layer egui doesn't ship.** It separates *what a widget is* from
*how it looks*:

```text
Design tokens ──(a Theme resolves)──► a Recipe per (variant, state, density) ──► the Widget paints
```

A widget never hard-codes a color or padding — it asks the installed **theme** for a **recipe**
built from semantic **tokens**. Swap the theme and your entire app restyles, instantly, **without
touching a single line of widget or app logic**.

## The problems it solves

| Pain with raw egui | What lumen-ui gives you |
|--------------------|-------------------------|
| Colors/spacings hard-coded and duplicated everywhere | **One source of visual truth** — semantic tokens, no scattered `Color32::from_rgb(...)` |
| No real theming; restyling means editing widget code | **Live theming** — `set_theme(ctx, …)` restyles the whole app in one call |
| Hard to ship dark/light/brand/high-contrast variants | A **theme = a palette + a mode**; new themes need *zero* recipe code (`PaletteTheme`) |
| Accessibility is on you | Every built-in theme is **WCAG 2.1 AA audited in CI**; visible focus, keyboard nav, 44 px touch targets |
| Inconsistent widgets, ad-hoc state | A coherent widget set + headless components (Modal/Toast/Tabs) that keep their own state |
| egui breaks ~3×/year | egui pinned behind a **single adaptation layer**; one place to bump |

## Who it's for

Anyone building a non-trivial egui app who wants a consistent, themeable, accessible look without
reinventing a styling layer — desktop tools, dashboards, creative/audio software, internal apps.

## Design principles

- **Deep, stable core** — recipes are parameterized by `(variant, state, density)` from day one, so
  adding states/variants/themes later is **additive, not breaking** ([ADR-0002](docs/adr/0002-recipes-parameterized-by-state.md)).
- **Fast** — recipe resolution is ~26 ns (a 300-widget frame spends <10 µs theming; see [docs/performance.md](docs/performance.md)).
- **Honest egui** — every egui signature is verified by compilation, never assumed.
- **Pay for what you use** — opt-in feature flags; the core pulls only `egui`.

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

## Compatibility

| lumen-ui | egui | MSRV |
|----------|------|------|
| 1.0.x | 0.34.x | Rust 1.92 |

egui is pinned in a single adaptation layer ([ADR-0004](docs/adr/0004-msrv-egui-pin.md)); a new
egui minor is handled in one place and shipped as a lumen-ui minor at most.

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
