//! [`Textarea`] — a themed multi-line text input (mirrors [`crate::TextField`]).

use egui::{Frame, Response, TextEdit, Ui, Widget};
use lumen_ui_core::{UiThemeExt, WidgetState};

/// Default visible rows before scrolling.
const DEFAULT_ROWS: usize = 4;

/// A multi-line text input bound to a `&mut String`. Reuses the theme's
/// [`lumen_ui_core::TextFieldRecipe`]; the border highlights when focused
/// (read from the previous frame, like [`crate::TextField`]).
#[derive(Debug)]
pub struct Textarea<'a> {
    text: &'a mut String,
    hint: String,
    rows: usize,
}

impl<'a> Textarea<'a> {
    #[must_use]
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            hint: String::new(),
            rows: DEFAULT_ROWS,
        }
    }

    #[must_use]
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = hint.into();
        self
    }

    #[must_use]
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }
}

impl Widget for Textarea<'_> {
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
                    TextEdit::multiline(self.text)
                        .id(id)
                        .frame(Frame::NONE)
                        .hint_text(self.hint.as_str())
                        .text_color(recipe.text_color)
                        .desired_rows(self.rows)
                        .desired_width(f32::INFINITY),
                )
            })
            .inner
    }
}
