//! [`Alert`] — an inline, block-level status banner with a semantic variant.

use egui::{Frame, Response, RichText, Stroke, Ui, Widget};
use lumen_ui_core::{AlertRecipe, AlertVariant, UiThemeExt};

/// Border thickness of an alert, in points.
const ALERT_BORDER_WIDTH: f32 = 1.0;

/// An inline alert / banner. Build with a variant constructor, optionally add a
/// [`Alert::title`], then `ui.add(alert)`. Colors come from the theme.
#[derive(Clone, Debug)]
pub struct Alert {
    message: String,
    title: Option<String>,
    variant: AlertVariant,
}

impl Alert {
    fn with(message: impl Into<String>, variant: AlertVariant) -> Self {
        Self {
            message: message.into(),
            title: None,
            variant,
        }
    }

    #[must_use]
    pub fn info(message: impl Into<String>) -> Self {
        Self::with(message, AlertVariant::Info)
    }

    #[must_use]
    pub fn success(message: impl Into<String>) -> Self {
        Self::with(message, AlertVariant::Success)
    }

    #[must_use]
    pub fn warning(message: impl Into<String>) -> Self {
        Self::with(message, AlertVariant::Warning)
    }

    #[must_use]
    pub fn danger(message: impl Into<String>) -> Self {
        Self::with(message, AlertVariant::Danger)
    }

    /// Add a bold, accent-colored title above the message.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Widget for Alert {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = AlertRecipe::resolve(ui.theme().tokens(), self.variant, &ui.ui_ctx());
        Frame::NONE
            .fill(recipe.fill)
            .stroke(Stroke::new(ALERT_BORDER_WIDTH, recipe.accent))
            .corner_radius(recipe.corner_radius)
            .inner_margin(crate::util::margin(recipe.inner_margin))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    if let Some(title) = &self.title {
                        ui.label(RichText::new(title).color(recipe.accent).strong());
                    }
                    ui.label(RichText::new(&self.message).color(recipe.text_color));
                });
            })
            .response
    }
}
