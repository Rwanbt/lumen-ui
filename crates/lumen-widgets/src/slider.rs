//! [`Slider`] — a draggable value control.

use std::ops::RangeInclusive;

use egui::{pos2, vec2, Key, Rect, Response, Sense, Ui, Widget, WidgetInfo};
use lumen_core::{UiThemeExt, WidgetState};

use crate::focus::focus_ring;

/// Fraction of the range moved per arrow-key press when the slider is focused.
const KEYBOARD_STEP_FRACTION: f32 = 0.01;

/// A horizontal slider bound to a `&mut f32` over an inclusive range.
#[derive(Debug)]
pub struct Slider<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
}

impl<'a> Slider<'a> {
    #[must_use]
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self { value, range }
    }
}

impl Widget for Slider<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let width = ui.spacing().slider_width;
        // a11y (v0.8): hit target follows the density (44 px in Touch).
        let height = ui.ui_ctx().min_interactive_size();
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(width, height), Sense::click_and_drag());

        let (min, max) = (*self.range.start(), *self.range.end());
        let span = (max - min).max(f32::EPSILON);
        let knob_radius = height * 0.35;
        let x_min = rect.left() + knob_radius;
        let x_max = rect.right() - knob_radius;

        if let Some(pos) = response.interact_pointer_pos() {
            let t = ((pos.x - x_min) / (x_max - x_min)).clamp(0.0, 1.0);
            let new = min + t * span;
            if (new - *self.value).abs() > f32::EPSILON {
                *self.value = new;
                response.mark_changed();
            }
        }

        // Keyboard nav (a11y, v0.8): arrows nudge the value when the slider is focused.
        if response.has_focus() {
            let step = span * KEYBOARD_STEP_FRACTION;
            let delta = ui.input(|i| {
                let mut d = 0.0;
                if i.key_pressed(Key::ArrowRight) || i.key_pressed(Key::ArrowUp) {
                    d += step;
                }
                if i.key_pressed(Key::ArrowLeft) || i.key_pressed(Key::ArrowDown) {
                    d -= step;
                }
                d
            });
            if delta != 0.0 {
                *self.value = (*self.value + delta).clamp(min, max);
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
        let theme = ui.theme();
        let recipe = theme.slider_recipe(state, &ui.ui_ctx());

        let cx = egui::lerp(x_min..=x_max, t);
        let cy = rect.center().y;
        let track_h = 4.0;
        let painter = ui.painter();
        painter.rect_filled(
            Rect::from_min_max(
                pos2(x_min, cy - track_h / 2.0),
                pos2(x_max, cy + track_h / 2.0),
            ),
            track_h / 2.0,
            recipe.track,
        );
        painter.rect_filled(
            Rect::from_min_max(
                pos2(x_min, cy - track_h / 2.0),
                pos2(cx, cy + track_h / 2.0),
            ),
            track_h / 2.0,
            recipe.fill,
        );
        painter.circle_filled(pos2(cx, cy), knob_radius, recipe.knob);

        // a11y: expose the value to screen readers / AccessKit (and kittest).
        let (enabled, value) = (ui.is_enabled(), *self.value);
        response.widget_info(|| WidgetInfo::slider(enabled, f64::from(value), ""));

        focus_ring(
            ui,
            &response,
            theme.tokens().radius.sm,
            theme.tokens().colors.primary,
        );
        response
    }
}
