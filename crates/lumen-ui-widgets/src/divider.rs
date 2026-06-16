//! [`Divider`] — a thin themed separator rule (horizontal or vertical).

use egui::{vec2, Response, Sense, Stroke, Ui, Widget};
use lumen_ui_core::{DividerRecipe, UiThemeExt};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

/// A thin separator colored by the theme's `border` token. Spans the available
/// width (horizontal) or height (vertical).
#[derive(Clone, Copy, Debug)]
pub struct Divider {
    orientation: Orientation,
}

impl Divider {
    #[must_use]
    pub fn horizontal() -> Self {
        Self {
            orientation: Orientation::Horizontal,
        }
    }

    #[must_use]
    pub fn vertical() -> Self {
        Self {
            orientation: Orientation::Vertical,
        }
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::horizontal()
    }
}

impl Widget for Divider {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = DividerRecipe::resolve(ui.theme().tokens());
        let stroke = Stroke::new(recipe.thickness, recipe.color);
        match self.orientation {
            Orientation::Horizontal => {
                let width = ui.available_width();
                let (rect, response) =
                    ui.allocate_exact_size(vec2(width, recipe.thickness), Sense::hover());
                ui.painter().hline(rect.x_range(), rect.center().y, stroke);
                response
            }
            Orientation::Vertical => {
                let height = ui.available_height();
                let (rect, response) =
                    ui.allocate_exact_size(vec2(recipe.thickness, height), Sense::hover());
                ui.painter().vline(rect.center().x, rect.y_range(), stroke);
                response
            }
        }
    }
}
