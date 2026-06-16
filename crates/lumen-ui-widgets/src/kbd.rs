//! [`Kbd`] — a keyboard-key indicator (e.g. `Ctrl`, `⌘`).

use egui::{Frame, Response, RichText, Ui, Widget};
use lumen_ui_core::{KbdRecipe, UiThemeExt};

/// A small bordered, monospace key cap. Colors/border come from [`KbdRecipe`].
#[derive(Clone, Debug)]
pub struct Kbd {
    key: String,
}

impl Kbd {
    #[must_use]
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}

impl Widget for Kbd {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = KbdRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        Frame::NONE
            .fill(recipe.fill)
            .stroke(recipe.border)
            .corner_radius(recipe.corner_radius)
            .inner_margin(crate::util::margin(recipe.inner_margin))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(&self.key)
                        .color(recipe.text_color)
                        .size(recipe.text_size)
                        .monospace(),
                );
            })
            .response
    }
}
