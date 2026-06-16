//! [`SegmentedControl`] — a row of mutually-exclusive options (one selected).

use egui::{Button, Color32, Frame, Margin, Response, RichText, Ui};
use lumen_ui_core::{SegmentedRecipe, UiThemeExt};

/// Inner padding of the container track around the segments, in points.
const TRACK_PADDING: i8 = 2;

/// A segmented control: a pill track with several options, exactly one selected.
/// Binds to a `&mut usize`; clicking a segment updates it.
#[derive(Debug)]
pub struct SegmentedControl<'a> {
    selected: &'a mut usize,
    segments: Vec<String>,
}

impl<'a> SegmentedControl<'a> {
    #[must_use]
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            segments: Vec::new(),
        }
    }

    #[must_use]
    pub fn segment(mut self, label: impl Into<String>) -> Self {
        self.segments.push(label.into());
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let recipe = SegmentedRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        Frame::NONE
            .fill(recipe.container_fill)
            .corner_radius(recipe.corner_radius)
            .inner_margin(Margin::same(TRACK_PADDING))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (index, label) in self.segments.iter().enumerate() {
                        let is_selected = index == *self.selected;
                        let fill = if is_selected {
                            recipe.selected_fill
                        } else {
                            Color32::TRANSPARENT
                        };
                        let text_color = if is_selected {
                            recipe.selected_text
                        } else {
                            recipe.text
                        };
                        let segment = Frame::NONE
                            .fill(fill)
                            .corner_radius(recipe.corner_radius)
                            .inner_margin(Margin::symmetric(
                                recipe.inner_margin.x as i8,
                                recipe.inner_margin.y as i8,
                            ))
                            .show(ui, |ui| {
                                ui.add(
                                    Button::new(
                                        RichText::new(label)
                                            .color(text_color)
                                            .size(recipe.text_size),
                                    )
                                    .frame(false),
                                )
                            })
                            .inner;
                        if segment.clicked() {
                            *self.selected = index;
                        }
                    }
                });
            })
            .response
    }
}
