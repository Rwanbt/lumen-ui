//! [`Knob`] — a rotary value control, the staple of audio/DAW UIs.

use std::f32::consts::PI;
use std::ops::RangeInclusive;

use egui::{vec2, Pos2, Response, Sense, Shape, Stroke, Ui, Widget, WidgetInfo};
use lumen_ui_core::{KnobRecipe, UiThemeExt};

/// Angle (radians) where the value arc starts: lower-left (135°), measured with egui's y-down axis.
const START: f32 = 0.75 * PI;
/// Total sweep of the arc (270°), clockwise over the top to the lower-right — gap at the bottom.
const SWEEP: f32 = 1.5 * PI;
/// Range fraction moved per point of vertical drag.
const DRAG_SENSITIVITY: f32 = 0.005;
/// Arc radius as a fraction of the knob diameter.
const RADIUS_RATIO: f32 = 0.40;
/// Arc stroke width as a fraction of the knob diameter.
const ARC_WIDTH_RATIO: f32 = 0.10;
/// Segments used to tessellate the arc.
const ARC_SEGMENTS: usize = 48;

/// A rotary knob bound to a `&mut f32` over an inclusive range. Drag **vertically** to change the
/// value (up increases). The 270° arc fills from the minimum and a pointer marks the value.
#[derive(Debug)]
pub struct Knob<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
}

impl<'a> Knob<'a> {
    #[must_use]
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self { value, range }
    }
}

/// Tessellate an arc centered at `center` of `radius`, from `start` spanning `sweep` radians.
fn arc_points(center: Pos2, radius: f32, start: f32, sweep: f32) -> Vec<Pos2> {
    (0..=ARC_SEGMENTS)
        .map(|i| {
            let angle = start + sweep * (i as f32 / ARC_SEGMENTS as f32);
            center + vec2(angle.cos(), angle.sin()) * radius
        })
        .collect()
}

impl Widget for Knob<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = KnobRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let size = recipe.size;
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(size, size), Sense::click_and_drag());

        let (min, max) = (*self.range.start(), *self.range.end());
        let span = (max - min).max(f32::EPSILON);

        if response.dragged() {
            // Vertical drag: dragging up (negative y) increases the value.
            let delta = -response.drag_delta().y * DRAG_SENSITIVITY * span;
            *self.value = (*self.value + delta).clamp(min, max);
            response.mark_changed();
        }

        let t = ((*self.value - min) / span).clamp(0.0, 1.0);
        let center = rect.center();
        let radius = size * RADIUS_RATIO;
        let stroke_width = size * ARC_WIDTH_RATIO;
        let painter = ui.painter();

        painter.add(Shape::line(
            arc_points(center, radius, START, SWEEP),
            Stroke::new(stroke_width, recipe.track),
        ));
        if t > 0.0 {
            painter.add(Shape::line(
                arc_points(center, radius, START, SWEEP * t),
                Stroke::new(stroke_width, recipe.fill),
            ));
        }
        let angle = START + SWEEP * t;
        let tip = center + vec2(angle.cos(), angle.sin()) * radius;
        painter.line_segment(
            [center, tip],
            Stroke::new(stroke_width * 0.6, recipe.indicator),
        );

        let (enabled, value) = (ui.is_enabled(), *self.value);
        response.widget_info(|| WidgetInfo::slider(enabled, f64::from(value), ""));
        response
    }
}
