//! Labeled-row patterns: [`property_row`] plus the [`SettingsPage`] and
//! [`InspectorPanel`] containers that frame a column of rows.

use egui::{ScrollArea, Ui};
use lumen_widgets::Heading;

const LABEL_WIDTH: f32 = 140.0;

/// One "label : control" row. The label gets a fixed column; `control` fills the rest.
pub fn property_row<R>(ui: &mut Ui, label: &str, control: impl FnOnce(&mut Ui) -> R) -> R {
    ui.horizontal(|ui| {
        let height = ui.spacing().interact_size.y;
        ui.add_sized([LABEL_WIDTH, height], lumen_widgets::Label::new(label));
        control(ui)
    })
    .inner
}

/// A titled, scrollable settings page. Put [`property_row`] calls in the body.
#[derive(Clone, Debug)]
pub struct SettingsPage {
    title: String,
}

impl SettingsPage {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }

    /// Draw the heading + a vertical scroll area containing `body`.
    pub fn show<R>(self, ui: &mut Ui, body: impl FnOnce(&mut Ui) -> R) -> R {
        ui.add(Heading::new(self.title));
        ui.add_space(8.0);
        ScrollArea::vertical().show(ui, body).inner
    }
}

/// A titled property panel (no scroll) — the inspector counterpart to
/// [`SettingsPage`]. Put [`property_row`] calls in the body.
#[derive(Clone, Debug)]
pub struct InspectorPanel {
    title: String,
}

impl InspectorPanel {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }

    /// Draw the heading + `body`.
    pub fn show<R>(self, ui: &mut Ui, body: impl FnOnce(&mut Ui) -> R) -> R {
        ui.add(Heading::new(self.title));
        ui.add_space(8.0);
        body(ui)
    }
}
