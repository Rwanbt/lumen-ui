//! [`Timeline`] — vertical chronological events: a dot per event linked by a connector line.

use egui::{pos2, Pos2, Response, RichText, Stroke, Ui};
use lumen_ui_core::{TimelineRecipe, UiThemeExt};

/// One timeline event: a title and an optional detail line (e.g. a timestamp or description).
#[derive(Clone, Debug)]
struct Event {
    title: String,
    detail: Option<String>,
}

/// A vertical list of chronological events. Each event is a dot in a left gutter, linked to the
/// next by a connector line, with the title (+ optional detail) to its right. Build with
/// [`Timeline::new`] + [`Timeline::event`] / [`Timeline::event_detailed`], then [`Timeline::show`].
#[derive(Debug, Default)]
pub struct Timeline {
    events: Vec<Event>,
}

impl Timeline {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn event(mut self, title: impl Into<String>) -> Self {
        self.events.push(Event {
            title: title.into(),
            detail: None,
        });
        self
    }

    #[must_use]
    pub fn event_detailed(mut self, title: impl Into<String>, detail: impl Into<String>) -> Self {
        self.events.push(Event {
            title: title.into(),
            detail: Some(detail.into()),
        });
        self
    }

    /// Draw the timeline. Returns the response of the enclosing vertical layout.
    pub fn show(self, ui: &mut Ui) -> Response {
        let recipe = TimelineRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let gutter = recipe.dot_radius * 2.0 + ui.spacing().item_spacing.x;

        ui.vertical(|ui| {
            // Connect each dot to the previous one. Drawing the segment when we reach the current
            // dot (knowing the previous) needs only a single pass and is robust to row heights.
            let mut prev_dot: Option<Pos2> = None;
            for event in &self.events {
                let row = ui
                    .horizontal_top(|ui| {
                        ui.add_space(gutter);
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(&event.title)
                                    .color(recipe.title_color)
                                    .size(recipe.title_size),
                            );
                            if let Some(detail) = &event.detail {
                                ui.label(
                                    RichText::new(detail)
                                        .color(recipe.detail_color)
                                        .size(recipe.detail_size),
                                );
                            }
                        });
                    })
                    .response;

                let rect = row.rect;
                let dot = pos2(
                    rect.left() + recipe.dot_radius,
                    rect.top() + recipe.dot_radius,
                );
                let painter = ui.painter();
                if let Some(previous) = prev_dot {
                    painter.line_segment(
                        [previous, dot],
                        Stroke::new(recipe.line_width, recipe.line_color),
                    );
                }
                painter.circle_filled(dot, recipe.dot_radius, recipe.dot_color);
                prev_dot = Some(dot);
            }
        })
        .response
    }
}
