//! [`Switch`] — an animated on/off toggle.

use egui::{vec2, CornerRadius, Response, Sense, StrokeKind, Ui, Widget, WidgetInfo, WidgetType};
use lumen_core::{UiThemeExt, WidgetState};

use crate::focus::focus_ring;

/// A pill-shaped toggle bound to a `&mut bool`. The knob slides with the v0.2
/// minimal motion (`animate_bool_with_time`).
#[derive(Debug)]
pub struct Switch<'a> {
    on: &'a mut bool,
}

impl<'a> Switch<'a> {
    #[must_use]
    pub fn new(on: &'a mut bool) -> Self {
        Self { on }
    }
}

impl Widget for Switch<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        // a11y (v0.8): the hit target follows the density (44 px in Touch — WCAG 2.5.5).
        let height = ui.ui_ctx().min_interactive_size();
        let width = height * 1.8;
        let (rect, mut response) = ui.allocate_exact_size(vec2(width, height), Sense::click());

        if response.clicked() {
            *self.on = !*self.on;
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
        let recipe = theme.toggle_recipe(*self.on, state, &ui.ui_ctx());
        let how_on =
            ui.ctx()
                .animate_bool_with_time(response.id, *self.on, theme.tokens().motion.base);

        let radius = height / 2.0;
        let painter = ui.painter();
        painter.rect_filled(rect, radius, recipe.track);
        if recipe.border.width > 0.0 {
            painter.rect_stroke(rect, radius, recipe.border, StrokeKind::Inside);
        }
        let knob_radius = radius - 2.0;
        let cx = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        painter.circle_filled(egui::pos2(cx, rect.center().y), knob_radius, recipe.knob);

        // a11y: expose toggle state to screen readers / AccessKit (and to egui_kittest).
        let (enabled, on) = (ui.is_enabled(), *self.on);
        response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, enabled, on, ""));

        // Focus-visible (keyboard nav): pill-shaped ring matching the track.
        focus_ring(
            ui,
            &response,
            CornerRadius::same(radius as u8),
            theme.tokens().colors.primary,
        );
        response
    }
}
