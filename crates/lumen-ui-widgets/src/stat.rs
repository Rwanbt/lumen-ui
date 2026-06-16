//! [`Stat`] — a metric block: a muted label, a large value, an optional delta.

use egui::{Response, RichText, Ui, Widget};
use lumen_ui_core::{StatRecipe, UiThemeExt};

/// A compact metric display (label + big value + optional colored delta).
#[derive(Clone, Debug)]
pub struct Stat {
    label: String,
    value: String,
    delta: Option<(String, bool)>,
}

impl Stat {
    #[must_use]
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            delta: None,
        }
    }

    /// Add a delta line; `positive` colors it with the success token, otherwise danger.
    #[must_use]
    pub fn delta(mut self, delta: impl Into<String>, positive: bool) -> Self {
        self.delta = Some((delta.into(), positive));
        self
    }
}

impl Widget for Stat {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = StatRecipe::resolve(ui.theme().tokens());
        ui.vertical(|ui| {
            ui.label(
                RichText::new(&self.label)
                    .color(recipe.label_color)
                    .size(recipe.label_size),
            );
            ui.label(
                RichText::new(&self.value)
                    .color(recipe.value_color)
                    .size(recipe.value_size)
                    .strong(),
            );
            if let Some((delta, positive)) = &self.delta {
                let color = if *positive {
                    recipe.positive_color
                } else {
                    recipe.negative_color
                };
                ui.label(RichText::new(delta).color(color).size(recipe.label_size));
            }
        })
        .response
    }
}
