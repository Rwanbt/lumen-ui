//! [`EmptyState`] — a centered placeholder for "no content yet" views.

use egui::{Response, RichText, Ui, Widget};
use lumen_ui_core::{EmptyStateRecipe, UiThemeExt};

/// A centered empty-state block: a title and an optional explanatory message.
/// Place an action (e.g. a [`crate::Button`]) below it in the calling UI.
#[derive(Clone, Debug)]
pub struct EmptyState {
    title: String,
    message: Option<String>,
}

impl EmptyState {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: None,
        }
    }

    #[must_use]
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl Widget for EmptyState {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = EmptyStateRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(&self.title)
                    .color(recipe.title_color)
                    .size(recipe.title_size)
                    .strong(),
            );
            if let Some(message) = &self.message {
                ui.add_space(recipe.gap);
                ui.label(
                    RichText::new(message)
                        .color(recipe.message_color)
                        .size(recipe.message_size),
                );
            }
        })
        .response
    }
}
