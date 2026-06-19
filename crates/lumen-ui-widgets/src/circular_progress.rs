//! [`CircularProgress`] — a determinate progress ring themed by tokens.

use std::f32::consts::{PI, TAU};

use egui::{vec2, Pos2, Response, Sense, Shape, Stroke, Ui, Widget};
use lumen_ui_core::{CircularProgressRecipe, UiThemeExt};

use crate::util::sanitize_fraction;

/// Segments used to approximate a full-circle arc; the drawn arc uses a fraction.
const ARC_SEGMENTS: usize = 48;

/// A circular progress ring for a known fraction in `[0, 1]`. The track is the
/// full ring; the filled arc sweeps clockwise from the top (12 o'clock).
#[derive(Clone, Copy, Debug)]
pub struct CircularProgress {
    fraction: f32,
}

impl CircularProgress {
    /// `fraction` is clamped to `[0, 1]`; non-finite input (NaN/∞) becomes `0.0`.
    #[must_use]
    pub fn new(fraction: f32) -> Self {
        Self {
            fraction: sanitize_fraction(fraction),
        }
    }
}

/// Points along a circular arc of `sweep` radians from `start`, at `radius` around `center`.
fn arc_points(center: Pos2, radius: f32, start: f32, sweep: f32, steps: usize) -> Vec<Pos2> {
    (0..=steps)
        .map(|i| {
            let angle = start + (i as f32 / steps as f32) * sweep;
            center + radius * vec2(angle.cos(), angle.sin())
        })
        .collect()
}

impl Widget for CircularProgress {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = CircularProgressRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let (rect, response) =
            ui.allocate_exact_size(vec2(recipe.diameter, recipe.diameter), Sense::hover());
        let center = rect.center();
        let radius = recipe.diameter / 2.0 - recipe.thickness / 2.0;
        let start = -PI / 2.0; // 12 o'clock
        let painter = ui.painter();
        // Track and fill must use the SAME primitive: `circle_stroke` and `Shape::line` tessellate
        // at slightly different effective radii, which made the two rings look off-centre. So the
        // track is drawn as a full-sweep polyline, identical to the fill arc.
        painter.add(Shape::line(
            arc_points(center, radius, start, TAU, ARC_SEGMENTS),
            Stroke::new(recipe.thickness, recipe.track),
        ));
        if self.fraction > 0.0 {
            let steps = ((ARC_SEGMENTS as f32 * self.fraction).ceil() as usize).max(2);
            painter.add(Shape::line(
                arc_points(center, radius, start, self.fraction * TAU, steps),
                Stroke::new(recipe.thickness, recipe.fill),
            ));
        }
        response
    }
}
