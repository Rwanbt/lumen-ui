//! [`Pagination`] — page navigation with prev/next arrows and page numbers.

use egui::{Button, Frame, Margin, RichText, Ui};
use lumen_ui_core::{PaginationRecipe, UiThemeExt};

/// Previous-page arrow glyph.
const PREV_GLYPH: &str = "\u{2039}"; // ‹
/// Next-page arrow glyph.
const NEXT_GLYPH: &str = "\u{203a}"; // ›

/// A pagination control over `total` pages with a 0-based `current` page.
/// [`Pagination::show`] returns the newly requested page, if any.
///
/// Note (PoC scope): renders every page number; ellipsis windowing for large page
/// counts is a future enhancement.
#[derive(Clone, Copy, Debug)]
pub struct Pagination {
    current: usize,
    total: usize,
}

impl Pagination {
    #[must_use]
    pub fn new(current: usize, total: usize) -> Self {
        Self { current, total }
    }

    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let recipe = PaginationRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let mut requested = None;
        ui.horizontal(|ui| {
            let link = |ui: &mut Ui, text: &str| {
                ui.add(
                    Button::new(
                        RichText::new(text)
                            .color(recipe.text)
                            .size(recipe.text_size),
                    )
                    .frame(false),
                )
                .clicked()
            };

            if self.current > 0 && link(ui, PREV_GLYPH) {
                requested = Some(self.current - 1);
            }
            for page in 0..self.total {
                let label = format!("{}", page + 1);
                if page == self.current {
                    Frame::NONE
                        .fill(recipe.active_fill)
                        .corner_radius(recipe.corner_radius)
                        .inner_margin(Margin::symmetric(
                            recipe.inner_margin.x as i8,
                            recipe.inner_margin.y as i8,
                        ))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(&label)
                                    .color(recipe.active_text)
                                    .size(recipe.text_size),
                            );
                        });
                } else if link(ui, &label) {
                    requested = Some(page);
                }
            }
            if self.current + 1 < self.total && link(ui, NEXT_GLYPH) {
                requested = Some(self.current + 1);
            }
        });
        requested
    }
}
