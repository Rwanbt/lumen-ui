//! [`Form`] — a vertical stack of fields plus an actions footer.
//!
//! Composes any field controls (typically [`lumen_ui_widgets::FormField`]) into a
//! consistently spaced column, with an optional row of actions (submit/cancel) below.
//! Spacing between rows is taken from the installed theme's tokens, so a form restyles
//! with the theme like every other lumen-ui surface.

use egui::{Response, Ui};
use lumen_ui_core::UiThemeExt;

type Section<'a> = Box<dyn FnOnce(&mut Ui) + 'a>;

/// A form layout: stack [`Form::field`] rows, then an optional [`Form::actions`] footer.
///
/// ```ignore
/// Form::new()
///     .field(|ui| { FormField::new("Email").show(ui, |ui| { ui.add(TextField::new(&mut email)); }); })
///     .field(|ui| { FormField::new("Password").show(ui, |ui| { ui.add(TextField::new(&mut pw).password(true)); }); })
///     .actions(|ui| { if ui.add(Button::primary("Save")).clicked() { /* submit */ } })
///     .show(ui);
/// ```
#[derive(Default)]
pub struct Form<'a> {
    fields: Vec<Section<'a>>,
    actions: Option<Section<'a>>,
}

impl<'a> Form<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append one field row. Rows are separated by a tokenized gap.
    #[must_use]
    pub fn field(mut self, content: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.fields.push(Box::new(content));
        self
    }

    /// Set the actions footer (rendered in a horizontal row below the fields).
    #[must_use]
    pub fn actions(mut self, content: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.actions = Some(Box::new(content));
        self
    }

    /// Draw the form. Returns the response of the enclosing vertical layout.
    pub fn show(self, ui: &mut Ui) -> Response {
        let gap = ui.theme().tokens().spacing.md;
        ui.vertical(|ui| {
            for (index, field) in self.fields.into_iter().enumerate() {
                if index > 0 {
                    ui.add_space(gap);
                }
                field(ui);
            }
            if let Some(actions) = self.actions {
                ui.add_space(gap);
                ui.horizontal(|ui| actions(ui));
            }
        })
        .response
    }
}
