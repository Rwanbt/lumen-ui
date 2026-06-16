//! [`Table`] — a themed data table (header + rows, optional striping).

use egui::{Grid, Response, RichText, Ui};
use lumen_ui_core::{TableRecipe, UiThemeExt};

/// A simple themed table built on [`egui::Grid`]: a header row of column titles
/// followed by string rows, with optional zebra striping.
///
/// Scope: static string cells. Sorting, virtualization and per-column widths are
/// future enhancements; pair with [`crate::Pagination`] for paged data.
#[derive(Clone, Debug)]
pub struct Table {
    id_source: String,
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    striped: bool,
}

impl Table {
    #[must_use]
    pub fn new(id_source: impl Into<String>) -> Self {
        Self {
            id_source: id_source.into(),
            columns: Vec::new(),
            rows: Vec::new(),
            striped: true,
        }
    }

    #[must_use]
    pub fn column(mut self, label: impl Into<String>) -> Self {
        self.columns.push(label.into());
        self
    }

    /// Append a row. Extra cells beyond the column count still render; missing
    /// cells simply leave the trailing columns empty.
    #[must_use]
    pub fn row<I, S>(mut self, cells: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.rows.push(cells.into_iter().map(Into::into).collect());
        self
    }

    #[must_use]
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let recipe = TableRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        Grid::new(&self.id_source)
            .striped(self.striped)
            .num_columns(self.columns.len())
            .spacing(recipe.spacing)
            .show(ui, |ui| {
                for column in &self.columns {
                    ui.label(
                        RichText::new(column)
                            .color(recipe.header_color)
                            .size(recipe.header_size)
                            .strong(),
                    );
                }
                ui.end_row();

                for row in &self.rows {
                    for cell in row {
                        ui.label(
                            RichText::new(cell)
                                .color(recipe.cell_color)
                                .size(recipe.cell_size),
                        );
                    }
                    ui.end_row();
                }
            })
            .response
    }
}
