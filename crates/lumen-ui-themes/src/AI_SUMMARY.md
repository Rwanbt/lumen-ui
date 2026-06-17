# AI_SUMMARY — src

> **Auto-generated 2026-06-17 12:16** — do not edit manually.
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
| `lib.rs` | 103 | |
| **Total** | **103** | |

## Rust API

## Rust Functions
- `audio_dark()`
- `high_contrast()`
