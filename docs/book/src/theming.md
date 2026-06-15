# Theming

## Built-in themes

| Theme | Source | Notes |
|-------|--------|-------|
| `DarkTheme::new()` | `lumen-core` | default; calm neutral dark |
| `LightTheme::new()` | `lumen-core` | light counterpart |
| `audio_dark()` | `lumen-themes` | near-black, teal accent (creative tools) |
| `high_contrast()` | `lumen-themes` | maximum-contrast, WCAG-friendly |

Install or swap any of them:

```rust,ignore
use lumen_ui::prelude::*;
use std::sync::Arc;

install(ctx, Arc::new(DarkTheme::new()), UiContext::default());
set_theme(ctx, Arc::new(high_contrast()));
```

## A theme is just a palette + a mode

You rarely need to implement the `Theme` trait by hand. `PaletteTheme` builds every recipe from a
`Tokens` palette plus a `ThemeMode` (which direction hover/active shift — lighten for dark, darken
for light). That's how the whole family is built, and how you make your own:

```rust,ignore
use lumen_ui::prelude::*;            // PaletteTheme, ThemeMode
use lumen_ui::{Colors, Elevation, Motion, Radius, Spacing, Tokens, Typography};
use egui::Color32;
use std::sync::Arc;

fn brand_theme() -> PaletteTheme {
    let colors = Colors {
        background: Color32::from_rgb(0x10, 0x10, 0x16),
        surface: Color32::from_rgb(0x1a, 0x1a, 0x24),
        primary: Color32::from_rgb(0x7c, 0x5c, 0xff),
        on_primary: Color32::WHITE,
        // … fill in the remaining semantic roles …
        ..DarkTheme::new().tokens().colors.clone()
    };
    PaletteTheme::new(
        Tokens {
            colors,
            spacing: Spacing::default(),
            radius: Radius::default(),
            typography: Typography::default(),
            elevation: Elevation::default(),
            motion: Motion::default(),
        },
        ThemeMode::Dark,
    )
}

install(ctx, Arc::new(brand_theme()), UiContext::default());
```

> **Check your palette is accessible.** Before shipping a custom theme, audit it:
> ```rust,ignore
> let report = lumen_ui::a11y::audit_colors(&brand_theme().tokens().colors);
> assert!(report.all_pass(), "{:?}", report.failures().collect::<Vec<_>>());
> ```
> See [Accessibility](accessibility.md).

## Scaffolding a theme with the CLI

`lumen-theme-gen` (in `tools/`) round-trips a palette through RON so you can tweak colors by hand:

```bash
# 1. Emit a starter palette (RON) from the dark theme:
cargo run -p lumen-theme-gen -- template > brand.ron

# 2. Edit brand.ron …

# 3. Generate a ready-to-paste PaletteTheme function:
cargo run -p lumen-theme-gen -- gen brand.ron > src/brand_theme.rs
```

Next: [Widgets](widgets.md).
