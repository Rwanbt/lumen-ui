//! `lumen-widgets` — themed egui widgets that consume `lumen-core` recipes.
//!
//! Every widget reads the installed [`lumen_core::Theme`] for its recipe; none
//! hard-codes a color or padding. Minimal motion is integrated from v0.2 via
//! `lumen_core::anim`; it switches to the `lumen-motion` spring solver in v0.5
//! with no public API change.

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

mod button;

pub use button::Button;
