# AI_CONTEXT — lumen-ui-themes

## Purpose
Extra themes beyond the core `DarkTheme`/`LightTheme`: `audio_dark()` (near-black, teal accent,
for audio/creative tools) and `high_contrast()` (WCAG-friendly max contrast). Each is just a
`lumen_ui_core::PaletteTheme` (a `Tokens` palette + a `ThemeMode`). Enabled via the façade `themes`
feature. Depends on `egui` + `lumen-ui-core`.

## Constraints
- A theme = palette + mode; recipe resolution is entirely in `lumen-ui-core` (`PaletteTheme` →
  `builder`). To add a theme, build a `Colors` palette and call `PaletteTheme::new(tokens, mode)`;
  do NOT re-implement recipes.
- `ThemeMode::Dark` lightens on hover/active; `Light` darkens. Pick the one matching the palette.

## Forbidden
- `#![forbid(unsafe_code)]`. No recipe logic here — it belongs to the core `builder`.

## Common patterns
```ignore
lumen_ui::install(ctx, std::sync::Arc::new(lumen_ui_themes::audio_dark()), Default::default());
lumen_ui::set_theme(ctx, std::sync::Arc::new(lumen_ui_themes::high_contrast()));
```

## Modules
- `lib.rs` — `audio_dark()`, `high_contrast()` (return `PaletteTheme`).
