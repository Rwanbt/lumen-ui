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

## Roadmap (v1.10, in progress)
Done: Knob, Fader. Next: VU/Peak meter, LevelBar, Waveform, Spectrum, XY pad, Transport. To be
validated on a real app (Seno / a CLAP plugin) before the milestone closes.
