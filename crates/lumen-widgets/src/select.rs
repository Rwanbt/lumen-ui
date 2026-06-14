//! [`Select`] — a themed dropdown bound to a `&mut T` (over `egui::ComboBox`).

use std::hash::Hash;

use egui::{ComboBox, Id, Response, Ui};

/// A dropdown selecting one value of `T`. Build with [`Select::new`] +
/// [`Select::option`], then [`Select::show`].
#[derive(Debug)]
pub struct Select<'a, T> {
    id: Id,
    selected: &'a mut T,
    options: Vec<(T, String)>,
}

impl<'a, T: PartialEq + Clone> Select<'a, T> {
    /// `id_source` must be stable and unique within the parent `Ui`.
    #[must_use]
    pub fn new(id_source: impl Hash, selected: &'a mut T) -> Self {
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

    /// Draw the dropdown. Returns the combo box's response.
    pub fn show(self, ui: &mut Ui) -> Response {
        let current = self
            .options
            .iter()
            .find(|(value, _)| value == &*self.selected)
            .map(|(_, label)| label.clone())
            .unwrap_or_default();

        ComboBox::from_id_salt(self.id)
            .selected_text(current)
            .show_ui(ui, |ui| {
                for (value, label) in &self.options {
                    ui.selectable_value(self.selected, value.clone(), label.clone());
                }
            })
            .response
    }
}
