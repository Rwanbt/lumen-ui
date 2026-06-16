//! [`FormField`] — a label + control + hint/error wrapper for forms.

use egui::{Response, RichText, Ui};
use lumen_ui_core::{FormFieldRecipe, UiThemeExt};

/// Wraps a control with a label above and an optional hint or error below. When
/// an error is set it replaces the hint and is colored with the danger token.
///
/// ```ignore
/// FormField::new("Email")
///     .hint("We'll never share it")
///     .show(ui, |ui| ui.add(TextField::new(&mut email)));
/// ```
#[derive(Clone, Debug)]
pub struct FormField {
    label: String,
    hint: Option<String>,
    error: Option<String>,
}

impl FormField {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            hint: None,
            error: None,
        }
    }

    #[must_use]
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    #[must_use]
    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response {
        let recipe = FormFieldRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        ui.vertical(|ui| {
            ui.label(
                RichText::new(&self.label)
                    .color(recipe.label_color)
                    .size(recipe.label_size)
                    .strong(),
            );
            ui.add_space(recipe.gap);
            content(ui);
            if let Some(error) = &self.error {
                ui.add_space(recipe.gap);
                ui.label(
                    RichText::new(error)
                        .color(recipe.error_color)
                        .size(recipe.hint_size),
                );
            } else if let Some(hint) = &self.hint {
                ui.add_space(recipe.gap);
                ui.label(
                    RichText::new(hint)
                        .color(recipe.hint_color)
                        .size(recipe.hint_size),
                );
            }
        })
        .response
    }
}
