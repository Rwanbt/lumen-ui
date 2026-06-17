//! [`ColorPicker`] — a themed swatch that opens egui's color-picker popup.

use egui::color_picker::{color_picker_color32, Alpha};
use egui::{
    Color32, Popup, PopupCloseBehavior, Response, Sense, StrokeKind, Ui, Vec2, Widget, WidgetInfo,
    WidgetType,
};
use lumen_ui_core::{ColorPickerRecipe, UiThemeExt};

/// A color picker bound to a `&mut Color32`. Renders a themed swatch of the current
/// color; clicking it opens egui's full HSV picker in a popup. Alpha editing is off by
/// default (opaque colors) — call [`ColorPicker::with_alpha`] to enable it.
///
/// The swatch is themed by [`ColorPickerRecipe`]; the picker popup is egui's own
/// color module (themed by `install`), kept as-is rather than reimplemented.
#[derive(Debug)]
pub struct ColorPicker<'a> {
    color: &'a mut Color32,
    // `egui::color_picker::Alpha` is not `Debug`; store the choice as a bool and map it in `ui`.
    alpha: bool,
}

impl<'a> ColorPicker<'a> {
    #[must_use]
    pub fn new(color: &'a mut Color32) -> Self {
        Self {
            color,
            alpha: false,
        }
    }

    /// Enable alpha (transparency) editing in the picker popup.
    #[must_use]
    pub fn with_alpha(mut self) -> Self {
        self.alpha = true;
        self
    }
}

impl Widget for ColorPicker<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = ColorPickerRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let (rect, mut response) = ui.allocate_exact_size(Vec2::splat(recipe.size), Sense::click());
        let enabled = ui.is_enabled();
        ui.painter().rect(
            rect,
            recipe.corner_radius,
            *self.color,
            recipe.border,
            StrokeKind::Inside,
        );
        // The swatch is just a colored square; name it for screen readers (and tests).
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, "color picker"));

        let alpha = if self.alpha {
            Alpha::OnlyBlend
        } else {
            Alpha::Opaque
        };
        // `Popup::menu` opens on click of `response`; the borrow ends before `show`, so the
        // closure may mark the response changed (mirrors egui's own `color_edit_button_srgba`).
        Popup::menu(&response)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                if color_picker_color32(ui, self.color, alpha) {
                    response.mark_changed();
                }
            });
        response
    }
}
