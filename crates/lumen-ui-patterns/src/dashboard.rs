//! [`DashboardLayout`] — the classic app shell: optional toolbar, status bar,
//! sidebar, and inspector around a central content area.
//!
//! Built on egui 0.34's `Panel` (nested via `show_inside`, themed by the installed
//! theme's global visuals). Call it with the `&mut Ui` eframe hands to `App::ui`.

use egui::{CentralPanel, Panel, Ui};

type Region<'a> = Box<dyn FnOnce(&mut Ui) + 'a>;

const SIDEBAR_DEFAULT_WIDTH: f32 = 220.0;
const INSPECTOR_DEFAULT_WIDTH: f32 = 280.0;

/// App-shell layout. Set the regions you need, then [`DashboardLayout::show`].
#[derive(Default)]
pub struct DashboardLayout<'a> {
    toolbar: Option<Region<'a>>,
    status_bar: Option<Region<'a>>,
    sidebar: Option<Region<'a>>,
    inspector: Option<Region<'a>>,
}

impl<'a> DashboardLayout<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Top bar, full width.
    #[must_use]
    pub fn toolbar(mut self, content: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.toolbar = Some(Box::new(content));
        self
    }

    /// Bottom bar, full width.
    #[must_use]
    pub fn status_bar(mut self, content: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.status_bar = Some(Box::new(content));
        self
    }

    /// Left panel (resizable).
    #[must_use]
    pub fn sidebar(mut self, content: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.sidebar = Some(Box::new(content));
        self
    }

    /// Right panel (resizable).
    #[must_use]
    pub fn inspector(mut self, content: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.inspector = Some(Box::new(content));
        self
    }

    /// Show the shell with `content` in the central area. Pass the `&mut Ui` from
    /// `eframe::App::ui`. Call once per frame.
    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
        // Order matters: top/bottom first, then sides, then central.
        if let Some(toolbar) = self.toolbar {
            Panel::top("lumen_dashboard_toolbar").show_inside(ui, toolbar);
        }
        if let Some(status_bar) = self.status_bar {
            Panel::bottom("lumen_dashboard_status").show_inside(ui, status_bar);
        }
        if let Some(sidebar) = self.sidebar {
            Panel::left("lumen_dashboard_sidebar")
                .resizable(true)
                .default_size(SIDEBAR_DEFAULT_WIDTH)
                .show_inside(ui, sidebar);
        }
        if let Some(inspector) = self.inspector {
            Panel::right("lumen_dashboard_inspector")
                .resizable(true)
                .default_size(INSPECTOR_DEFAULT_WIDTH)
                .show_inside(ui, inspector);
        }
        CentralPanel::default().show_inside(ui, content);
    }
}
