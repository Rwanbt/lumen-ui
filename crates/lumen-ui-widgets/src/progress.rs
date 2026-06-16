//! [`Progress`] — a linear determinate progress bar themed by tokens.

use egui::{vec2, Rect, Response, Sense, Ui, Widget};
use lumen_ui_core::{ProgressRecipe, UiThemeExt};

use crate::util::sanitize_fraction;

/// A linear progress bar for a known fraction in `[0, 1]`. Track and fill colors
/// come from the theme via [`ProgressRecipe`]; it spans the available width.
#[derive(Clone, Copy, Debug)]
pub struct Progress {
    fraction: f32,
}

impl Progress {
    /// `fraction` is clamped to `[0, 1]`; non-finite input (NaN/∞) becomes `0.0`
    /// (`f32::clamp` would otherwise pass NaN through).
    #[must_use]
    pub fn new(fraction: f32) -> Self {
        Self {
            fraction: sanitize_fraction(fraction),
        }
    }
}

impl Widget for Progress {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = ProgressRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let width = ui.available_width();
        let (rect, response) = ui.allocate_exact_size(vec2(width, recipe.height), Sense::hover());
        let painter = ui.painter();
        painter.rect_filled(rect, recipe.corner_radius, recipe.track);
        if self.fraction > 0.0 {
            let fill_rect =
                Rect::from_min_size(rect.min, vec2(rect.width() * self.fraction, rect.height()));
            painter.rect_filled(fill_rect, recipe.corner_radius, recipe.fill);
        }
        response
    }
}
