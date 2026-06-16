//! [`Breadcrumb`] — a navigation trail; ancestor segments are clickable.

use egui::{Button, RichText, Ui};
use lumen_ui_core::{BreadcrumbRecipe, UiThemeExt};

/// Separator glyph drawn between segments.
const SEPARATOR: &str = "/";

/// A breadcrumb trail. The last segment is the current page (not clickable); the
/// rest are links. [`Breadcrumb::show`] returns the index of a clicked ancestor.
#[derive(Clone, Debug, Default)]
pub struct Breadcrumb {
    segments: Vec<String>,
}

impl Breadcrumb {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn item(mut self, label: impl Into<String>) -> Self {
        self.segments.push(label.into());
        self
    }

    /// Returns `Some(index)` of a clicked ancestor segment this frame, else `None`.
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let recipe = BreadcrumbRecipe::resolve(ui.theme().tokens());
        let last = self.segments.len().saturating_sub(1);
        let mut clicked = None;
        ui.horizontal(|ui| {
            for (index, segment) in self.segments.iter().enumerate() {
                if index > 0 {
                    ui.label(
                        RichText::new(SEPARATOR)
                            .color(recipe.separator_color)
                            .size(recipe.text_size),
                    );
                }
                if index == last {
                    ui.label(
                        RichText::new(segment)
                            .color(recipe.current_color)
                            .size(recipe.text_size)
                            .strong(),
                    );
                } else {
                    let link = ui.add(
                        Button::new(
                            RichText::new(segment)
                                .color(recipe.link_color)
                                .size(recipe.text_size),
                        )
                        .frame(false)
                        .small(),
                    );
                    if link.clicked() {
                        clicked = Some(index);
                    }
                }
            }
        });
        clicked
    }
}
