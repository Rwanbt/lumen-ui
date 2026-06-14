//! [`RadioGroup`] — single selection among a list of options.

use egui::{vec2, Response, Sense, Stroke, Ui, Widget};
use lumen_core::{UiThemeExt, WidgetState};

use crate::text::Label;

/// A vertical group of radio buttons bound to a `&mut T`. Build it with
/// [`RadioGroup::new`] then chain [`RadioGroup::option`].
#[derive(Debug)]
pub struct RadioGroup<'a, T> {
    selected: &'a mut T,
    options: Vec<(T, String)>,
}

impl<'a, T: PartialEq + Clone> RadioGroup<'a, T> {
    #[must_use]
    pub fn new(selected: &'a mut T) -> Self {
        Self {
            selected,
            options: Vec::new(),
        }
    }

    #[must_use]
    pub fn option(mut self, value: T, label: impl Into<String>) -> Self {
        self.options.push((value, label.into()));
        self
    }
}

impl<T: PartialEq + Clone> Widget for RadioGroup<'_, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            let mut union: Option<Response> = None;
            for (value, label) in &self.options {
                let row = radio_row(ui, *self.selected == *value, label);
                if row.clicked() {
                    *self.selected = value.clone();
                }
                union = Some(match union.take() {
                    Some(u) => u | row,
                    None => row,
                });
            }
            union.unwrap_or_else(|| ui.allocate_response(vec2(0.0, 0.0), Sense::hover()))
        })
        .inner
    }
}

/// Draw one selectable radio row (circle + label) and return its combined response.
fn radio_row(ui: &mut Ui, selected: bool, label: &str) -> Response {
    ui.horizontal(|ui| {
        let size = ui.spacing().interact_size.y;
        let (rect, response) = ui.allocate_exact_size(vec2(size, size), Sense::click());

        let state = if response.hovered() {
            WidgetState::Hovered
        } else {
            WidgetState::Normal
        };
        let recipe = ui.theme().toggle_recipe(selected, state, &ui.ui_ctx());

        let center = rect.center();
        let radius = size * 0.35;
        let ring = if selected {
            Stroke::new(2.0, recipe.track)
        } else {
            recipe.border
        };
        ui.painter().circle_stroke(center, radius, ring);
        if selected {
            ui.painter()
                .circle_filled(center, radius * 0.5, recipe.track);
        }

        response | ui.add(Label::new(label.to_owned()))
    })
    .inner
}
