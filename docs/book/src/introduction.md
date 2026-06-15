# lumen-ui

**lumen-ui** is a token-driven, themeable design system for
[egui](https://github.com/emilk/egui) (Rust, egui 0.34).

The idea in one sentence: **widgets read a *recipe* resolved by a *theme* from design *tokens*
for a given `(variant, state, density)`.** Swap the theme and the whole app restyles — zero
widget or app-logic changes.

```text
Tokens  ──(a Theme resolves)──►  Recipe per (variant, state, density)  ──►  Widget draws
```

## Why

egui's stock styling is global and imperative: you mutate one `Style` for the whole context.
lumen-ui adds a thin indirection — a `Theme` that maps semantic tokens to per-widget recipes —
so you can:

- ship multiple themes (dark, light, high-contrast, brand) and switch them live;
- keep widgets free of hard-coded colors, paddings, and radii;
- guarantee accessibility (every built-in theme is WCAG-AA audited in CI);
- stay fast (recipe resolution is ~26 ns — see [performance](https://github.com/Rwanbt/lumen-ui/blob/main/docs/performance.md)).

## What's inside

| Crate | Role |
|-------|------|
| `lumen-core` | tokens, density/context, the `Theme` trait + recipes, `install()`, contrast audit |
| `lumen-widgets` | themed widgets (Button, TextField, Switch, Slider, Card, …) |
| `lumen-layout` | fl/grid layout over `egui_taffy`, responsive breakpoints |
| `lumen-motion` | spring + easing animation |
| `lumen-patterns` | app-shell patterns (DashboardLayout, Sidebar, LogPanel, CommandPalette) |
| `lumen-themes` | extra themes (`audio_dark`, `high_contrast`) |
| `lumen-icons` | painter-drawn icon set |
| `lumen-ui` | façade: re-exports + prelude + feature flags |

You depend on the **`lumen-ui`** façade and turn on the features you need. Start with
[Getting started](getting-started.md).

License: MIT OR Apache-2.0.
