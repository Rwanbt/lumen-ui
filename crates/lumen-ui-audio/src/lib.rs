//! `lumen-ui-audio` — audio/DAW controls for **lumen-ui**.
//!
//! lumen-ui was born from a DAW context (Seno); no web design system offers these, so they are a
//! unique differentiator. Each control is painter-drawn and theme-colored: it resolves its style
//! from a pure recipe (cf. ADR-0009) just like the core widgets, and reads the installed
//! [`lumen_ui_core::Theme`].
//!
//! Enable via the `audio` feature of the `lumen-ui` façade.
//!
//! ```ignore
//! use lumen_ui_audio::{Knob, Fader};
//!
//! ui.add(Knob::new(&mut cutoff, 20.0..=20_000.0));
//! ui.add(Fader::new(&mut gain_db, -60.0..=6.0));
//! ```

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

mod fader;
mod knob;
mod level_bar;
mod vu_meter;
mod waveform;
mod xy_pad;

pub use fader::Fader;
pub use knob::Knob;
pub use level_bar::LevelBar;
pub use vu_meter::VuMeter;
pub use waveform::Waveform;
pub use xy_pad::XyPad;

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
