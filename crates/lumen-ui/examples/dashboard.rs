//! `dashboard` — the app-shell pattern (v0.6).
//!
//! A toolbar, left sidebar nav, right inspector, status bar, and central content
//! — wired in ~20 lines. Driven from `App::update` so the panels are top-level.
//!
//! Run with: `cargo run -p lumen-ui --example dashboard --features patterns`

use std::sync::Arc;

use eframe::egui;
use lumen_ui::prelude::*;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "lumen-ui — dashboard",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            install(
                &cc.egui_ctx,
                Arc::new(DarkTheme::new()),
                UiContext::default(),
            );
            Ok(Box::<App>::default())
        }),
    )
}

#[derive(Default)]
struct App {
    section: usize,
    notify: bool,
}

const SECTIONS: [&str; 3] = ["Overview", "Reports", "Settings"];

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Read-only copy so the status-bar/central closures don't borrow `self`
        // (only `sidebar` and `inspector` mutate disjoint fields).
        let section = self.section;
        DashboardLayout::new()
            .toolbar(|ui| {
                Toolbar::new().show(ui, |ui| {
                    ui.add(Heading::new("Acme"));
                    ui.add(Badge::primary("v0.6"));
                });
            })
            .sidebar(|ui| {
                ui.add(Heading::new("Nav"));
                ui.add_space(6.0);
                for (index, name) in SECTIONS.iter().enumerate() {
                    let widget = if index == self.section {
                        Button::primary(*name)
                    } else {
                        Button::ghost(*name)
                    };
                    if ui.add(widget).clicked() {
                        self.section = index;
                    }
                }
            })
            .inspector(|ui| {
                ui.add(Heading::new("Inspector"));
                ui.add(Label::muted("Properties for the selection."));
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.add(Switch::new(&mut self.notify));
                    ui.add(Label::new("Notifications"));
                });
            })
            .status_bar(move |ui| {
                StatusBar::new().show(ui, |ui| {
                    ui.add(Label::muted(format!("Section: {}", SECTIONS[section])));
                });
            })
            .show(ui, move |ui| {
                ui.add(Heading::display(SECTIONS[section]));
                ui.add(Label::muted("Central content area."));
            });
    }
}
