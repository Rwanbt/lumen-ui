//! `motion` — spring physics + a fade transition (v0.5).
//!
//! Click the button: a bar springs to its new width and a panel fades in/out.
//!
//! Run with: `cargo run -p lumen-ui --example motion --features motion`

use std::sync::Arc;

use eframe::egui;
use lumen_ui::prelude::*;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "lumen-ui — motion",
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
    open: bool,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.add(Heading::new("Motion"));
        if ui
            .add(Button::primary(if self.open {
                "Collapse"
            } else {
                "Expand"
            }))
            .clicked()
        {
            self.open = !self.open;
        }
        ui.add_space(12.0);

        // Spring-animated bar width.
        let width = Spring::SMOOTH.animate(
            ui.ctx(),
            egui::Id::new("bar"),
            if self.open { 260.0 } else { 48.0 },
        );
        let fill = ui.theme().tokens().colors.primary;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 16.0), egui::Sense::hover());
        ui.painter().rect_filled(rect, 8.0, fill);

        ui.add_space(12.0);

        // Fade transition: the panel only lays out while (at all) visible.
        fade(ui, "panel", self.open, |ui| {
            Card::new().show(ui, |ui| {
                ui.add(Heading::new("Details"));
                ui.add(Label::muted("This panel fades in and out."));
            });
        });
    }
}
