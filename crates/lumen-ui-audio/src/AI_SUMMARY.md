# AI_SUMMARY — src

> **Auto-generated 2026-06-18 00:00** — do not edit manually.
> Source: `tools/ai_docs/generate_ai_summary.py`
> For purpose, thread model and constraints, read `AI_CONTEXT.md`.

## Purpose
Signal-**display** widgets for lumen-ui (the DAW differentiator): level meters and a waveform.
Painter-drawn, theme-colored, resolving pure recipes (ADR-0009) from the installed
`lumen_ui_core::Theme`. Depends only on `egui` + `lumen-ui-core`.

**Scope note (v1.1 reclassification):** the generic controls a DAW also uses — `Knob`, `Fader`,
`XyPad`, `Transport` — live in **`lumen-ui-widgets`**, because nothing about them is audio-specific.
This crate keeps only the genuinely audio-flavored *displays*.

## Files & LOC
| File | LOC | |
|------|-----|--|
| `level_bar.rs` | 41 | |
| `lib.rs` | 40 | |
| `vu_meter.rs` | 71 | |
| `waveform.rs` | 60 | |
| **Total** | **212** | |

## Rust API
- `LevelBar` (struct)
- `VuMeter` (struct)
- `Waveform` (struct)
