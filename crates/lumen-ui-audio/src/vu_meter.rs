//! [`VuMeter`] — a vertical level meter with green/amber/red zones and a peak-hold marker.

use egui::{lerp, pos2, vec2, Rect, Response, Sense, Ui, Widget};
use lumen_ui_core::{MeterRecipe, UiThemeExt};

use crate::{ZONE_LOW_MAX, ZONE_MID_MAX};

/// Base width of the meter before density scaling, in points.
const VU_BASE_WIDTH: f32 = 14.0;
/// Base height of the meter before density scaling, in points.
const VU_BASE_HEIGHT: f32 = 120.0;
/// Thickness of the peak-hold line, in points.
const PEAK_LINE: f32 = 2.0;

/// A vertical VU/peak meter. `level` and the optional peak are fractions of full scale (`0..=1`,
/// clamped) — the caller maps dB to that range. Display-only (no interaction); the filled portion
/// is split into low/mid/high colored zones and an optional peak-hold line is drawn across.
#[derive(Clone, Copy, Debug)]
pub struct VuMeter {
    level: f32,
    peak: Option<f32>,
}

impl VuMeter {
    #[must_use]
    pub fn new(level: f32) -> Self {
        Self {
            level: level.clamp(0.0, 1.0),
            peak: None,
        }
    }

    /// Add a peak-hold marker at `peak` (fraction of full scale).
    #[must_use]
    pub fn peak(mut self, peak: f32) -> Self {
        self.peak = Some(peak.clamp(0.0, 1.0));
        self
    }
}

impl Widget for VuMeter {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = MeterRecipe::resolve(ui.theme().tokens());
        let scale = ui.ui_ctx().density_scale();
        let (rect, response) = ui.allocate_exact_size(
            vec2(VU_BASE_WIDTH * scale, VU_BASE_HEIGHT * scale),
            Sense::hover(),
        );

        let painter = ui.painter();
        painter.rect_filled(rect, 2.0, recipe.track);

        // Bottom = 0, top = full scale.
        let y_at = |t: f32| lerp(rect.bottom()..=rect.top(), t.clamp(0.0, 1.0));
        let zones = [
            (0.0, ZONE_LOW_MAX, recipe.low),
            (ZONE_LOW_MAX, ZONE_MID_MAX, recipe.mid),
            (ZONE_MID_MAX, 1.0, recipe.high),
        ];
        for (lo, hi, color) in zones {
            let top = self.level.min(hi);
            if top > lo {
                painter.rect_filled(
                    Rect::from_min_max(pos2(rect.left(), y_at(top)), pos2(rect.right(), y_at(lo))),
                    0.0,
                    color,
                );
            }
        }

        if let Some(peak) = self.peak {
            let y = y_at(peak);
            painter.rect_filled(
                Rect::from_min_max(
                    pos2(rect.left(), y - PEAK_LINE / 2.0),
                    pos2(rect.right(), y + PEAK_LINE / 2.0),
                ),
                0.0,
                recipe.peak,
            );
        }
        response
    }
}
