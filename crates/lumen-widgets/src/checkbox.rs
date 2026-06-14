//! [`Checkbox`] — a labelled boolean checkbox.

use egui::{pos2, vec2, Response, Sense, Stroke, StrokeKind, Ui, Widget};
use lumen_core::{UiThemeExt, WidgetState};

use crate::text::Label;

/// A checkbox bound to a `&mut bool`, with a trailing label.
#[derive(Debug)]
pub struct Checkbox<'a> {
    checked: &'a mut bool,
    label: String,
}

impl<'a> Checkbox<'a> {
    #[must_use]
    pub fn new(checked: &'a mut bool, label: impl Into<String>) -> Self {
        Self {
            checked,
            label: label.into(),
        }
    }
}

impl Widget for Checkbox<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            let size = ui.spacing().interact_size.y;
            let (rect, mut response) = ui.allocate_exact_size(vec2(size, size), Sense::click());

            if response.clicked() {
                *self.checked = !*self.checked;
                response.mark_changed();
            }

            let state = if !ui.is_enabled() {
                WidgetState::Disabled
            } else if response.hovered() {
                WidgetState::Hovered
            } else {
                WidgetState::Normal
            };

            let theme = ui.theme();
            let recipe = theme.toggle_recipe(*self.checked, state, &ui.ui_ctx());
            let radius = theme.tokens().radius.sm;

            let painter = ui.painter();
            painter.rect_filled(rect, radius, recipe.track);
            if recipe.border.width > 0.0 {
                painter.rect_stroke(rect, radius, recipe.border, StrokeKind::Inside);
            }
            if *self.checked {
                // A check mark drawn as two segments inside the box.
                let stroke = Stroke::new(2.0, recipe.knob);
                let p1 = pos2(rect.left() + size * 0.25, rect.center().y);
                let p2 = pos2(rect.left() + size * 0.45, rect.bottom() - size * 0.28);
                let p3 = pos2(rect.right() - size * 0.22, rect.top() + size * 0.28);
                painter.line_segment([p1, p2], stroke);
                painter.line_segment([p2, p3], stroke);
            }

            response | ui.add(Label::new(self.label))
        })
        .inner
    }
}
