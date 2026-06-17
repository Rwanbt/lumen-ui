# AI_CONTEXT — lumen-ui-audio

## Purpose
Audio/DAW controls for lumen-ui (the differentiator vs web design systems — lumen-ui comes from the
Seno DAW context). Painter-drawn, theme-colored widgets resolving pure recipes (ADR-0009) from the
installed `lumen_ui_core::Theme`. Depends only on `egui` + `lumen-ui-core`. Created at v1.10.

## Constraints
- Like the core widgets: resolve style from a recipe, never hard-code a color. Knob uses
  `KnobRecipe` (in `lumen-ui-core`); Fader reuses `SliderRecipe` (it is a vertical slider).
- Verify every egui signature by compilation before commit (project risk #1).
- Hit targets / sizes scale with `UiContext::density_scale()`.

## Forbidden
- `#![forbid(unsafe_code)]`, `#![warn(missing_debug_implementations)]`.
- No global state; the theme is read from egui data via `UiThemeExt`.

## Common patterns
```rust
use lumen_ui_audio::{Knob, Fader};
ui.add(Knob::new(&mut cutoff, 20.0..=20_000.0)); // drag vertically to change
ui.add(Fader::new(&mut gain_db, -60.0..=6.0));
```

## Modules
- `knob.rs` — `Knob` (`&mut f32`, range): a 270° rotary control (gap at the bottom). Vertical drag
  changes the value; the arc fills from the minimum, a pointer marks the value. Arc tessellated via
  `arc_points` → `Shape::line`. `KnobRecipe` (track/fill/indicator/size). a11y: `WidgetInfo::slider`.
- `fader.rs` — `Fader` (`&mut f32`, range): a vertical fader (max at top); click/drag sets the value.
  Reuses `SliderRecipe` (track/fill/knob). a11y: `WidgetInfo::slider`.
- `vu_meter.rs` — `VuMeter::new(level).peak(p)` (levels are `0..=1` fractions of full scale): a
  vertical meter; the fill is split into low/mid/high colored zones with an optional peak-hold line.
  Display-only. Uses `MeterRecipe`.
- `level_bar.rs` — `LevelBar::new(level)`: a horizontal bar whose fill is colored by the zone the
  level falls in. Display-only. Uses `MeterRecipe`.
- `lib.rs` — shared zone thresholds (`ZONE_LOW_MAX` 0.6, `ZONE_MID_MAX` 0.85) + `zone_color`.
- `waveform.rs` — `Waveform::new(&[f32])` (samples in `-1..=1`): per-column **min/max envelope**, so
  it stays correct for buffers far larger than the pixel width. Display-only. `WaveformRecipe`.
- `xy_pad.rs` — `XyPad::new(&mut x, &mut y, x_range, y_range)`: a square 2-D control; X left→right,
  Y bottom→top; click/drag sets both. Crosshair + point. `XyPadRecipe`. a11y: `WidgetInfo::slider` (x).
- `transport.rs` — `Transport::new().playing(b).recording(b).show(ui) -> Option<TransportAction>`
  ({PlayPause, Stop, Record}): a control bar with painter-drawn icons (triangle/bars/square/circle —
  no font glyphs), record lit while recording. Buttons carry `WidgetInfo::labeled`. `TransportRecipe`.

## Roadmap (v1.10)
Done: Knob, Fader, VuMeter, LevelBar, Waveform, XyPad, Transport. **Milestone closed** for the
control set. Spectrum analyzer deferred (needs FFT input — app-specific). Validation on a real app
(Seno / a CLAP plugin) is the remaining pre-1.0 follow-up.
