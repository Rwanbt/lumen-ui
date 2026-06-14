//! `responsive` — flexbox layout + breakpoints (v0.4).
//!
//! Resize the window: the toolbar stays justified edge-to-edge, and the card row
//! changes its column count by breakpoint — no manual layout code.
//!
//! Run with: `cargo run -p lumen-ui --example responsive --features layout`

use std::sync::Arc;

use eframe::egui;
use lumen_ui::prelude::*;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "lumen-ui — responsive",
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
        let bp = responsive(ui, |b| b);

        Flex::row()
            .fill_width()
            .justify(Justify::SpaceBetween)
            .align(Align::Center)
            .show(ui, "toolbar", |t| {
                t.item(|ui| {
                    ui.add(Heading::new("lumen-ui"));
                });
                t.item(|ui| {
                    ui.add(Button::primary("Action"));
                });
            });

        ui.add_space(12.0);
        ui.add(Label::muted(format!("breakpoint: {bp:?}")));
        ui.add_space(8.0);

        let columns = match bp {
            Breakpoint::Xs => 1,
            Breakpoint::Sm | Breakpoint::Md => 2,
            Breakpoint::Lg | Breakpoint::Xl => 3,
        };

        Flex::row().gap(8.0).fill_width().show(ui, "cards", |t| {
            for index in 0..columns {
                t.item_grow(|ui| {
                    Card::new().show(ui, |ui| {
                        ui.add(Heading::new(format!("Card {}", index + 1)));
                        ui.add(Label::muted("Grows to share the row."));
                    });
                });
            }
        });
    }
}
