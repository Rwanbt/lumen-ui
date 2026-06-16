//! [`Wizard`] — a multi-step flow: step indicator + current step body + navigation.
//!
//! Pure composition of [`Stepper`] (progress) and [`Button`] (Back / Next / Finish).
//! Binds to a `&mut usize` current step; the body closure renders the active step.
//! No recipe — gaps come from theme tokens.

use egui::Ui;
use lumen_ui_core::UiThemeExt;
use lumen_ui_widgets::{Button, Stepper};

/// What the user did in a [`Wizard`] this frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WizardResponse {
    /// The Finish button (on the last step) was clicked.
    pub finished: bool,
}

/// A step-by-step flow bound to a `&mut usize` current step.
///
/// ```ignore
/// let r = Wizard::new(&mut step)
///     .step("Account")
///     .step("Profile")
///     .step("Done")
///     .show(ui, |ui, index| { ui.label(format!("Step {index} body")); });
/// if r.finished { /* complete onboarding */ }
/// ```
pub struct Wizard<'a> {
    current: &'a mut usize,
    steps: Vec<String>,
    back_label: String,
    next_label: String,
    finish_label: String,
}

impl<'a> Wizard<'a> {
    #[must_use]
    pub fn new(current: &'a mut usize) -> Self {
        Self {
            current,
            steps: Vec::new(),
            back_label: "Back".to_owned(),
            next_label: "Next".to_owned(),
            finish_label: "Finish".to_owned(),
        }
    }

    /// Append a step label (shown in the [`Stepper`]).
    #[must_use]
    pub fn step(mut self, label: impl Into<String>) -> Self {
        self.steps.push(label.into());
        self
    }

    /// Override the Back button label (default: "Back").
    #[must_use]
    pub fn back_label(mut self, label: impl Into<String>) -> Self {
        self.back_label = label.into();
        self
    }

    /// Override the Next button label (default: "Next").
    #[must_use]
    pub fn next_label(mut self, label: impl Into<String>) -> Self {
        self.next_label = label.into();
        self
    }

    /// Override the Finish button label (default: "Finish").
    #[must_use]
    pub fn finish_label(mut self, label: impl Into<String>) -> Self {
        self.finish_label = label.into();
        self
    }

    /// Draw the wizard. `body` receives the active step index. Navigation updates
    /// the bound step; the last step shows Finish instead of Next.
    pub fn show(self, ui: &mut Ui, body: impl FnOnce(&mut Ui, usize)) -> WizardResponse {
        let gap = ui.theme().tokens().spacing.md;
        let Wizard {
            current,
            steps,
            back_label,
            next_label,
            finish_label,
        } = self;
        let last = steps.len().saturating_sub(1);
        // Clamp in case the caller set an out-of-range step.
        if *current > last {
            *current = last;
        }
        let active = *current;
        let mut out = WizardResponse::default();

        ui.vertical(|ui| {
            let mut stepper = Stepper::new(active);
            for label in &steps {
                stepper = stepper.step(label.clone());
            }
            stepper.show(ui);

            ui.add_space(gap);
            body(ui, active);
            ui.add_space(gap);

            ui.horizontal(|ui| {
                if active > 0 && ui.add(Button::secondary(back_label.clone())).clicked() {
                    *current = active - 1;
                }
                if active < last {
                    if ui.add(Button::primary(next_label.clone())).clicked() {
                        *current = active + 1;
                    }
                } else if ui.add(Button::primary(finish_label.clone())).clicked() {
                    out.finished = true;
                }
            });
        });

        out
    }
}
