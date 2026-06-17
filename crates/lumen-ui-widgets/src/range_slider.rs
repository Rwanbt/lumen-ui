//! [`RangeSlider`] — a two-handle slider selecting a sub-range.

use std::ops::RangeInclusive;

use egui::{lerp, pos2, vec2, Rect, Response, Sense, Ui, Widget, WidgetInfo};
use lumen_ui_core::{UiThemeExt, WidgetState};

use crate::focus::focus_ring;

/// Thickness of the slider track, in points.
const TRACK_HEIGHT: f32 = 4.0;
/// Knob radius as a fraction of the interactive height.
const KNOB_RATIO: f32 = 0.35;

/// A horizontal slider with two handles bound to `(low, high)` `&mut f32` over an
/// inclusive range. The handles cannot cross: `low` is kept ≤ `high`. The handle
/// nearest the pointer is the one moved; it stays selected for the whole drag.
#[derive(Debug)]
pub struct RangeSlider<'a> {
    low: &'a mut f32,
    high: &'a mut f32,
    range: RangeInclusive<f32>,
}

impl<'a> RangeSlider<'a> {
    #[must_use]
    pub fn new(low: &'a mut f32, high: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self { low, high, range }
    }
}

impl Widget for RangeSlider<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let width = ui.spacing().slider_width;
        // a11y (v0.8): hit target follows the density (44 px in Touch).
        let height = ui.ui_ctx().min_interactive_size();
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(width, height), Sense::click_and_drag());

        let (min, max) = (*self.range.start(), *self.range.end());
        let span = (max - min).max(f32::EPSILON);
        let knob_radius = height * KNOB_RATIO;
        let x_min = rect.left() + knob_radius;
        let x_max = rect.right() - knob_radius;

        // Keep the stored values ordered before mapping to pixels.
        if *self.low > *self.high {
            std::mem::swap(self.low, self.high);
        }

        let value_to_x = |v: f32| lerp(x_min..=x_max, ((v - min) / span).clamp(0.0, 1.0));
        let x_to_value = |x: f32| min + ((x - x_min) / (x_max - x_min)).clamp(0.0, 1.0) * span;

        // Remember which handle the drag grabbed so it can't swap mid-drag when handles meet.
        let active_key = response.id;
        if let Some(pos) = response.interact_pointer_pos() {
            let stored = ui.ctx().data(|d| d.get_temp::<u8>(active_key));
            let active = stored.unwrap_or_else(|| {
                let pick = if (pos.x - value_to_x(*self.low)).abs()
                    <= (value_to_x(*self.high) - pos.x).abs()
                {
                    0
                } else {
                    1
                };
                ui.ctx().data_mut(|d| d.insert_temp(active_key, pick));
                pick
            });
            let v = x_to_value(pos.x);
            if active == 0 {
                *self.low = v.min(*self.high);
            } else {
                *self.high = v.max(*self.low);
            }
            response.mark_changed();
        }
        if response.drag_stopped() {
            ui.ctx().data_mut(|d| d.remove::<u8>(active_key));
        }

        let state = if !ui.is_enabled() {
            WidgetState::Disabled
        } else if response.dragged() || response.hovered() {
            WidgetState::Hovered
        } else {
            WidgetState::Normal
        };
        let theme = ui.theme();
        let recipe = theme.slider_recipe(state, &ui.ui_ctx());

        let cy = rect.center().y;
        let low_x = value_to_x(*self.low);
        let high_x = value_to_x(*self.high);
        let half = TRACK_HEIGHT / 2.0;
        let painter = ui.painter();
        painter.rect_filled(
            Rect::from_min_max(pos2(x_min, cy - half), pos2(x_max, cy + half)),
            half,
            recipe.track,
        );
        painter.rect_filled(
            Rect::from_min_max(pos2(low_x, cy - half), pos2(high_x, cy + half)),
            half,
            recipe.fill,
        );
        painter.circle_filled(pos2(low_x, cy), knob_radius, recipe.knob);
        painter.circle_filled(pos2(high_x, cy), knob_radius, recipe.knob);

        // a11y: expose the low bound as the slider value (egui has no native range role).
        let (enabled, low) = (ui.is_enabled(), *self.low);
        response.widget_info(|| WidgetInfo::slider(enabled, f64::from(low), ""));

        focus_ring(
            ui,
            &response,
            theme.tokens().radius.sm,
            theme.tokens().colors.primary,
        );
        response
    }
}
