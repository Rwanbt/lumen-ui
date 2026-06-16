//! [`Avatar`] — a circular initials badge themed by tokens.

use egui::{vec2, Align2, Color32, FontId, Response, Sense, Ui, Widget};
use lumen_ui_core::{AvatarRecipe, UiThemeExt};

/// Maximum number of initials rendered in an avatar.
const MAX_INITIALS: usize = 2;

/// A circular avatar showing up to two uppercase initials derived from a name.
/// Diameter and colors come from the theme via [`AvatarRecipe`].
#[derive(Clone, Debug)]
pub struct Avatar {
    initials: String,
    bg: Option<Color32>,
}

impl Avatar {
    /// Derive initials from `name` (first letter of the first two words).
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            initials: initials_from(name),
            bg: None,
        }
    }

    /// Override the background color (defaults to the theme primary).
    #[must_use]
    pub fn color(mut self, bg: Color32) -> Self {
        self.bg = Some(bg);
        self
    }
}

fn initials_from(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .take(MAX_INITIALS)
        .collect::<String>()
        .to_uppercase()
}

impl Widget for Avatar {
    fn ui(self, ui: &mut Ui) -> Response {
        let recipe = AvatarRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let (rect, response) =
            ui.allocate_exact_size(vec2(recipe.size, recipe.size), Sense::hover());
        let bg = self.bg.unwrap_or(recipe.bg);
        let painter = ui.painter();
        painter.circle_filled(rect.center(), recipe.size / 2.0, bg);
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            &self.initials,
            FontId::proportional(recipe.font_size),
            recipe.text_color,
        );
        response
    }
}
