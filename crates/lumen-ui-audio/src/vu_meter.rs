//! [`VuMeter`] — a vertical level meter (mono-color by level, or stacked LED zones).

use egui::{lerp, pos2, vec2, Rect, Response, Sense, Stroke, Ui, Widget};
use lumen_ui_core::{MeterRecipe, UiThemeExt};

use crate::{zone_color, ZONE_LOW_MAX, ZONE_MID_MAX};

/// Base width of the meter before density scaling, in points.
const VU_BASE_WIDTH: f32 = 14.0;
/// Base height of the meter before density scaling, in points.
const VU_BASE_HEIGHT: f32 = 120.0;
/// Thickness of the peak-hold line, in points.
const PEAK_LINE: f32 = 2.0;
/// Length of a scale tick, in points.
const TICK_LEN: f32 = 4.0;

/// A vertical VU/peak meter. `level` and the optional peak are fractions of full scale (`0..=1`,
/// clamped) — the caller maps dB to that range. Display-only.
///
/// By default the whole fill is **one color chosen by the current level** (green→amber→red, the
/// software-meter look of the Seno/Dynama displays). [`VuMeter::segmented`] switches to stacked
/// LED-style zones. Tick marks sit at the zone thresholds; an optional peak-hold line is drawn.
#[derive(Clone, Copy, Debug)]
pub struct VuMeter {
    level: f32,
    peak: Option<f32>,
    segmented: bool,
}

impl VuMeter {
    #[must_use]
    pub fn new(level: f32) -> Self {
        Self {
            level: level.clamp(0.0, 1.0),
            peak: None,
            segmented: false,
        }
    }

    /// Add a peak-hold marker at `peak` (fraction of full scale).
    #[must_use]
    pub fn peak(mut self, peak: f32) -> Self {
        self.peak = Some(peak.clamp(0.0, 1.0));
        self
    }

    /// Render stacked LED-style zones (low/mid/high) instead of one color by level.
    #[must_use]
    pub fn segmented(mut self) -> Self {
        self.segmented = true;
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
        let band = |lo: f32, hi: f32| {
            Rect::from_min_max(pos2(rect.left(), y_at(hi)), pos2(rect.right(), y_at(lo)))
        };

        if self.segmented {
            for (lo, hi, color) in [
                (0.0, ZONE_LOW_MAX, recipe.low),
                (ZONE_LOW_MAX, ZONE_MID_MAX, recipe.mid),
                (ZONE_MID_MAX, 1.0, recipe.high),
            ] {
                let top = self.level.min(hi);
                if top > lo {
                    painter.rect_filled(band(lo, top), 0.0, color);
                }
            }
        } else if self.level > 0.0 {
            // One color for the whole fill, chosen by the current level's zone.
            painter.rect_filled(band(0.0, self.level), 0.0, zone_color(self.level, &recipe));
        }

        // Scale ticks at the zone thresholds.
        for threshold in [ZONE_LOW_MAX, ZONE_MID_MAX] {
            let y = y_at(threshold);
            painter.line_segment(
                [
                    pos2(rect.left(), y),
                    pos2(rect.left() + TICK_LEN * scale, y),
                ],
                Stroke::new(1.0, recipe.tick),
            );
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
