//! [`Toolbar`] and [`StatusBar`] — thin horizontal bar helpers, typically used as
//! the `toolbar` / `status_bar` regions of a [`crate::DashboardLayout`].

use egui::{InnerResponse, Ui};

/// A horizontal action bar with consistent inner padding.
#[derive(Clone, Copy, Debug, Default)]
pub struct Toolbar;

impl Toolbar {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Lay out toolbar items left-to-right.
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        ui.horizontal(|ui| {
            ui.add_space(4.0);
            content(ui)
        })
    }
}

/// A compact bottom status bar. Items render in the theme's muted body size by default.
#[derive(Clone, Copy, Debug, Default)]
pub struct StatusBar;

impl StatusBar {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Lay out status items left-to-right.
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        ui.horizontal(|ui| {
            ui.add_space(4.0);
            content(ui)
        })
    }
}
