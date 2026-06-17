//! `lumen-ui-audio` — signal-display widgets for **lumen-ui** (the DAW differentiator).
//!
//! These are the genuinely audio-flavored *displays* — level meters and a waveform. The generic
//! controls a DAW also uses (Knob, Fader, XyPad, Transport) live in `lumen-ui-widgets`, since
//! nothing about them is audio-specific. Like every lumen widget, each display is painter-drawn and
//! resolves a pure recipe (cf. ADR-0009) from the installed [`lumen_ui_core::Theme`].
//!
//! Display-only: you pass values your own DSP computes (a `0..=1` level, a `&[f32]` of samples) and
//! the widget draws them — there is no audio processing here.
//!
//! Enable via the `audio` feature of the `lumen-ui` façade.
//!
//! ```ignore
//! use lumen_ui_audio::{VuMeter, Waveform};
//!
//! ui.add(VuMeter::new(level).peak(peak)); // level/peak are 0..=1 fractions of full scale
//! ui.add(Waveform::new(&samples));        // samples in -1.0..=1.0
//! ```

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

mod level_bar;
mod vu_meter;
mod waveform;

pub use level_bar::LevelBar;
pub use vu_meter::VuMeter;
pub use waveform::Waveform;

use egui::Color32;
use lumen_ui_core::MeterRecipe;

/// Upper bound of the low (safe) meter zone, as a fraction of full scale.
pub(crate) const ZONE_LOW_MAX: f32 = 0.6;
/// Upper bound of the mid (caution) meter zone, as a fraction of full scale.
pub(crate) const ZONE_MID_MAX: f32 = 0.85;

/// Color for a level `t` (`0..=1`) under the three-zone meter scheme.
pub(crate) fn zone_color(t: f32, recipe: &MeterRecipe) -> Color32 {
    if t <= ZONE_LOW_MAX {
        recipe.low
    } else if t <= ZONE_MID_MAX {
        recipe.mid
    } else {
        recipe.high
    }
}
