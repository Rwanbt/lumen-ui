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
- `lib.rs` — `audio_dark()`, `high_contrast()`, `nord()`, `solarized_dark()`, and `seno_night()`
  (the Seno DAW / Dynama / Spectra house palette — layered near-black + signature accent `#e8653d`;
  `danger` deepened from Seno's signal-red so button text clears AA).
  (all return `PaletteTheme`). **Every theme has a `*_passes_wcag_aa` test** (`audit_colors`):
  preset palettes are tuned (darkened danger, dark-on-yellow `on_warning`, brightened text) so all
  10 audited pairs clear AA — keep that invariant when adding a preset.
  v1.9 tooling: `ThemeBuilder::new(bg, accent).mode(..).build()` derives a full AA-oriented palette
  (surfaces/border via `mix` from bg→text, `text`/`on_*` via `readable_on` near-white/near-black,
  mode inferred from bg luminance) — best-effort (mid-tone seeds can't satisfy AA both ways).
  `system_mode(ctx)` maps `ctx.system_theme()` → `ThemeMode` (dark when unknown) for auto
  `prefers-color-scheme`.
