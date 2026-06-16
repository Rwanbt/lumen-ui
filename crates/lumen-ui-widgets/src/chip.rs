//! [`Chip`] — a compact pill/tag, optionally removable.

use egui::{Button, Frame, Margin, Response, RichText, Ui};
use lumen_ui_core::{ChipRecipe, UiThemeExt};

/// Glyph used for the remove affordance.
const REMOVE_GLYPH: &str = "\u{00d7}"; // ×

/// A pill-shaped chip/tag. Unlike [`crate::Badge`], a chip can be removable: call
/// [`Chip::removable`] and inspect [`ChipResponse::removed`] after [`Chip::show`].
#[derive(Clone, Debug)]
pub struct Chip {
    label: String,
    removable: bool,
}

/// Outcome of showing a [`Chip`].
#[derive(Debug)]
pub struct ChipResponse {
    pub response: Response,
    /// `true` on the frame the remove affordance was clicked.
    pub removed: bool,
}

impl Chip {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            removable: false,
        }
    }

    /// Show a trailing remove (`×`) affordance.
    #[must_use]
    pub fn removable(mut self) -> Self {
        self.removable = true;
        self
    }

    pub fn show(self, ui: &mut Ui) -> ChipResponse {
        let recipe = ChipRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let mut removed = false;
        let response = Frame::NONE
            .fill(recipe.fill)
            .corner_radius(recipe.corner_radius)
            .inner_margin(Margin::symmetric(
                recipe.inner_margin.x as i8,
                recipe.inner_margin.y as i8,
            ))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(&self.label)
                            .color(recipe.text_color)
                            .size(recipe.text_size),
                    );
                    if self.removable {
                        let remove = ui.add(
                            Button::new(
                                RichText::new(REMOVE_GLYPH)
                                    .color(recipe.text_color)
                                    .size(recipe.text_size),
                            )
                            .frame(false)
                            .small(),
                        );
                        removed = remove.clicked();
                    }
                });
            })
            .response;
        ChipResponse { response, removed }
    }
}
