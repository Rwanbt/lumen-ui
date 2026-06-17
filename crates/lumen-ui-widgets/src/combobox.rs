//! [`Combobox`] — a searchable [`crate::Select`]: type to filter the options.

use std::hash::Hash;

use egui::{ComboBox, Id, Popup, PopupCloseBehavior, Response, Ui};

use crate::TextField;

/// A dropdown selecting one value of `T`, with a search field that filters the options by a
/// case-insensitive substring. Build with [`Combobox::new`] + [`Combobox::option`], then
/// [`Combobox::show`].
///
/// The open state is owned by the underlying [`egui::ComboBox`] (egui memory); the transient
/// search query is stored in `ctx.data` keyed by the combobox id. The selected value is the
/// caller's (`&mut T`).
#[derive(Debug)]
pub struct Combobox<'a, T> {
    id: Id,
    selected: &'a mut T,
    options: Vec<(T, String)>,
    hint: String,
}

impl<'a, T: PartialEq + Clone> Combobox<'a, T> {
    /// `id_source` must be stable and unique within the parent `Ui`.
    #[must_use]
    pub fn new(id_source: impl Hash, selected: &'a mut T) -> Self {
        Self {
            id: Id::new(id_source),
            selected,
            options: Vec::new(),
            hint: String::from("Search…"),
        }
    }

    #[must_use]
    pub fn option(mut self, value: T, label: impl Into<String>) -> Self {
        self.options.push((value, label.into()));
        self
    }

    /// Override the search field placeholder (default `"Search…"`).
    #[must_use]
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = hint.into();
        self
    }

    /// Draw the combobox. Returns the trigger button's response.
    pub fn show(self, ui: &mut Ui) -> Response {
        let current = self
            .options
            .iter()
            .find(|(value, _)| value == &*self.selected)
            .map(|(_, label)| label.clone())
            .unwrap_or_default();

        let query_id = self.id.with("query");
        let mut query: String = ui.data(|d| d.get_temp(query_id).unwrap_or_default());

        // CloseOnClickOutside (not the default CloseOnClick) so clicking the search field to type
        // does not dismiss the popup; we close it ourselves once a value is picked (below).
        let inner = ComboBox::from_id_salt(self.id)
            .selected_text(current)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .show_ui(ui, |ui| {
                ui.add(TextField::new(&mut query).hint(self.hint.as_str()));
                let needle = query.to_lowercase();
                let mut picked = false;
                for (value, label) in &self.options {
                    let matches = needle.is_empty() || label.to_lowercase().contains(&needle);
                    // Short-circuit: filtered-out options are never rendered.
                    if matches
                        && ui
                            .selectable_value(self.selected, value.clone(), label.clone())
                            .clicked()
                    {
                        picked = true;
                    }
                }
                picked
            });

        ui.data_mut(|d| d.insert_temp(query_id, query));

        if inner.inner == Some(true) {
            // egui's `ComboBox::widget_to_popup_id` is private; it is `button_id.with("popup")`
            // (verified against egui 0.34.3 source — re-check on egui upgrade). `inner.response.id`
            // IS that button id, so this targets exactly this combobox's popup.
            Popup::close_id(ui.ctx(), inner.response.id.with("popup"));
        }
        inner.response
    }
}
