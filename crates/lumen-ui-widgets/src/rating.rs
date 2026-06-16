//! [`Rating`] — a clickable star rating bound to a count.

use egui::{Button, Response, RichText, Ui};
use lumen_ui_core::{RatingRecipe, UiThemeExt};

/// Filled star glyph (★).
const FILLED_STAR: &str = "\u{2605}";
/// Empty star glyph (☆).
const EMPTY_STAR: &str = "\u{2606}";
/// Default number of stars.
const DEFAULT_MAX: u32 = 5;

/// A star rating bound to a `&mut u32` (number of filled stars). Clicking the
/// n-th star sets the value to `n`.
#[derive(Debug)]
pub struct Rating<'a> {
    value: &'a mut u32,
    max: u32,
}

impl<'a> Rating<'a> {
    #[must_use]
    pub fn new(value: &'a mut u32) -> Self {
        Self {
            value,
            max: DEFAULT_MAX,
        }
    }

    #[must_use]
    pub fn max(mut self, max: u32) -> Self {
        self.max = max;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let recipe = RatingRecipe::resolve(ui.theme().tokens());
        ui.horizontal(|ui| {
            for star in 1..=self.max {
                let filled = star <= *self.value;
                let (glyph, color) = if filled {
                    (FILLED_STAR, recipe.filled)
                } else {
                    (EMPTY_STAR, recipe.empty)
                };
                let clicked = ui
                    .add(
                        Button::new(RichText::new(glyph).color(color).size(recipe.size))
                            .frame(false),
                    )
                    .clicked();
                if clicked {
                    *self.value = star;
                }
            }
        })
        .response
    }
}
