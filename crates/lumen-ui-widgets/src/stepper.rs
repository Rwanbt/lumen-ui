//! [`Stepper`] — a horizontal multi-step progress indicator.

use egui::{vec2, Align2, FontId, Response, Sense, Stroke, Ui};
use lumen_ui_core::{StepperRecipe, UiThemeExt};

/// Width of the connector drawn between two step circles, in points.
const CONNECTOR_WIDTH: f32 = 24.0;
/// Thickness of the connector line, in points.
const CONNECTOR_THICKNESS: f32 = 2.0;

/// A step indicator (wizard/onboarding). `current` is the 0-based active step;
/// earlier steps render as done, later ones as upcoming. Display-only.
#[derive(Clone, Debug)]
pub struct Stepper {
    current: usize,
    steps: Vec<String>,
}

impl Stepper {
    #[must_use]
    pub fn new(current: usize) -> Self {
        Self {
            current,
            steps: Vec::new(),
        }
    }

    #[must_use]
    pub fn step(mut self, label: impl Into<String>) -> Self {
        self.steps.push(label.into());
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let recipe = StepperRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        ui.horizontal(|ui| {
            for (index, label) in self.steps.iter().enumerate() {
                if index > 0 {
                    let color = if index <= self.current {
                        recipe.connector_done
                    } else {
                        recipe.connector_todo
                    };
                    let (rect, _) = ui.allocate_exact_size(
                        vec2(CONNECTOR_WIDTH, recipe.circle_size),
                        Sense::hover(),
                    );
                    ui.painter().hline(
                        rect.x_range(),
                        rect.center().y,
                        Stroke::new(CONNECTOR_THICKNESS, color),
                    );
                }

                let reached = index <= self.current;
                let (fill, number_color) = if reached {
                    (recipe.active_fill, recipe.active_text)
                } else {
                    (recipe.inactive_fill, recipe.inactive_text)
                };
                let (circle, _) = ui.allocate_exact_size(
                    vec2(recipe.circle_size, recipe.circle_size),
                    Sense::hover(),
                );
                let painter = ui.painter();
                painter.circle_filled(circle.center(), recipe.circle_size / 2.0, fill);
                painter.text(
                    circle.center(),
                    Align2::CENTER_CENTER,
                    format!("{}", index + 1),
                    FontId::proportional(recipe.text_size),
                    number_color,
                );

                let label_color = if reached {
                    recipe.label_active
                } else {
                    recipe.label_inactive
                };
                ui.label(
                    egui::RichText::new(label)
                        .color(label_color)
                        .size(recipe.text_size),
                );
            }
        })
        .response
    }
}
