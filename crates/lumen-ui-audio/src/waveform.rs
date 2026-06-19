//! [`Waveform`] — an interactive, zoomable waveform display of an audio sample buffer.
//!
//! Two representations, chosen automatically by zoom level (the Ableton / Pro Tools model):
//! - **zoomed out** (many samples per pixel) → a filled min/max **envelope** overview;
//! - **zoomed in** (≤ ~1 sample per pixel) → the **precise per-sample line**, so a sine reads as a
//!   true continuous sinusoid rather than an amplitude blob.
//!
//! Scroll to zoom (centred on the cursor), drag to pan, double-click to reset. Display-only for the
//! audio data itself — it never mutates the buffer.

use egui::epaint::Mesh;
use egui::{pos2, vec2, Painter, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Widget};
use lumen_ui_core::{UiThemeExt, WaveformRecipe};

/// Base width before density scaling, in points — a floor; the widget fills the available width.
const WAVE_BASE_WIDTH: f32 = 360.0;
/// Default height before density scaling, in points (a DAW-lane-like aspect).
const WAVE_DEFAULT_HEIGHT: f32 = 96.0;
/// Baseline / minimum amplitude thickness, in points.
const WAVE_STROKE: f32 = 1.0;
/// Precise per-sample line thickness, in points.
const WAVE_LINE_STROKE: f32 = 1.4;
/// Half-height of the envelope as a fraction of the widget height — leaves a top/bottom margin so
/// peaks don't touch the edges (Seno parity: `waveHeight * 0.45`).
const WAVE_HALF_HEIGHT_RATIO: f32 = 0.45;
/// Background corner radius, in points.
const WAVE_RADIUS: f32 = 2.0;
/// Maximum zoom: the viewport may show as little as `1 / WAVE_MAX_ZOOM` of the buffer.
const WAVE_MAX_ZOOM: f32 = 800.0;
/// Scroll-to-zoom sensitivity — exponential factor applied per unit of smooth scroll.
const WAVE_ZOOM_RATE: f32 = 0.0018;
/// At or below this many samples per pixel, switch from the envelope to the precise sample line.
const WAVE_PRECISE_SAMPLES_PER_PX: f32 = 1.0;

/// Persistent zoom/pan state, keyed by the widget's auto id in egui's temporary data store.
#[derive(Clone, Copy)]
struct WaveformState {
    /// `1.0` = whole buffer visible; `N` = `1/N` of the buffer visible.
    zoom: f32,
    /// Left edge of the viewport as a fraction of the buffer, in `0..=(1 - 1/zoom)`.
    offset: f32,
}

impl Default for WaveformState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            offset: 0.0,
        }
    }
}

impl WaveformState {
    /// Clamp zoom to its range and keep the viewport inside the buffer.
    fn clamped(mut self) -> Self {
        self.zoom = self.zoom.clamp(1.0, WAVE_MAX_ZOOM);
        let max_offset = (1.0 - 1.0 / self.zoom).max(0.0);
        self.offset = self.offset.clamp(0.0, max_offset);
        self
    }

    /// Zoom by `factor` while keeping the buffer fraction under `cursor` (in `0..=1` across the
    /// viewport) pinned in place — the natural "zoom toward the pointer" behaviour.
    fn zoom_at(mut self, cursor: f32, factor: f32) -> Self {
        let anchor = self.offset + cursor / self.zoom;
        self.zoom = (self.zoom * factor).clamp(1.0, WAVE_MAX_ZOOM);
        self.offset = anchor - cursor / self.zoom;
        self.clamped()
    }
}

/// An interactive, zoomable waveform of a `&[f32]` sample buffer (samples in `-1.0..=1.0`). Fills
/// the available width; set the height with [`Waveform::height`]. Zoom/pan can be disabled with
/// [`Waveform::interactive`] for a static display.
#[derive(Clone, Copy, Debug)]
pub struct Waveform<'a> {
    samples: &'a [f32],
    height: f32,
    interactive: bool,
}

impl<'a> Waveform<'a> {
    #[must_use]
    pub fn new(samples: &'a [f32]) -> Self {
        Self {
            samples,
            height: WAVE_DEFAULT_HEIGHT,
            interactive: true,
        }
    }

    /// Set the widget height in points (before density scaling).
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Enable or disable scroll-to-zoom / drag-to-pan (default `true`).
    #[must_use]
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }
}

impl Widget for Waveform<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = WaveformRecipe::resolve(ui.theme().tokens());
        let scale = ui.ui_ctx().density_scale();
        let width = ui.available_width().max(WAVE_BASE_WIDTH * scale);
        let sense = if self.interactive {
            Sense::click_and_drag()
        } else {
            Sense::hover()
        };
        let (rect, response) = ui.allocate_exact_size(vec2(width, self.height * scale), sense);

        // Zoom/pan state is keyed by the widget's own auto id, so multiple waveforms never collide.
        let mut state: WaveformState = ui.data(|d| d.get_temp(response.id)).unwrap_or_default();
        if self.interactive && self.samples.len() > 1 {
            state = update_view(state, &response, ui, rect);
            ui.data_mut(|d| d.insert_temp(response.id, state));
        }

        paint(ui.painter(), rect, &recipe, self.samples, state);
        response
    }
}

/// Apply this frame's scroll (zoom), drag (pan) and double-click (reset) to the view state.
fn update_view(
    mut state: WaveformState,
    response: &Response,
    ui: &Ui,
    rect: Rect,
) -> WaveformState {
    if response.double_clicked() {
        return WaveformState::default();
    }
    if response.hovered() {
        let scroll = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll != 0.0 && rect.width() > 0.0 {
            let cursor = response.hover_pos().map_or(0.5, |p| {
                ((p.x - rect.left()) / rect.width()).clamp(0.0, 1.0)
            });
            state = state.zoom_at(cursor, (scroll * WAVE_ZOOM_RATE).exp());
        }
    }
    if response.dragged() && rect.width() > 0.0 {
        state.offset -= response.drag_delta().x / (rect.width() * state.zoom);
    }
    state.clamped()
}

/// Per-frame geometry derived from the rect and zoom state — bundled to keep paint helpers small.
#[derive(Clone, Copy)]
struct WaveLayout {
    rect: Rect,
    mid_y: f32,
    half_h: f32,
    /// First visible sample as a (fractional) buffer index.
    start: f32,
    /// Number of samples spanned by the viewport.
    visible: f32,
}

/// Paint the background, baseline, and the appropriate representation for the current zoom.
fn paint(
    painter: &Painter,
    rect: Rect,
    recipe: &WaveformRecipe,
    samples: &[f32],
    state: WaveformState,
) {
    painter.rect_filled(rect, WAVE_RADIUS, recipe.background);
    let mid_y = rect.center().y;
    let half_h = rect.height() * WAVE_HALF_HEIGHT_RATIO;
    painter.line_segment(
        [pos2(rect.left(), mid_y), pos2(rect.right(), mid_y)],
        Stroke::new(WAVE_STROKE, recipe.baseline),
    );

    if samples.is_empty() {
        return;
    }

    let total = samples.len() as f32;
    let visible = (total / state.zoom).max(1.0);
    let layout = WaveLayout {
        rect,
        mid_y,
        half_h,
        start: state.offset * total,
        visible,
    };
    let samples_per_px = visible / rect.width().max(1.0);

    if samples_per_px <= WAVE_PRECISE_SAMPLES_PER_PX {
        paint_precise(painter, recipe, samples, layout);
    } else {
        paint_envelope(painter, recipe, samples, layout);
    }
}

/// Zoomed-in look: only the true per-sample line — a precise, continuous trace (no fill, so it
/// reads like an oscilloscope / Pro Tools sample view rather than a colored blob).
fn paint_precise(painter: &Painter, recipe: &WaveformRecipe, samples: &[f32], layout: WaveLayout) {
    let WaveLayout {
        rect,
        mid_y,
        half_h,
        start,
        visible,
    } = layout;
    let first = (start.floor() as usize).min(samples.len().saturating_sub(1));
    let last = ((start + visible).ceil() as usize).min(samples.len());
    if last <= first + 1 {
        return;
    }
    let x_of = |i: usize| rect.left() + (i as f32 - start) / visible * rect.width();
    let y_of = |v: f32| mid_y - v.clamp(-1.0, 1.0) * half_h;
    let line: Vec<Pos2> = (first..last)
        .map(|i| pos2(x_of(i), y_of(samples[i])))
        .collect();
    painter.add(Shape::line(
        line,
        Stroke::new(WAVE_LINE_STROKE, recipe.wave),
    ));
}

/// Zoomed-out look: per-pixel-column min/max envelope, filled body plus top/bottom outline.
fn paint_envelope(painter: &Painter, recipe: &WaveformRecipe, samples: &[f32], layout: WaveLayout) {
    let WaveLayout {
        rect,
        mid_y,
        half_h,
        start,
        visible,
    } = layout;
    let cols = rect.width().max(1.0) as usize;
    let mut top = Vec::with_capacity(cols + 1);
    let mut bot = Vec::with_capacity(cols + 1);
    for col in 0..=cols {
        let a = (start + col as f32 / cols as f32 * visible).floor() as usize;
        let b = (start + (col + 1) as f32 / cols as f32 * visible).ceil() as usize;
        let a = a.min(samples.len().saturating_sub(1));
        let b = b.clamp(a + 1, samples.len());
        let (mut lo, mut hi) = (f32::INFINITY, f32::NEG_INFINITY);
        for &s in &samples[a..b] {
            lo = lo.min(s);
            hi = hi.max(s);
        }
        if lo > hi {
            lo = 0.0;
            hi = 0.0;
        }
        let x = rect.left() + col as f32;
        top.push(pos2(x, mid_y - hi.clamp(-1.0, 1.0) * half_h));
        bot.push(pos2(x, mid_y - lo.clamp(-1.0, 1.0) * half_h));
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zoom_clamps_to_range() {
        let zoomed_out = WaveformState {
            zoom: 0.2,
            offset: 0.0,
        }
        .clamped();
        assert_eq!(zoomed_out.zoom, 1.0); // never below "whole buffer"
        let zoomed_in = WaveformState {
            zoom: 5000.0,
            offset: 0.0,
        }
        .clamped();
        assert_eq!(zoomed_in.zoom, WAVE_MAX_ZOOM);
    }

    #[test]
    fn offset_stays_inside_buffer() {
        let s = WaveformState {
            zoom: 4.0,
            offset: 0.95,
        }
        .clamped();
        assert!((s.offset - 0.75).abs() < 1e-6); // 1 - 1/4
    }

    #[test]
    fn zoom_at_pins_the_cursor_sample() {
        // Whole buffer visible; zoom 2× toward the right edge keeps fraction 1.0 at the right edge.
        let s = WaveformState::default().zoom_at(1.0, 2.0);
        assert!((s.zoom - 2.0).abs() < 1e-6);
        assert!((s.offset - 0.5).abs() < 1e-6); // right half now fills the viewport
    }
}
