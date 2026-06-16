//! [`DataGrid`] — a virtualized, sortable table built on [`egui_extras::TableBuilder`].
//!
//! Feature-gated behind `datagrid` (see `docs/adr/0010-datagrid-egui-extras.md`). Unlike the
//! always-available [`crate::Table`] (static `Grid`, small data), `DataGrid` virtualizes its body
//! (only visible rows are rendered), supports resizable columns, and emits sort requests.
//!
//! Sorting is caller-driven: a click on a sortable header updates the bound [`SortState`]; the
//! caller sorts its own data and re-renders. The grid only *renders* the rows it is given.

use egui::{Align, Label, Layout, Response, RichText, Sense, Ui};
use egui_extras::{Column, TableBuilder};
use lumen_ui_core::{DataGridRecipe, UiThemeExt};

/// Minimum width a column may shrink to, in points.
const MIN_COLUMN_WIDTH: f32 = 48.0;

/// Sort direction for a [`DataGrid`] column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    /// The opposite direction.
    #[must_use]
    pub fn toggled(self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }
}

/// Which column is sorted, and in which direction. Bound via [`DataGrid::sort`] and updated when
/// the user clicks a sortable header; the caller reads it to sort its data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SortState {
    /// 0-based index of the sorted column.
    pub column: usize,
    pub direction: SortDirection,
}

#[derive(Clone, Debug)]
struct GridColumn {
    label: String,
    sortable: bool,
}

/// A virtualized, sortable data table. Cells are strings (rich cells come later — ADR-0010).
///
/// ```ignore
/// DataGrid::new("users")
///     .sortable_column("Name")
///     .column("Role")
///     .row(["Ada", "Engineer"])
///     .sort(&mut sort_state)
///     .show(ui);
/// // then sort your data by `sort_state` before the next frame.
/// ```
#[derive(Debug)]
pub struct DataGrid<'a> {
    id_source: String,
    columns: Vec<GridColumn>,
    rows: Vec<Vec<String>>,
    striped: bool,
    resizable: bool,
    row_height: Option<f32>,
    sort: Option<&'a mut Option<SortState>>,
}

impl<'a> DataGrid<'a> {
    #[must_use]
    pub fn new(id_source: impl Into<String>) -> Self {
        Self {
            id_source: id_source.into(),
            columns: Vec::new(),
            rows: Vec::new(),
            striped: true,
            resizable: true,
            row_height: None,
            sort: None,
        }
    }

    /// Append a non-sortable column.
    #[must_use]
    pub fn column(mut self, label: impl Into<String>) -> Self {
        self.columns.push(GridColumn {
            label: label.into(),
            sortable: false,
        });
        self
    }

    /// Append a sortable column (its header is clickable when [`DataGrid::sort`] is bound).
    #[must_use]
    pub fn sortable_column(mut self, label: impl Into<String>) -> Self {
        self.columns.push(GridColumn {
            label: label.into(),
            sortable: true,
        });
        self
    }

    /// Append a row. Cells beyond the column count are dropped; missing cells render empty.
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

    #[must_use]
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Override the fixed body row height (default: from the theme's [`DataGridRecipe`]).
    #[must_use]
    pub fn row_height(mut self, height: f32) -> Self {
        self.row_height = Some(height);
        self
    }

    /// Bind the sort state. Without this, sortable headers render as plain (non-clickable) labels.
    #[must_use]
    pub fn sort(mut self, sort: &'a mut Option<SortState>) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Draw the grid. Returns the response of the enclosing scope.
    pub fn show(self, ui: &mut Ui) -> Response {
        let recipe = DataGridRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let DataGrid {
            id_source,
            columns,
            rows,
            striped,
            resizable,
            row_height,
            mut sort,
        } = self;
        let row_height = row_height.unwrap_or(recipe.row_height);
        let ncols = columns.len();
        let current: Option<SortState> = sort.as_deref().copied().flatten();

        ui.scope(|ui| {
            let mut builder = TableBuilder::new(ui)
                .id_salt(&id_source)
                .striped(striped)
                .resizable(resizable)
                .cell_layout(Layout::left_to_right(Align::Center))
                .min_scrolled_height(0.0);
            for _ in 0..ncols {
                builder = builder.column(
                    Column::remainder()
                        .at_least(MIN_COLUMN_WIDTH)
                        .resizable(resizable)
                        .clip(true),
                );
            }

            let mut clicked: Option<usize> = None;
            let table = builder.header(recipe.header_height, |mut header| {
                for (index, column) in columns.iter().enumerate() {
                    header.col(|ui| {
                        let mut caption = column.label.clone();
                        if let Some(state) = current {
                            if state.column == index {
                                caption.push(' ');
                                caption.push(match state.direction {
                                    SortDirection::Ascending => '\u{25B2}',
                                    SortDirection::Descending => '\u{25BC}',
                                });
                            }
                        }
                        let text = RichText::new(caption)
                            .color(recipe.header_color)
                            .size(recipe.header_size)
                            .strong();
                        // Only clickable when sortable *and* a sort state is bound to store the result.
                        if column.sortable && sort.is_some() {
                            if ui.add(Label::new(text).sense(Sense::click())).clicked() {
                                clicked = Some(index);
                            }
                        } else {
                            ui.add(Label::new(text));
                        }
                    });
                }
            });

            table.body(|body| {
                body.rows(row_height, rows.len(), |mut row| {
                    let r = row.index();
                    for c in 0..ncols {
                        row.col(|ui| {
                            let cell = rows[r].get(c).map_or("", String::as_str);
                            ui.add(
                                Label::new(
                                    RichText::new(cell)
                                        .color(recipe.cell_color)
                                        .size(recipe.cell_size),
                                )
                                .truncate(),
                            );
                        });
                    }
                });
            });

            // Apply a header click to the bound sort state: toggle if same column, else ascending.
            if let (Some(index), Some(state)) = (clicked, sort.as_mut()) {
                **state = Some(match **state {
                    Some(prev) if prev.column == index => SortState {
                        column: index,
                        direction: prev.direction.toggled(),
                    },
                    _ => SortState {
                        column: index,
                        direction: SortDirection::Ascending,
                    },
                });
            }
        })
        .response
    }
}
