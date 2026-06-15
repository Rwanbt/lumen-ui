//! `icons` — the lumen-ui-icons set (v0.7).
//!
//! Run with: `cargo run -p lumen-ui --example icons --features icons`

use std::sync::Arc;

use eframe::egui;
use lumen_ui::prelude::*;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "lumen-ui — icons",
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
struct App;

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("lumen-ui-icons");
        ui.add_space(12.0);
        let icons = [
            (Icon::check(), "check"),
            (Icon::close(), "close"),
            (Icon::chevron_down(), "chevron_down"),
            (Icon::chevron_right(), "chevron_right"),
            (Icon::plus(), "plus"),
            (Icon::minus(), "minus"),
            (Icon::search(), "search"),
            (Icon::menu(), "menu"),
        ];
        for (icon, name) in icons {
            ui.horizontal(|ui| {
                ui.add(icon.size(24.0));
                ui.label(name);
            });
        }
    }
}
