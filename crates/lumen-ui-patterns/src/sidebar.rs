//! [`Sidebar`] — a vertical navigation list bound to a selected index.

use egui::{Align, Layout, Ui};
use lumen_ui_widgets::Button;

/// A vertical nav: full-width entries, the selected one styled as primary. Bind
/// it to a `&mut usize`; clicking an entry updates it.
#[derive(Debug)]
pub struct Sidebar<'a> {
    selected: &'a mut usize,
    items: Vec<String>,
}

impl<'a> Sidebar<'a> {
    #[must_use]
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            items: Vec::new(),
        }
    }

    #[must_use]
    pub fn item(mut self, label: impl Into<String>) -> Self {
        self.items.push(label.into());
        self
    }

    /// Draw the nav. Updates the bound index on click.
    pub fn show(self, ui: &mut Ui) {
        ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
            for (index, label) in self.items.iter().enumerate() {
                let widget = if index == *self.selected {
                    Button::primary(label.clone())
                } else {
                    Button::ghost(label.clone())
                };
                if ui.add(widget).clicked() {
                    *self.selected = index;
                }
            }
        });
    }
}
