//! [`IconButton`] — a square, ghost-style button wrapping any icon widget.

use egui::{Color32, Sense, Ui, Vec2, Widget};
use lumen_ui_core::{IconButtonRecipe, UiThemeExt};

/// A compact square button that hosts an arbitrary icon widget (e.g. a
/// `lumen_ui_icons::Icon`, or any `egui::Widget`). Ghost style: transparent at
/// rest, themed fill on hover/press. Returns the button's click [`Response`].
///
/// Decoupled from `lumen-ui-icons` on purpose — it accepts `impl Widget`, so the
/// widgets crate keeps its single dependency on `lumen-ui-core`.
///
/// [`Response`]: egui::Response
#[derive(Clone, Debug)]
pub struct IconButton<W> {
    icon: W,
    size: Option<f32>,
}

impl<W: Widget> IconButton<W> {
    #[must_use]
    pub fn new(icon: W) -> Self {
        Self { icon, size: None }
    }

    /// Override the resolved square size (points).
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }
}

impl<W: Widget> Widget for IconButton<W> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let theme = ui.theme();
        let recipe = IconButtonRecipe::resolve(theme.tokens(), theme.mode(), &ui.ui_ctx());
        let size = self.size.unwrap_or(recipe.size);
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());
        let fill = if response.is_pointer_button_down_on() {
            recipe.active_fill
        } else if response.hovered() {
            recipe.hover_fill
        } else {
            Color32::TRANSPARENT
        };
        ui.painter().rect_filled(rect, recipe.corner_radius, fill);
        ui.put(rect, self.icon);
        response
    }
}
