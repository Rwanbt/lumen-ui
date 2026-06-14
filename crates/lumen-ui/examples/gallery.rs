//! `gallery` — every v0.2 widget on one screen, with a **live theme switch**.
//!
//! This is the v0.2 exit criterion: toggling the theme restyles the entire
//! gallery without touching any widget code.
//!
//! Run with: `cargo run -p lumen-ui --example gallery`

use std::sync::Arc;

use eframe::egui;
use lumen_ui::prelude::*;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "lumen-ui — gallery",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            install(
                &cc.egui_ctx,
                Arc::new(DarkTheme::new()),
                UiContext::default(),
            );
            Ok(Box::<Gallery>::default())
        }),
    )
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Plan {
    #[default]
    Free,
    Pro,
    Team,
}

struct Gallery {
    dark: bool,
    name: String,
    agree: bool,
    notify: bool,
    volume: f32,
    plan: Plan,
}

impl Default for Gallery {
    fn default() -> Self {
        Self {
            dark: true,
            name: String::new(),
            agree: false,
            notify: true,
            volume: 0.5,
            plan: Plan::Free,
        }
    }
}

impl eframe::App for Gallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(Heading::display("lumen-ui gallery"));
                let label = if self.dark {
                    "Switch to Light"
                } else {
                    "Switch to Dark"
                };
                if ui.add(Button::ghost(label)).clicked() {
                    self.dark = !self.dark;
                    if self.dark {
                        set_theme(ui.ctx(), Arc::new(DarkTheme::new()));
                    } else {
                        set_theme(ui.ctx(), Arc::new(LightTheme::new()));
                    }
                }
            });
            ui.add(Label::muted(
                "Toggle the theme — the whole gallery restyles, zero widget changes.",
            ));
            ui.add_space(12.0);

            Card::new().show(ui, |ui| {
                ui.add(Heading::new("Buttons"));
                ui.horizontal(|ui| {
                    if ui.add(Button::primary("Primary")).clicked() {
                        toast_success(ui.ctx(), "Primary action done");
                    }
                    ui.add(Button::secondary("Secondary"));
                    ui.add(Button::ghost("Ghost"));
                    if ui.add(Button::danger("Danger")).clicked() {
                        toast_error(ui.ctx(), "Something went wrong");
                    }
                    ui.add(Button::primary("Disabled").enabled(false));
                });
            });
            ui.add_space(8.0);

            Card::new().show(ui, |ui| {
                ui.add(Heading::new("Badges"));
                ui.horizontal(|ui| {
                    ui.add(Badge::new("Neutral"));
                    ui.add(Badge::primary("Primary"));
                    ui.add(Badge::success("Success"));
                    ui.add(Badge::warning("Warning"));
                    ui.add(Badge::danger("Danger"));
                });
            });
            ui.add_space(8.0);

            Card::new().show(ui, |ui| {
                ui.add(Heading::new("Inputs"));
                ui.add(TextField::new(&mut self.name).hint("Your name"));
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.add(Switch::new(&mut self.notify));
                    ui.add(Label::new("Notifications"));
                });
                ui.add(Checkbox::new(&mut self.agree, "I accept the terms"));
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut self.volume, 0.0..=1.0));
                    ui.add(Label::muted(format!("{:.0}%", self.volume * 100.0)));
                });
            });
            ui.add_space(8.0);

            Card::new().show(ui, |ui| {
                ui.add(Heading::new("Plan"));
                ui.add(
                    RadioGroup::new(&mut self.plan)
                        .option(Plan::Free, "Free")
                        .option(Plan::Pro, "Pro")
                        .option(Plan::Team, "Team"),
                );
            });
            ui.add_space(8.0);

            Card::new().show(ui, |ui| {
                ui.add(Heading::new("Tabs"));
                let tab = Tabs::new("gallery_tabs")
                    .tab("Overview")
                    .tab("Details")
                    .tab("Activity")
                    .show(ui);
                ui.add_space(6.0);
                match tab {
                    0 => ui.add(Label::new("Overview content.")),
                    1 => ui.add(Label::new("Detailed content.")),
                    _ => ui.add(Label::new("Recent activity.")),
                };
            });
            ui.add_space(8.0);

            Accordion::new("Advanced settings")
                .default_open(false)
                .show(ui, |ui| {
                    ui.add(Label::muted("Collapsible content, state kept by egui."));
                });
        });

        // Render queued toasts on top, once per frame.
        show_toasts(ui.ctx());
    }
}
