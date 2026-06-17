//! [`Calendar`] — a month grid that selects a day, bound to a `&mut Date`.

use std::hash::Hash;

use egui::{Align2, FontId, Grid, Id, Response, RichText, Sense, Ui, Vec2, WidgetInfo, WidgetType};
use lumen_ui_core::{day_of_week, days_in_month, month_name, CalendarRecipe, Date, UiThemeExt};

/// Weekday column labels, Monday-first (matching [`lumen_ui_core::day_of_week`]).
const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
/// Header-arrow glyph font size as a fraction of the cell size.
const ARROW_GLYPH_RATIO: f32 = 0.7;

/// A month calendar bound to a `&mut Date`. Clicking a day sets the bound date; the prev/next
/// header arrows page the *displayed* month (kept in `ctx.data`) without changing the selection
/// until a day is clicked. Build with [`Calendar::new`], then [`Calendar::show`].
#[derive(Debug)]
pub struct Calendar<'a> {
    id: Id,
    selected: &'a mut Date,
}

impl<'a> Calendar<'a> {
    /// `id_source` must be stable and unique within the parent `Ui`.
    #[must_use]
    pub fn new(id_source: impl Hash, selected: &'a mut Date) -> Self {
        Self {
            id: Id::new(id_source),
            selected,
        }
    }

    /// Draw the calendar. Returns the response of the enclosing layout.
    pub fn show(self, ui: &mut Ui) -> Response {
        let recipe = CalendarRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());

        // The displayed month is independent of the selection so the user can browse months.
        let view_key = self.id.with("view");
        let mut view: (i32, u32) = ui
            .data(|d| d.get_temp(view_key))
            .unwrap_or((self.selected.year, self.selected.month));

        let response = ui
            .vertical(|ui| {
                ui.horizontal(|ui| {
                    if arrow(ui, &recipe, "‹", "previous month").clicked() {
                        let d = Date::new(view.0, view.1, 1).previous_month();
                        view = (d.year, d.month);
                    }
                    ui.label(
                        RichText::new(format!("{} {}", month_name(view.1), view.0))
                            .color(recipe.header_color)
                            .size(recipe.header_size),
                    );
                    if arrow(ui, &recipe, "›", "next month").clicked() {
                        let d = Date::new(view.0, view.1, 1).next_month();
                        view = (d.year, d.month);
                    }
                });

                Grid::new(self.id.with("grid"))
                    .num_columns(7)
                    .show(ui, |ui| {
                        for weekday in WEEKDAYS {
                            ui.label(
                                RichText::new(weekday)
                                    .color(recipe.weekday_color)
                                    .size(recipe.label_size),
                            );
                        }
                        ui.end_row();

                        let leading = day_of_week(Date::new(view.0, view.1, 1));
                        let total = days_in_month(view.0, view.1);
                        let mut column = 0u32;
                        for _ in 0..leading {
                            ui.allocate_exact_size(Vec2::splat(recipe.cell_size), Sense::hover());
                            column += 1;
                        }
                        for day in 1..=total {
                            let is_selected = self.selected.year == view.0
                                && self.selected.month == view.1
                                && self.selected.day == day;
                            if day_cell(ui, &recipe, day, is_selected).clicked() {
                                *self.selected = Date::new(view.0, view.1, day);
                            }
                            column += 1;
                            if column == 7 {
                                ui.end_row();
                                column = 0;
                            }
                        }
                    });
            })
            .response;

        ui.data_mut(|d| d.insert_temp(view_key, view));
        response
    }
}

/// Draw a themed prev/next month arrow and return its click [`Response`].
fn arrow(ui: &mut Ui, recipe: &CalendarRecipe, glyph: &str, label: &str) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(recipe.cell_size), Sense::click());
    let enabled = ui.is_enabled();
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        glyph,
        FontId::proportional(recipe.cell_size * ARROW_GLYPH_RATIO),
        recipe.header_color,
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, label));
    response
}

/// Draw one day cell (selected days get a filled background) and return its click [`Response`].
/// The day number is the screen-reader label, which also makes the cell queryable in tests.
fn day_cell(ui: &mut Ui, recipe: &CalendarRecipe, day: u32, selected: bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(recipe.cell_size), Sense::click());
    let enabled = ui.is_enabled();
    let painter = ui.painter();
    if selected {
        painter.rect_filled(rect, recipe.corner_radius, recipe.selected_fill);
    }
    let text_color = if selected {
        recipe.selected_text
    } else {
        recipe.day_color
    };
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        day.to_string(),
        FontId::proportional(recipe.label_size),
        text_color,
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, day.to_string()));
    response
}
