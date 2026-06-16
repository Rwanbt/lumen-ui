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

impl Widget for CircularProgress {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = CircularProgressRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let (rect, response) =
            ui.allocate_exact_size(vec2(recipe.diameter, recipe.diameter), Sense::hover());
        let center = rect.center();
        let radius = recipe.diameter / 2.0 - recipe.thickness / 2.0;
        let painter = ui.painter();
        painter.circle_stroke(center, radius, Stroke::new(recipe.thickness, recipe.track));
        if self.fraction > 0.0 {
            let start = -PI / 2.0;
            let end = start + self.fraction * TAU;
            let steps = ((ARC_SEGMENTS as f32 * self.fraction).ceil() as usize).max(2);
            let points: Vec<Pos2> = (0..=steps)
                .map(|i| {
                    let angle = start + (i as f32 / steps as f32) * (end - start);
                    center + radius * vec2(angle.cos(), angle.sin())
                })
                .collect();
            painter.add(Shape::line(
                points,
                Stroke::new(recipe.thickness, recipe.fill),
            ));
        }
        response
    }
}
