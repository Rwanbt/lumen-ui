//! [`Carousel`] — show one slide at a time with prev/next navigation and dot indicators.

use std::hash::Hash;

use egui::{Align2, FontId, Id, Response, Sense, Ui, Vec2, WidgetInfo, WidgetType};
use lumen_ui_core::{CarouselRecipe, UiThemeExt};

/// Navigation arrow glyph font size as a fraction of the arrow button size.
const ARROW_GLYPH_RATIO: f32 = 0.8;

/// A carousel bound to a `&mut usize` current-slide index over `len` slides. Renders prev/next
/// arrows (wrapping around the ends) framing the current slide, with a row of dot indicators
/// below. The slide body is supplied per frame by the `add_slide` closure.
///
/// ```ignore
/// Carousel::new("gallery", &mut current, photos.len())
///     .show(ui, |ui, index| { ui.add(Image::new(&photos[index])); });
/// ```
#[derive(Debug)]
pub struct Carousel<'a> {
    id: Id,
    current: &'a mut usize,
    len: usize,
}

impl<'a> Carousel<'a> {
    /// `id_source` must be stable and unique within the parent `Ui`.
    #[must_use]
    pub fn new(id_source: impl Hash, current: &'a mut usize, len: usize) -> Self {
        Self {
            id: Id::new(id_source),
            current,
            len,
        }
    }

    /// Draw the carousel, rendering the current slide via `add_slide`. Returns the response of the
    /// enclosing layout. A no-op (empty response) when `len == 0`.
    pub fn show(self, ui: &mut Ui, add_slide: impl FnOnce(&mut Ui, usize)) -> Response {
        let recipe = CarouselRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        if self.len == 0 {
            return ui.allocate_exact_size(Vec2::ZERO, Sense::hover()).1;
        }
        // Defend against a stale index if the caller shrank the slide set.
        *self.current = (*self.current).min(self.len - 1);

        ui.push_id(self.id, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if arrow(ui, &recipe, "‹", "previous").clicked() {
                        *self.current = (*self.current + self.len - 1) % self.len;
                    }
                    add_slide(ui, *self.current);
                    if arrow(ui, &recipe, "›", "next").clicked() {
                        *self.current = (*self.current + 1) % self.len;
                    }
                });
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = recipe.gap;
                    for index in 0..self.len {
                        let (rect, _) =
                            ui.allocate_exact_size(Vec2::splat(recipe.dot_size), Sense::hover());
                        let color = if index == *self.current {
                            recipe.dot_active
                        } else {
                            recipe.dot_inactive
                        };
                        ui.painter()
                            .circle_filled(rect.center(), recipe.dot_size / 2.0, color);
                    }
                });
            })
            .response
        })
        .inner
    }
}

/// Draw one themed navigation arrow and return its click [`Response`]. `label` is the screen-reader
/// name (the glyphs ‹/› don't read well), which also makes it queryable in tests.
fn arrow(ui: &mut Ui, recipe: &CarouselRecipe, glyph: &str, label: &str) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(recipe.arrow_size), Sense::click());
    let enabled = ui.is_enabled();
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        glyph,
        FontId::proportional(recipe.arrow_size * ARROW_GLYPH_RATIO),
        recipe.arrow_color,
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, label));
    response
}
