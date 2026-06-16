//! [`Skeleton`] — a static placeholder block shown while content loads.

use egui::{vec2, Response, Sense, Ui, Widget};
use lumen_ui_core::{SkeletonRecipe, UiThemeExt};

/// A loading placeholder: a rounded rectangle filled with the theme's
/// `surface_variant`. Size is explicit so it can stand in for any content shape.
#[derive(Clone, Copy, Debug)]
pub struct Skeleton {
    width: f32,
    height: f32,
}

impl Skeleton {
    #[must_use]
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Widget for Skeleton {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = SkeletonRecipe::resolve(ui.theme().tokens());
        let (rect, response) =
            ui.allocate_exact_size(vec2(self.width, self.height), Sense::hover());
        ui.painter()
            .rect_filled(rect, recipe.corner_radius, recipe.fill);
        response
    }
}
