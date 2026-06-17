//! [`XyPad`] — a 2-D control binding two values to a draggable point.

use std::ops::RangeInclusive;

use egui::{lerp, pos2, vec2, Response, Sense, Stroke, Ui, Widget, WidgetInfo};
use lumen_ui_core::{UiThemeExt, XyPadRecipe};

/// Radius of the draggable point as a fraction of the pad size.
const POINT_RATIO: f32 = 0.05;

/// A square 2-D pad binding `(x, y)` `&mut f32` to a draggable point, each over its own inclusive
/// range. The X axis runs left→right, the Y axis bottom→top. Click or drag to set both at once.
#[derive(Debug)]
pub struct XyPad<'a> {
    x: &'a mut f32,
    y: &'a mut f32,
    x_range: RangeInclusive<f32>,
    y_range: RangeInclusive<f32>,
}

impl<'a> XyPad<'a> {
    #[must_use]
    pub fn new(
        x: &'a mut f32,
        y: &'a mut f32,
        x_range: RangeInclusive<f32>,
        y_range: RangeInclusive<f32>,
    ) -> Self {
        Self {
            x,
            y,
            x_range,
            y_range,
        }
    }
}

impl Widget for XyPad<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = XyPadRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let size = recipe.size;
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(size, size), Sense::click_and_drag());

        let (x_min, x_max) = (*self.x_range.start(), *self.x_range.end());
        let (y_min, y_max) = (*self.y_range.start(), *self.y_range.end());
        let x_span = (x_max - x_min).max(f32::EPSILON);
        let y_span = (y_max - y_min).max(f32::EPSILON);
        let point_radius = size * POINT_RATIO;
        let inner = rect.shrink(point_radius);

        if let Some(pos) = response.interact_pointer_pos() {
            let tx = ((pos.x - inner.left()) / inner.width()).clamp(0.0, 1.0);
            // Screen y is top-down; the pad's Y axis is bottom-up.
            let ty = ((inner.bottom() - pos.y) / inner.height()).clamp(0.0, 1.0);
            *self.x = x_min + tx * x_span;
            *self.y = y_min + ty * y_span;
            response.mark_changed();
        }

        let tx = ((*self.x - x_min) / x_span).clamp(0.0, 1.0);
        let ty = ((*self.y - y_min) / y_span).clamp(0.0, 1.0);
        let px = lerp(inner.left()..=inner.right(), tx);
        let py = lerp(inner.bottom()..=inner.top(), ty);

        let painter = ui.painter();
        painter.rect_filled(rect, 4.0, recipe.background);
        let guide = Stroke::new(1.0, recipe.guide);
        painter.line_segment([pos2(rect.left(), py), pos2(rect.right(), py)], guide);
        painter.line_segment([pos2(px, rect.top()), pos2(px, rect.bottom())], guide);
        painter.circle_filled(pos2(px, py), point_radius, recipe.point);

        let (enabled, value) = (ui.is_enabled(), *self.x);
        response.widget_info(|| WidgetInfo::slider(enabled, f64::from(value), ""));
        response
    }
}
