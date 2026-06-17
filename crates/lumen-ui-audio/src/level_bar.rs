//! [`LevelBar`] — a horizontal level indicator whose fill is colored by zone.

use egui::{pos2, vec2, Rect, Response, Sense, Ui, Widget};
use lumen_ui_core::{MeterRecipe, UiThemeExt};

use crate::zone_color;

/// Base width of the bar before density scaling, in points.
const LEVEL_BASE_WIDTH: f32 = 120.0;
/// Base height of the bar before density scaling, in points.
const LEVEL_BASE_HEIGHT: f32 = 10.0;

/// A horizontal level bar. `level` is a fraction of full scale (`0..=1`, clamped); the fill grows
/// left→right and is colored by the zone the level falls in (low/mid/high). Display-only.
#[derive(Clone, Copy, Debug)]
pub struct LevelBar {
    level: f32,
}

impl LevelBar {
    #[must_use]
    pub fn new(level: f32) -> Self {
        Self {
            level: level.clamp(0.0, 1.0),
        }
    }
}

impl Widget for LevelBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = MeterRecipe::resolve(ui.theme().tokens());
        let scale = ui.ui_ctx().density_scale();
        let (rect, response) = ui.allocate_exact_size(
            vec2(LEVEL_BASE_WIDTH * scale, LEVEL_BASE_HEIGHT * scale),
            Sense::hover(),
        );
        let radius = rect.height() / 2.0;

        let painter = ui.painter();
        painter.rect_filled(rect, radius, recipe.track);
        if self.level > 0.0 {
            let fill = Rect::from_min_max(
                rect.min,
                pos2(rect.left() + rect.width() * self.level, rect.bottom()),
            );
            painter.rect_filled(fill, radius, zone_color(self.level, &recipe));
        }
        response
    }
}
