//! [`TextField`] — a themed single-line text input.

use egui::{Frame, Response, TextEdit, Ui, Widget};
use lumen_ui_core::{UiThemeExt, WidgetState};

/// A single-line text input bound to a `&mut String`. The border highlights when
/// focused (read from the previous frame, like [`crate::Button`]).
#[derive(Debug)]
pub struct TextField<'a> {
    text: &'a mut String,
    hint: String,
    password: bool,
}

impl<'a> TextField<'a> {
    #[must_use]
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            hint: String::new(),
            password: false,
        }
    }

    #[must_use]
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = hint.into();
        self
    }

    #[must_use]
    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }
}

impl Widget for TextField<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let id = ui.next_auto_id();
        let was_focused = ui.ctx().read_response(id).is_some_and(|r| r.has_focus());
        let state = if !ui.is_enabled() {
            WidgetState::Disabled
        } else if was_focused {
            WidgetState::Focused
        } else {
            WidgetState::Normal
        };

        let recipe = ui.theme().text_field_recipe(state, &ui.ui_ctx());
        Frame::NONE
            .fill(recipe.fill)
            .stroke(recipe.border)
            .corner_radius(recipe.corner_radius)
            .inner_margin(crate::util::margin(recipe.inner_margin))
            .show(ui, |ui| {
                ui.add(
                    TextEdit::singleline(self.text)
                        .id(id)
                        .frame(Frame::NONE)
                        .hint_text(self.hint.as_str())
                        .text_color(recipe.text_color)
                        .password(self.password)
                        .desired_width(f32::INFINITY),
                )
            })
            .inner
    }
}
