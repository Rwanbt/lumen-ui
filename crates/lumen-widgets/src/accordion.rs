//! [`Accordion`] — a themed collapsible section.
//!
//! A thin, themed wrapper over `egui::CollapsingHeader` (which already persists
//! its open/closed state in `ctx.data`). Drawing follows the installed theme via
//! the global visuals set by `apply_to_ctx`.

use egui::{CollapsingHeader, Ui};

/// A collapsible titled section.
#[derive(Clone, Debug)]
pub struct Accordion {
    title: String,
    default_open: bool,
}

impl Accordion {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            default_open: false,
        }
    }

    #[must_use]
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Draw the section. Returns the closure's value when the body is shown
    /// (i.e. when the section is open), `None` while collapsed.
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> Option<R> {
        CollapsingHeader::new(self.title)
            .default_open(self.default_open)
            .show(ui, add_contents)
            .body_returned
    }
}
