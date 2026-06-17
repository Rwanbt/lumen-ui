//! [`NumberInput`] — a numeric field with −/+ stepper buttons.

use std::ops::RangeInclusive;

use egui::{Align2, DragValue, FontId, Response, Sense, Ui, Vec2, Widget, WidgetInfo, WidgetType};
use lumen_ui_core::{NumberInputRecipe, UiThemeExt};

/// Stepper glyph font size as a fraction of the button size.
const GLYPH_RATIO: f32 = 0.6;

/// A numeric input bound to a `&mut f64` over an inclusive range. Renders a
/// draggable/editable [`egui::DragValue`] flanked by −/+ stepper buttons that each
/// move the value by `step` (default `1.0`), clamped to the range.
#[derive(Debug)]
pub struct NumberInput<'a> {
    value: &'a mut f64,
    range: RangeInclusive<f64>,
    step: f64,
}

impl<'a> NumberInput<'a> {
    #[must_use]
    pub fn new(value: &'a mut f64, range: RangeInclusive<f64>) -> Self {
        Self {
            value,
            range,
            step: 1.0,
        }
    }

    /// Override the increment applied by the −/+ buttons and the drag speed.
    #[must_use]
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }
}

/// Draw one themed −/+ stepper button and return its click [`Response`]. `label` is the
/// screen-reader name (the glyphs −/+ don't read well), which also makes it queryable in tests.
fn stepper(ui: &mut Ui, recipe: &NumberInputRecipe, glyph: &str, label: &str) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(recipe.button_size), Sense::click());
    let enabled = ui.is_enabled();
    let painter = ui.painter();
    painter.rect_filled(rect, recipe.corner_radius, recipe.button_fill);
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        glyph,
        FontId::proportional(recipe.button_size * GLYPH_RATIO),
        recipe.button_text,
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, label));
    response
}

impl Widget for NumberInput<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = NumberInputRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let (min, max) = (*self.range.start(), *self.range.end());

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = recipe.gap;

            let mut changed = false;
            if stepper(ui, &recipe, "−", "decrement").clicked() {
                *self.value = (*self.value - self.step).clamp(min, max);
                changed = true;
            }
            let mut response = ui.add(
                DragValue::new(self.value)
                    .range(self.range.clone())
                    .speed(self.step),
            );
            if stepper(ui, &recipe, "+", "increment").clicked() {
                *self.value = (*self.value + self.step).clamp(min, max);
                changed = true;
            }
            if changed {
                response.mark_changed();
            }
            response
        })
        .inner
    }
}
