//! [`DescriptionList`] — term/definition pairs in an aligned two-column layout.

use std::hash::Hash;

use egui::{Grid, Id, Response, RichText, Ui};
use lumen_ui_core::{DescriptionListRecipe, UiThemeExt};

/// A list of term/definition pairs (like HTML `<dl>`), laid out as two aligned columns.
/// Build with [`DescriptionList::new`] + [`DescriptionList::item`], then [`DescriptionList::show`].
#[derive(Debug)]
pub struct DescriptionList {
    id: Id,
    items: Vec<(String, String)>,
}

impl DescriptionList {
    /// `id_source` must be stable and unique within the parent `Ui`.
    #[must_use]
    pub fn new(id_source: impl Hash) -> Self {
        Self {
            id: Id::new(id_source),
            items: Vec::new(),
        }
    }

    #[must_use]
    pub fn item(mut self, term: impl Into<String>, definition: impl Into<String>) -> Self {
        self.items.push((term.into(), definition.into()));
        self
    }

    /// Draw the list. Returns the grid's response.
    pub fn show(self, ui: &mut Ui) -> Response {
        let recipe = DescriptionListRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let column_gap = ui.spacing().item_spacing.x.max(recipe.row_gap);
        Grid::new(self.id)
            .num_columns(2)
            .spacing([column_gap, recipe.row_gap])
            .show(ui, |ui| {
                for (term, definition) in &self.items {
                    ui.label(
                        RichText::new(term)
                            .color(recipe.term_color)
                            .size(recipe.term_size),
                    );
                    ui.label(
                        RichText::new(definition)
                            .color(recipe.definition_color)
                            .size(recipe.definition_size),
                    );
                    ui.end_row();
                }
            })
            .response
    }
}
