# AI_SUMMARY — src

> **Auto-generated 2026-06-19 08:24** — do not edit manually.
> Source: `tools/ai_docs/generate_ai_summary.py`
> For purpose, thread model and constraints, read `AI_CONTEXT.md`.

## Purpose
Extra themes beyond the core `DarkTheme`/`LightTheme`: `audio_dark()` (near-black, teal accent,
for audio/creative tools) and `high_contrast()` (WCAG-friendly max contrast). Each is just a
`lumen_ui_core::PaletteTheme` (a `Tokens` palette + a `ThemeMode`). Enabled via the façade `themes`
feature. Depends on `egui` + `lumen-ui-core`.

## Files & LOC
| File | LOC | |
|------|-----|--|
| `lib.rs` | 330 | |
| **Total** | **330** | |

## Rust API
- `ThemeBuilder` (struct)

## Rust Functions
- `audio_dark()`
- `high_contrast()`
- `nord()`
- `seno_dawn()`
- `seno_night()`
- `solarized_dark()`
- `system_mode()`
