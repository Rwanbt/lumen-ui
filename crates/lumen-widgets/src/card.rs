//! [`Card`] — a themed surface container.
//!
//! Unlike most widgets, a card *wraps* content, so it exposes [`Card::show`]
//! (like egui's own containers) rather than implementing `Widget`.

use egui::{Frame, InnerResponse, Margin, Ui};
use lumen_core::UiThemeExt;

/// A themed surface: fill, border, corner radius, elevation, and padding all
/// resolved from the theme's `card_recipe`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Card;

impl Card {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Draw the card and its contents. Returns the closure's value alongside the response.
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let r = ui.theme().card_recipe(&ui.ui_ctx());
        Frame::NONE
            .fill(r.fill)
            .stroke(r.stroke)
            .corner_radius(r.corner_radius)
            .shadow(r.shadow)
            .inner_margin(Margin::symmetric(
                r.inner_margin.x as i8,
                r.inner_margin.y as i8,
            ))
            .show(ui, add_contents)
    }
}
