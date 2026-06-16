//! [`Code`] — an inline monospace code span themed by tokens.

use egui::{Frame, Margin, Response, RichText, Ui, Widget};
use lumen_ui_core::{CodeRecipe, UiThemeExt};

/// An inline code span: monospace text on a subtle `surface_variant` background
/// (no border — unlike [`crate::Kbd`], which represents a physical key).
#[derive(Clone, Debug)]
pub struct Code {
    text: String,
}

impl Code {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for Code {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = CodeRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        Frame::NONE
            .fill(recipe.fill)
            .corner_radius(recipe.corner_radius)
            .inner_margin(Margin::symmetric(
                recipe.inner_margin.x as i8,
                recipe.inner_margin.y as i8,
            ))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(&self.text)
                        .color(recipe.text_color)
                        .size(recipe.text_size)
                        .monospace(),
                );
            })
            .response
    }
}
