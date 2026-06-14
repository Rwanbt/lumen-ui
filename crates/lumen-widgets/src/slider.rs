//! [`Slider`] — a draggable value control.

use std::ops::RangeInclusive;

use egui::{pos2, vec2, Rect, Response, Sense, Ui, Widget};
use lumen_core::{UiThemeExt, WidgetState};

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
        let height = ui.spacing().interact_size.y;
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

        let t = ((*self.value - min) / span).clamp(0.0, 1.0);
        let state = if !ui.is_enabled() {
            WidgetState::Disabled
        } else if response.dragged() || response.hovered() {
            WidgetState::Hovered
        } else {
            WidgetState::Normal
        };
        let recipe = ui.theme().slider_recipe(state, &ui.ui_ctx());

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

        response
    }
}
