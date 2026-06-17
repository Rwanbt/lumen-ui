# AI_CONTEXT — lumen-ui-audio

## Purpose
Signal-**display** widgets for lumen-ui (the DAW differentiator): level meters and a waveform.
Painter-drawn, theme-colored, resolving pure recipes (ADR-0009) from the installed
`lumen_ui_core::Theme`. Depends only on `egui` + `lumen-ui-core`.

**Scope note (v1.1 reclassification):** the generic controls a DAW also uses — `Knob`, `Fader`,
`XyPad`, `Transport` — live in **`lumen-ui-widgets`**, because nothing about them is audio-specific.
This crate keeps only the genuinely audio-flavored *displays*.

## Constraints
- Display-only: the caller passes values its own DSP computes (a `0..=1` level, a `&[f32]` of
  samples); there is **no audio processing** here. Ballistics/smoothing/peak-decay are the caller's.
- Resolve style from a recipe, never hard-code a color. Meters use `MeterRecipe`, Waveform uses
  `WaveformRecipe` (both in `lumen-ui-core`).
- Verify every egui signature by compilation before commit. Sizes scale with `density_scale()`.

## Forbidden
- `#![forbid(unsafe_code)]`, `#![warn(missing_debug_implementations)]`. No global state.

## Common patterns
```rust
use lumen_ui_audio::{VuMeter, Waveform};
ui.add(VuMeter::new(level).peak(peak)); // 0..=1 fractions of full scale
ui.add(Waveform::new(&samples));        // samples in -1.0..=1.0
```

## Modules
- `vu_meter.rs` — `VuMeter::new(level).peak(p)` (levels `0..=1`): vertical meter. **Default: one
  color chosen by the current level** (`zone_color`) — the software-meter look; `.segmented()`
  switches to stacked LED zones. Tick marks at the zone thresholds + optional peak-hold line.
  Uses `MeterRecipe` (incl. `tick`).
- `level_bar.rs` — `LevelBar::new(level)`: horizontal bar whose fill is colored by the level's zone.
- `waveform.rs` — `Waveform::new(&[f32])` (samples in `-1..=1`): per-column **min/max envelope**
  rendered as a **filled `epaint::Mesh` body + top/bottom outline** (Seno/Dynama/Spectra house
  style). Default **symmetric** (`|amp|` mirrored); `.signed()` draws the true min/max envelope.
  Uses `WaveformRecipe` (incl. translucent `fill`).
- `lib.rs` — shared zone thresholds (`ZONE_LOW_MAX` 0.6, `ZONE_MID_MAX` 0.85) + `zone_color`.

## Reference implementations (real projects)
Seno DAW + the Dynama/Spectra CLAP plugins draw signals with a **filled `epaint::Mesh` + a 1–1.5 px
polyline outline**, evaluated **per screen column**, dB-mapped with a floor; their level meter fills
with **one color chosen by the current value** (not stacked zones) plus dB ticks + a numeric
readout. The v1.1 style pass aligns these displays with that house style. The snapshot test
(`tests/snapshot.rs`, `#[ignore]`, run by the `snapshots` workflow) guards the pixels.

## Roadmap
Done: VuMeter, LevelBar, Waveform. Deferred: Spectrum analyzer (needs an FFT input — app-specific).
Validation inside a real DAW (Seno / a CLAP plugin) is the remaining pre-1.0 follow-up.
