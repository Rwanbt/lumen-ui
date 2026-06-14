//! `minimal` — the v0.1 validation example.
//!
//! Demonstrates the whole point of lumen-ui in one screen: installing a theme,
//! drawing themed widgets, and **swapping the theme live** so the entire app
//! restyles without touching any widget code.
//!
//! Run with: `cargo run -p lumen-ui --example minimal`

use std::sync::Arc;

use eframe::egui;
use lumen_ui::prelude::*;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "lumen-ui — minimal",
        options,
        Box::new(|cc| {
            install(
                &cc.egui_ctx,
                Arc::new(DarkTheme::new()),
                UiContext::default(),
            );
            Ok(Box::<DemoApp>::default())
        }),
    )
}

#[derive(Default)]
struct DemoApp {
    clicks: u32,
}

impl eframe::App for DemoApp {
    // eframe 0.34: `ui` is the required method; eframe provides the central panel.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.add(Heading::display("lumen-ui"));
        ui.add(Label::muted(
            "A token-driven, themeable design system for egui.",
        ));
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            if ui.add(Button::primary("Primary")).clicked() {
                self.clicks += 1;
            }
            if ui.add(Button::secondary("Secondary")).clicked() {
                self.clicks += 1;
            }
            ui.add(Button::ghost("Ghost"));
            if ui.add(Button::danger("Danger")).clicked() {
                self.clicks += 1;
            }
            ui.add(Button::primary("Disabled").enabled(false));
        });

        ui.add_space(8.0);
        ui.add(Label::new(format!("clicks: {}", self.clicks)));
    }
}
