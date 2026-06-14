//! `lumen-widgets` — themed egui widgets that consume `lumen-core` recipes.
//!
//! Every widget reads the installed [`lumen_core::Theme`] for its recipe; none
//! hard-codes a color or padding. Minimal motion is integrated from v0.2 via
//! `lumen_core::anim`; it switches to the `lumen-motion` spring solver in v0.5
//! with no public API change.

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

mod badge;
mod button;
mod card;
mod checkbox;
mod switch;
mod text;

pub use badge::Badge;
pub use button::Button;
pub use card::Card;
pub use checkbox::Checkbox;
pub use switch::Switch;
pub use text::{Heading, Label};
