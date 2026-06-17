//! [`Fader`] — a vertical level fader (audio/DAW mixing strips).

use std::ops::RangeInclusive;

use egui::{lerp, pos2, vec2, Rect, Response, Sense, Ui, Widget, WidgetInfo};
use lumen_ui_core::{UiThemeExt, WidgetState};

/// Base width of the fader hit area before density scaling, in points.
const FADER_BASE_WIDTH: f32 = 36.0;
/// Base height of the fader before density scaling, in points.
const FADER_BASE_HEIGHT: f32 = 140.0;
/// Track width as a fraction of the fader width.
const TRACK_WIDTH_RATIO: f32 = 0.18;
/// Knob radius as a fraction of the fader width.
const KNOB_RATIO: f32 = 0.40;

/// A vertical fader bound to a `&mut f32` over an inclusive range. The maximum is at the top; drag
/// or click sets the value. Reuses the theme's [`lumen_ui_core::SliderRecipe`].
#[derive(Debug)]
pub struct Fader<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
}

impl<'a> Fader<'a> {
    #[must_use]
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self { value, range }
    }
}

impl Widget for Fader<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let scale = ui.ui_ctx().density_scale();
        let width = FADER_BASE_WIDTH * scale;
        let height = FADER_BASE_HEIGHT * scale;
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(width, height), Sense::click_and_drag());

        let (min, max) = (*self.range.start(), *self.range.end());
        let span = (max - min).max(f32::EPSILON);
        let knob_radius = width * KNOB_RATIO;
        let y_top = rect.top() + knob_radius;
        let y_bot = rect.bottom() - knob_radius;

        if let Some(pos) = response.interact_pointer_pos() {
            // Top of the track is the maximum, bottom is the minimum.
            let t = ((y_bot - pos.y) / (y_bot - y_top)).clamp(0.0, 1.0);
            let new = min + t * span;
            if (new - *self.value).abs() > f32::EPSILON {
                *self.value = new;
                response.mark_changed();
            }
        }

        let t = ((*self.value - min) / span).clamp(0.0, 1.0);
        let state = if !ui.is_enabled() {
            WidgetState::Disabled
        } else if response.dragged() || response.hovered() {
            WidgetState::Hovered
        } else {
            WidgetState::Normal
        };
        let recipe = ui.theme().slider_recipe(state, &ui.ui_ctx());

        let cx = rect.center().x;
        let track_half = (width * TRACK_WIDTH_RATIO) / 2.0;
        let cy = lerp(y_bot..=y_top, t);
        let painter = ui.painter();
        painter.rect_filled(
            Rect::from_min_max(pos2(cx - track_half, y_top), pos2(cx + track_half, y_bot)),
            track_half,
            recipe.track,
        );
        painter.rect_filled(
            Rect::from_min_max(pos2(cx - track_half, cy), pos2(cx + track_half, y_bot)),
            track_half,
            recipe.fill,
        );
        painter.circle_filled(pos2(cx, cy), knob_radius, recipe.knob);

        let (enabled, value) = (ui.is_enabled(), *self.value);
        response.widget_info(|| WidgetInfo::slider(enabled, f64::from(value), ""));
        response
    }
}
