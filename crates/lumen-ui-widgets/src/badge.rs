//! [`Badge`] — a small pill-shaped status label.

use egui::{Frame, Margin, Response, RichText, Ui, Widget};
use lumen_ui_core::{BadgeVariant, UiThemeExt};

/// A compact status label. Build it with [`Badge::new`] (neutral) or a variant
/// constructor, then `ui.add(badge)`.
#[derive(Clone, Debug)]
pub struct Badge {
    text: String,
    variant: BadgeVariant,
}

impl Badge {
    fn with(text: impl Into<String>, variant: BadgeVariant) -> Self {
        Self {
            text: text.into(),
            variant,
        }
    }

    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self::with(text, BadgeVariant::Neutral)
    }

    #[must_use]
    pub fn primary(text: impl Into<String>) -> Self {
        Self::with(text, BadgeVariant::Primary)
    }

    #[must_use]
    pub fn success(text: impl Into<String>) -> Self {
        Self::with(text, BadgeVariant::Success)
    }

    #[must_use]
    pub fn warning(text: impl Into<String>) -> Self {
        Self::with(text, BadgeVariant::Warning)
    }

    #[must_use]
    pub fn danger(text: impl Into<String>) -> Self {
        Self::with(text, BadgeVariant::Danger)
    }
}

impl Widget for Badge {
    fn ui(self, ui: &mut Ui) -> Response {
        let r = ui.theme().badge_recipe(self.variant, &ui.ui_ctx());
        Frame::NONE
            .fill(r.fill)
            .corner_radius(r.corner_radius)
            .inner_margin(Margin::symmetric(
                r.inner_margin.x as i8,
                r.inner_margin.y as i8,
            ))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(self.text)
                        .color(r.text_color)
                        .size(r.text_size),
                )
            })
            .inner
    }
}
