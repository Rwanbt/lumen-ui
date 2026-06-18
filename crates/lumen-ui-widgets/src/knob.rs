//! [`Knob`] — a rotary value control, the staple of audio/DAW UIs.

use std::f32::consts::PI;
use std::ops::RangeInclusive;

use egui::{vec2, Color32, Pos2, Response, Sense, Shape, Stroke, Ui, Widget, WidgetInfo};
use lumen_ui_core::{KnobRecipe, UiThemeExt};

/// Angle (radians) where the value arc starts: lower-left (135°), measured with egui's y-down axis.
const START: f32 = 0.75 * PI;
/// Total sweep of the arc (270°), clockwise over the top to the lower-right — gap at the bottom.
const SWEEP: f32 = 1.5 * PI;
/// Range fraction moved per point of vertical drag.
const DRAG_SENSITIVITY: f32 = 0.005;
/// Value-arc radius as a fraction of the knob diameter.
const ARC_RATIO: f32 = 0.42;
/// Body-disc radius as a fraction of the knob diameter.
const DISC_RATIO: f32 = 0.36;
/// Track / value arc stroke width, in points.
const ARC_WIDTH: f32 = 2.0;
/// Pointer-tick length as a fraction of the disc radius (from the rim inward).
const TICK_RATIO: f32 = 0.55;
/// Amount the disc lightens while being dragged.
const ACTIVE_LIGHTEN: f32 = 0.10;
/// Segments used to tessellate the arc.
const ARC_SEGMENTS: usize = 40;

/// A rotary knob bound to a `&mut f32` over an inclusive range. Drag **vertically** to change the
/// value (up increases). A recessed 270° track arc fills from the minimum; a body disc with a ring
/// (the ring lights to the accent while hovered/dragged) carries a pointer tick at its rim.
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

/// Blend `color` toward white by `t` (`0..=1`) — a cheap "lighten on interaction" for the disc.
fn lighten(color: Color32, t: f32) -> Color32 {
    let f = |c: u8| (f32::from(c) + (255.0 - f32::from(c)) * t).round() as u8;
    Color32::from_rgb(f(color.r()), f(color.g()), f(color.b()))
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

        let active = response.dragged();
        let hot = active || response.hovered();
        let t = ((*self.value - min) / span).clamp(0.0, 1.0);
        let center = rect.center();
        let arc_r = size * ARC_RATIO;
        let disc_r = size * DISC_RATIO;
        let painter = ui.painter();

        // Body disc (lightens while dragged) + ring (lights to the accent while hovered/dragged).
        let disc = if active {
            lighten(recipe.disc, ACTIVE_LIGHTEN)
        } else {
            recipe.disc
        };
        let ring = if hot { recipe.fill } else { recipe.ring };
        painter.circle_filled(center, disc_r, disc);
        painter.circle_stroke(center, disc_r, Stroke::new(1.0, ring));

        // Recessed track arc, then the filled value arc on top.
        painter.add(Shape::line(
            arc_points(center, arc_r, START, SWEEP),
            Stroke::new(ARC_WIDTH, recipe.track),
        ));
        if t > 0.0 {
            painter.add(Shape::line(
                arc_points(center, arc_r, START, SWEEP * t),
                Stroke::new(ARC_WIDTH, recipe.fill),
            ));
        }

        // Pointer tick at the rim, pointing to the value.
        let dir = {
            let angle = START + SWEEP * t;
            vec2(angle.cos(), angle.sin())
        };
        painter.line_segment(
            [
                center + dir * (disc_r * (1.0 - TICK_RATIO)),
                center + dir * disc_r,
            ],
            Stroke::new(ARC_WIDTH, recipe.indicator),
        );

        let (enabled, value) = (ui.is_enabled(), *self.value);
        response.widget_info(|| WidgetInfo::slider(enabled, f64::from(value), ""));
        response
    }
}
