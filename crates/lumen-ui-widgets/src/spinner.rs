//! [`Spinner`] — an indeterminate loading indicator themed by the primary color.

use egui::{Response, Ui, Widget};
use lumen_ui_core::{SpinnerRecipe, UiThemeExt};

/// An indeterminate spinner. Reads [`SpinnerRecipe`] from the installed theme,
/// so its color follows the theme's primary and its size follows density.
#[derive(Clone, Copy, Debug, Default)]
pub struct Spinner {
    size: Option<f32>,
}

impl Spinner {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Override the resolved diameter (points).
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }
}

impl Widget for Spinner {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = SpinnerRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let size = self.size.unwrap_or(recipe.size);
        egui::Spinner::new().size(size).color(recipe.color).ui(ui)
    }
}
