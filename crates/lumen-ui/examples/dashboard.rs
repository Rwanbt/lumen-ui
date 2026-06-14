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
    name: String,
    volume: f32,
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
                    if ui.add(Button::ghost("Commands")).clicked() {
                        open_command_palette(ui.ctx(), "cmd_palette");
                    }
                });
            })
            .sidebar(|ui| {
                ui.add(Heading::new("Nav"));
                ui.add_space(6.0);
                Sidebar::new(&mut self.section)
                    .item(SECTIONS[0])
                    .item(SECTIONS[1])
                    .item(SECTIONS[2])
                    .show(ui);
            })
            .inspector(|ui| {
                InspectorPanel::new("Inspector").show(ui, |ui| {
                    property_row(ui, "Notifications", |ui| {
                        ui.add(Switch::new(&mut self.notify));
                    });
                });
            })
            .status_bar(move |ui| {
                StatusBar::new().show(ui, |ui| {
                    ui.add(Label::muted(format!("Section: {}", SECTIONS[section])));
                });
            })
            .show(ui, |ui| match section {
                1 => {
                    ui.add(Heading::display("Reports"));
                    ui.add_space(8.0);
                    let logs = [
                        LogEntry::info("Server started on :8080"),
                        LogEntry::debug("Cache warmed (1280 entries)"),
                        LogEntry::warn("Slow query: 412ms"),
                        LogEntry::error("Upstream timeout (retrying)"),
                    ];
                    LogPanel::new().show(ui, &logs);
                }
                2 => {
                    SettingsPage::new("Settings").show(ui, |ui| {
                        property_row(ui, "Name", |ui| {
                            ui.add(TextField::new(&mut self.name).hint("Your name"));
                        });
                        property_row(ui, "Volume", |ui| {
                            ui.add(Slider::new(&mut self.volume, 0.0..=1.0));
                        });
                    });
                }
                _ => {
                    ui.add(Heading::display("Overview"));
                    ui.add(Label::muted("Central content area."));
                }
            });

        // Command palette overlay — opening/closing is state-free for the caller.
        if let Some(index) = CommandPalette::new("cmd_palette")
            .command("Go to Overview")
            .command("Go to Reports")
            .command("Go to Settings")
            .show(ui.ctx())
        {
            self.section = index;
        }
    }
}
