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

pub use fader::Fader;
pub use knob::Knob;
