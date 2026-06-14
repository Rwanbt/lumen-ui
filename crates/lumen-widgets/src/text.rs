//! Themed text primitives: [`Label`] and [`Heading`].
//!
//! Both resolve their color and size from the installed theme's [`lumen_core::TextRecipe`]
//! for a [`TextRole`] — no hard-coded color or font size.

use egui::{Response, RichText, Ui, Widget};
use lumen_core::{TextRole, UiThemeExt};

/// Body text. Use [`Label::muted`] for secondary/de-emphasized text.
#[derive(Clone, Debug)]
pub struct Label {
    text: String,
    role: TextRole,
}

impl Label {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            role: TextRole::Body,
        }
    }

    /// De-emphasized text (uses the theme's muted color).
    #[must_use]
    pub fn muted(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            role: TextRole::Muted,
        }
    }
}

impl Widget for Label {
    fn ui(self, ui: &mut Ui) -> Response {
        let r = ui.theme().text_recipe(self.role, &ui.ui_ctx());
        ui.label(RichText::new(self.text).color(r.color).size(r.size))
    }
}

/// Section heading. Use [`Heading::display`] for the largest, page-title size.
#[derive(Clone, Debug)]
pub struct Heading {
    text: String,
    role: TextRole,
}

impl Heading {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            role: TextRole::Heading,
        }
    }

    /// The largest text size (page title).
    #[must_use]
    pub fn display(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            role: TextRole::Display,
        }
    }
}

impl Widget for Heading {
    fn ui(self, ui: &mut Ui) -> Response {
        let r = ui.theme().text_recipe(self.role, &ui.ui_ctx());
        ui.label(
            RichText::new(self.text)
                .color(r.color)
                .size(r.size)
                .strong(),
        )
    }
}
