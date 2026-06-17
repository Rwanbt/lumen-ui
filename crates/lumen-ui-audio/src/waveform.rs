//! [`Waveform`] — a min/max envelope display of an audio sample buffer.

use egui::{pos2, vec2, Response, Sense, Stroke, Ui, Widget};
use lumen_ui_core::{UiThemeExt, WaveformRecipe};

/// Base width of the waveform before density scaling, in points.
const WAVE_BASE_WIDTH: f32 = 200.0;
/// Base height of the waveform before density scaling, in points.
const WAVE_BASE_HEIGHT: f32 = 64.0;
/// Envelope stroke width, in points.
const WAVE_STROKE: f32 = 1.0;

/// A waveform display of a `&[f32]` sample buffer (samples in `-1.0..=1.0`). Renders the per-column
/// min/max **envelope**, so it stays correct for buffers far larger than the pixel width.
/// Display-only.
#[derive(Clone, Copy, Debug)]
pub struct Waveform<'a> {
    samples: &'a [f32],
}

impl<'a> Waveform<'a> {
    #[must_use]
    pub fn new(samples: &'a [f32]) -> Self {
        Self { samples }
    }
}

impl Widget for Waveform<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = WaveformRecipe::resolve(ui.theme().tokens());
        let scale = ui.ui_ctx().density_scale();
        let (rect, response) = ui.allocate_exact_size(
            vec2(WAVE_BASE_WIDTH * scale, WAVE_BASE_HEIGHT * scale),
            Sense::hover(),
        );

        let painter = ui.painter();
        painter.rect_filled(rect, 2.0, recipe.background);
        let mid_y = rect.center().y;
        let half_h = rect.height() / 2.0;
        painter.line_segment(
            [pos2(rect.left(), mid_y), pos2(rect.right(), mid_y)],
            Stroke::new(WAVE_STROKE, recipe.baseline),
        );

        let columns = rect.width().max(1.0) as usize;
        if !self.samples.is_empty() {
            let stroke = Stroke::new(WAVE_STROKE, recipe.wave);
            for column in 0..columns {
                // Samples mapped to this pixel column.
                let start = column * self.samples.len() / columns;
                let end = ((column + 1) * self.samples.len() / columns).max(start + 1);
                let mut lo = f32::INFINITY;
                let mut hi = f32::NEG_INFINITY;
                for &s in &self.samples[start..end.min(self.samples.len())] {
                    lo = lo.min(s);
                    hi = hi.max(s);
                }
                if lo <= hi {
                    let x = rect.left() + column as f32;
                    let y_hi = mid_y - hi.clamp(-1.0, 1.0) * half_h;
                    let y_lo = mid_y - lo.clamp(-1.0, 1.0) * half_h;
                    painter.line_segment([pos2(x, y_hi), pos2(x, y_lo)], stroke);
                }
            }
        }
        response
    }
}
