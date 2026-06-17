//! [`MultiSelect`] — pick several values of `T`, bound to a `&mut Vec<T>`.

use std::hash::Hash;

use egui::{ComboBox, Id, PopupCloseBehavior, Response, Ui};

/// A dropdown selecting any number of values of `T`, bound to the caller's `&mut Vec<T>`
/// (insertion order preserved). Build with [`MultiSelect::new`] + [`MultiSelect::option`], then
/// [`MultiSelect::show`]. The open state is owned by the underlying [`egui::ComboBox`] (egui
/// memory); the popup stays open while toggling and closes on a click outside.
#[derive(Debug)]
pub struct MultiSelect<'a, T> {
    id: Id,
    selected: &'a mut Vec<T>,
    options: Vec<(T, String)>,
}

impl<'a, T: PartialEq + Clone> MultiSelect<'a, T> {
    /// `id_source` must be stable and unique within the parent `Ui`.
    #[must_use]
    pub fn new(id_source: impl Hash, selected: &'a mut Vec<T>) -> Self {
        Self {
            id: Id::new(id_source),
            selected,
            options: Vec::new(),
        }
    }

    #[must_use]
    pub fn option(mut self, value: T, label: impl Into<String>) -> Self {
        self.options.push((value, label.into()));
        self
    }

    /// Draw the multi-select. Returns the trigger button's response.
    pub fn show(self, ui: &mut Ui) -> Response {
        let summary = match self.selected.len() {
            0 => String::from("None"),
            n => format!("{n} selected"),
        };

        // CloseOnClickOutside so the popup stays open while the user toggles several options.
        ComboBox::from_id_salt(self.id)
            .selected_text(summary)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .show_ui(ui, |ui| {
                for (value, label) in &self.options {
                    let mut checked = self.selected.contains(value);
                    if ui.selectable_label(checked, label.clone()).clicked() {
                        checked = !checked;
                        if checked {
                            self.selected.push(value.clone());
                        } else {
                            self.selected.retain(|v| v != value);
                        }
                    }
                }
            })
            .response
    }
}
