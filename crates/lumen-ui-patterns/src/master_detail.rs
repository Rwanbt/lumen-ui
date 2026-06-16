//! [`MasterDetail`] — a two-pane list/detail layout.
//!
//! A resizable left pane lists items (rendered with the themed [`Sidebar`] nav, bound to a
//! `&mut usize`); the central pane shows the detail of the selected item. Built on egui
//! 0.34's `Panel`/`CentralPanel` (`show_inside`), so it nests inside the `&mut Ui` eframe
//! hands to `App::ui` — the same convention as [`crate::DashboardLayout`].

use egui::{CentralPanel, Panel, Ui};

use crate::Sidebar;

const LIST_DEFAULT_WIDTH: f32 = 240.0;

/// A list/detail split bound to a selected index.
///
/// ```ignore
/// MasterDetail::new(&mut selected)
///     .item("Inbox")
///     .item("Sent")
///     .show(ui, |ui, index| { ui.label(format!("Detail for item {index}")); });
/// ```
#[derive(Debug)]
pub struct MasterDetail<'a> {
    selected: &'a mut usize,
    items: Vec<String>,
    list_width: f32,
    resizable: bool,
}

impl<'a> MasterDetail<'a> {
    #[must_use]
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            items: Vec::new(),
            list_width: LIST_DEFAULT_WIDTH,
            resizable: true,
        }
    }

    /// Append one list entry.
    #[must_use]
    pub fn item(mut self, label: impl Into<String>) -> Self {
        self.items.push(label.into());
        self
    }

    /// Set the initial width of the list pane (default: 240 pt).
    #[must_use]
    pub fn list_width(mut self, width: f32) -> Self {
        self.list_width = width;
        self
    }

    /// Allow the user to drag the divider to resize the list pane (default: `true`).
    #[must_use]
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Draw the split. `detail` receives the `&mut Ui` of the central pane and the
    /// currently selected index. Pass the `&mut Ui` from `eframe::App::ui`.
    pub fn show(self, ui: &mut Ui, detail: impl FnOnce(&mut Ui, usize)) {
        let MasterDetail {
            selected,
            items,
            list_width,
            resizable,
        } = self;

        Panel::left("lumen_master_detail_list")
            .resizable(resizable)
            .default_size(list_width)
            .show_inside(ui, |ui| {
                // Reborrow so `selected` stays readable for the detail pane below.
                let mut nav = Sidebar::new(&mut *selected);
                for label in items {
                    nav = nav.item(label);
                }
                nav.show(ui);
            });

        let chosen = *selected;
        CentralPanel::default().show_inside(ui, |ui| detail(ui, chosen));
    }
}
