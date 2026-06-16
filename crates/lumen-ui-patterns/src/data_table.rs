//! [`DataTable`] — search + sortable [`DataGrid`] + pagination, with the
//! filter/sort/paginate logic wired together.
//!
//! Feature-gated behind `datagrid` (pulls in `lumen-ui-widgets/datagrid`). Where [`DataGrid`] only
//! renders the rows it is given, `DataTable` owns the data transformation a real table needs:
//! it filters by the search query, sorts by the clicked column, and paginates — all driven by a
//! caller-owned [`DataTableState`] so the pattern stays stateless.

use egui::Ui;
use lumen_ui_core::UiThemeExt;
use lumen_ui_widgets::{DataGrid, Pagination, SortDirection, TextField};

/// Default number of rows per page.
const DEFAULT_PAGE_SIZE: usize = 10;

/// Caller-owned state for a [`DataTable`]: the search query, the active sort, and the current
/// (0-based) page. Default is empty query, unsorted, first page.
#[derive(Clone, Debug, Default)]
pub struct DataTableState {
    pub query: String,
    pub sort: Option<lumen_ui_widgets::SortState>,
    /// 0-based page index.
    pub page: usize,
}

#[derive(Clone, Debug)]
struct Column {
    label: String,
    sortable: bool,
}

/// A search + sortable grid + pagination table over string rows.
///
/// ```ignore
/// DataTable::new("users")
///     .sortable_column("Name")
///     .column("Role")
///     .row(["Ada", "Engineer"])
///     .page_size(25)
///     .show(ui, &mut state);
/// ```
pub struct DataTable {
    id_source: String,
    columns: Vec<Column>,
    rows: Vec<Vec<String>>,
    page_size: usize,
    searchable: bool,
}

impl DataTable {
    #[must_use]
    pub fn new(id_source: impl Into<String>) -> Self {
        Self {
            id_source: id_source.into(),
            columns: Vec::new(),
            rows: Vec::new(),
            page_size: DEFAULT_PAGE_SIZE,
            searchable: true,
        }
    }

    /// Append a non-sortable column.
    #[must_use]
    pub fn column(mut self, label: impl Into<String>) -> Self {
        self.columns.push(Column {
            label: label.into(),
            sortable: false,
        });
        self
    }

    /// Append a sortable column.
    #[must_use]
    pub fn sortable_column(mut self, label: impl Into<String>) -> Self {
        self.columns.push(Column {
            label: label.into(),
            sortable: true,
        });
        self
    }

    /// Append a row of string cells.
    #[must_use]
    pub fn row<I, S>(mut self, cells: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.rows.push(cells.into_iter().map(Into::into).collect());
        self
    }

    /// Rows per page (clamped to at least 1; default 10).
    #[must_use]
    pub fn page_size(mut self, size: usize) -> Self {
        self.page_size = size;
        self
    }

    /// Show or hide the search field (default: shown).
    #[must_use]
    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    /// Draw the table: search field, then the current page in a sortable [`DataGrid`], then
    /// pagination. Filtering and sorting apply across the full dataset before paging.
    pub fn show(self, ui: &mut Ui, state: &mut DataTableState) {
        let DataTable {
            id_source,
            columns,
            rows,
            page_size,
            searchable,
        } = self;
        let page_size = page_size.max(1);
        let gap = ui.theme().tokens().spacing.sm;

        if searchable {
            ui.add(TextField::new(&mut state.query).hint("Search"));
            ui.add_space(gap);
        }

        // Filter: case-insensitive substring across any cell.
        let needle = state.query.trim().to_lowercase();
        let mut view: Vec<&Vec<String>> = rows
            .iter()
            .filter(|row| {
                needle.is_empty() || row.iter().any(|cell| cell.to_lowercase().contains(&needle))
            })
            .collect();

        // Sort by the active column (string comparison), if any.
        if let Some(sort) = state.sort {
            if sort.column < columns.len() {
                view.sort_by(|a, b| {
                    let left = a.get(sort.column).map_or("", String::as_str);
                    let right = b.get(sort.column).map_or("", String::as_str);
                    let ordering = left.cmp(right);
                    match sort.direction {
                        SortDirection::Ascending => ordering,
                        SortDirection::Descending => ordering.reverse(),
                    }
                });
            }
        }

        // Paginate (clamp the page to the available range).
        let total_pages = view.len().div_ceil(page_size).max(1);
        if state.page >= total_pages {
            state.page = total_pages - 1;
        }
        let start = state.page * page_size;
        let end = (start + page_size).min(view.len());

        // Render the current page in a sortable grid.
        let mut grid = DataGrid::new(id_source);
        for column in &columns {
            grid = if column.sortable {
                grid.sortable_column(column.label.clone())
            } else {
                grid.column(column.label.clone())
            };
        }
        for row in &view[start..end] {
            grid = grid.row((*row).clone());
        }
        grid.sort(&mut state.sort).show(ui);

        if total_pages > 1 {
            ui.add_space(gap);
            if let Some(page) = Pagination::new(state.page, total_pages).show(ui) {
                state.page = page;
            }
        }
    }
}
