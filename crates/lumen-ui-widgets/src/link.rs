//! [`Link`] — a themed inline text link (clickable, primary-colored).

use egui::{Response, RichText, Ui, Widget};
use lumen_ui_core::{LinkRecipe, UiThemeExt};

/// A clickable text link colored by the theme's `primary` token. Inspect the
/// returned [`Response::clicked`] to react.
#[derive(Clone, Debug)]
pub struct Link {
    text: String,
}

impl Link {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for Link {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = LinkRecipe::resolve(ui.theme().tokens());
        ui.add(egui::Link::new(
            RichText::new(&self.text)
                .color(recipe.color)
                .size(recipe.text_size),
        ))
    }
}
