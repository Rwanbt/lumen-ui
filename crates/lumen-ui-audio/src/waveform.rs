//! [`Waveform`] — a filled min/max envelope display of an audio sample buffer.

use egui::epaint::Mesh;
use egui::{pos2, vec2, Response, Sense, Shape, Stroke, Ui, Widget};
use lumen_ui_core::{UiThemeExt, WaveformRecipe};

/// Base width of the waveform before density scaling, in points.
const WAVE_BASE_WIDTH: f32 = 200.0;
/// Base height of the waveform before density scaling, in points.
const WAVE_BASE_HEIGHT: f32 = 64.0;
/// Envelope outline stroke width, in points.
const WAVE_STROKE: f32 = 1.0;

/// A waveform display of a `&[f32]` sample buffer (samples in `-1.0..=1.0`). Renders the per-column
/// min/max **envelope** as a filled body plus a top/bottom outline (the house style of the Seno /
/// Dynama / Spectra signal displays), staying correct for buffers far larger than the pixel width.
///
/// Two looks: **symmetric** (default — `|amplitude|` mirrored around the midline, the classic DAW
/// look) or **signed** ([`Waveform::signed`] — the true min/max envelope). Display-only.
#[derive(Clone, Copy, Debug)]
pub struct Waveform<'a> {
    samples: &'a [f32],
    signed: bool,
}

impl<'a> Waveform<'a> {
    #[must_use]
    pub fn new(samples: &'a [f32]) -> Self {
        Self {
            samples,
            signed: false,
        }
    }

    /// Draw the true signed min/max envelope instead of the symmetric `|amplitude|` look.
    #[must_use]
    pub fn signed(mut self) -> Self {
        self.signed = true;
        self
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

        if self.samples.is_empty() {
            return response;
        }

        // One (top, bottom) envelope point per pixel column.
        let columns = rect.width().max(1.0) as usize;
        let mut top = Vec::with_capacity(columns + 1);
        let mut bot = Vec::with_capacity(columns + 1);
        for column in 0..=columns {
            let start = column * self.samples.len() / (columns + 1);
            let end = ((column + 1) * self.samples.len() / (columns + 1)).max(start + 1);
            let mut lo = f32::INFINITY;
            let mut hi = f32::NEG_INFINITY;
            for &s in &self.samples[start..end.min(self.samples.len())] {
                lo = lo.min(s);
                hi = hi.max(s);
            }
            if lo > hi {
                lo = 0.0;
                hi = 0.0;
            }
            let x = rect.left() + column as f32;
            let (top_y, bot_y) = if self.signed {
                (
                    mid_y - hi.clamp(-1.0, 1.0) * half_h,
                    mid_y - lo.clamp(-1.0, 1.0) * half_h,
                )
            } else {
                let amp = hi.abs().max(lo.abs()).clamp(0.0, 1.0) * half_h;
                (mid_y - amp, mid_y + amp)
            };
            top.push(pos2(x, top_y));
            bot.push(pos2(x, bot_y));
        }

        // Filled body: a mesh strip between the top and bottom envelopes (shared vertices, no seams).
        let mut mesh = Mesh::default();
        for i in 0..top.len() - 1 {
            let base = mesh.vertices.len() as u32;
            mesh.colored_vertex(top[i], recipe.fill);
            mesh.colored_vertex(top[i + 1], recipe.fill);
            mesh.colored_vertex(bot[i + 1], recipe.fill);
            mesh.colored_vertex(bot[i], recipe.fill);
            mesh.add_triangle(base, base + 1, base + 2);
            mesh.add_triangle(base, base + 2, base + 3);
        }
        painter.add(Shape::mesh(mesh));
        painter.add(Shape::line(top, Stroke::new(WAVE_STROKE, recipe.wave)));
        painter.add(Shape::line(bot, Stroke::new(WAVE_STROKE, recipe.wave)));

        response
    }
}
