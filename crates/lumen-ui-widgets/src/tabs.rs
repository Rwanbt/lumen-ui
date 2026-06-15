//! [`Tabs`] — a headless tab bar whose selection lives in `egui` memory.
//!
//! No external boolean/index to manage: the selected tab is persisted in
//! `ctx.data` under a caller-provided id. `show` returns the selected index so
//! the caller can render the matching content.

use std::hash::Hash;

use egui::{Id, Ui};

use crate::button::Button;

/// A row of tabs. Build with [`Tabs::new`] + [`Tabs::tab`], then call [`Tabs::show`].
#[derive(Clone, Debug)]
pub struct Tabs {
    id: Id,
    labels: Vec<String>,
}

impl Tabs {
    /// `id_source` must be stable and unique within the parent `Ui`.
    #[must_use]
    pub fn new(id_source: impl Hash) -> Self {
        Self {
            id: Id::new(id_source),
            labels: Vec::new(),
        }
    }

    #[must_use]
    pub fn tab(mut self, label: impl Into<String>) -> Self {
        self.labels.push(label.into());
        self
    }

    /// Draw the tab bar and return the selected index (persisted in `ctx.data`).
    pub fn show(self, ui: &mut Ui) -> usize {
        let mut selected: usize = ui.ctx().data_mut(|d| d.get_temp(self.id).unwrap_or(0));
        selected = selected.min(self.labels.len().saturating_sub(1));

        ui.horizontal(|ui| {
            for (index, label) in self.labels.iter().enumerate() {
                // The selected tab reads as a primary button; the rest as ghosts.
                let widget = if index == selected {
                    Button::primary(label.clone())
                } else {
                    Button::ghost(label.clone())
                };
                if ui.add(widget).clicked() {
                    selected = index;
                }
            }
        });

        ui.ctx().data_mut(|d| d.insert_temp(self.id, selected));
        selected
    }
}
