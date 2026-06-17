//! `catalogue` — showcases the v2 catalogue: the audio crate + theme presets, switched live.
//!
//! Run with: `cargo run -p lumen-ui --example catalogue --features audio,themes`

use std::sync::Arc;

use eframe::egui;
use lumen_ui::prelude::*;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "lumen-ui — catalogue",
        options,
        Box::new(|cc| {
            install(
                &cc.egui_ctx,
                Arc::new(DarkTheme::new()),
                UiContext::default(),
            );
            Ok(Box::<CatalogueApp>::default())
        }),
    )
}

#[derive(Default)]
struct CatalogueApp {
    theme: usize,
    cutoff: f32,
    gain: f32,
    level: f32,
    x: f32,
    y: f32,
    playing: bool,
    recording: bool,
}

/// The theme for the selected segment. Index 4 derives a custom theme from two seeds (v1.9).
fn theme_for(index: usize) -> Arc<dyn Theme> {
    match index {
        1 => Arc::new(LightTheme::new()),
        2 => Arc::new(nord()),
        3 => Arc::new(solarized_dark()),
        4 => Arc::new(
            ThemeBuilder::new(
                egui::Color32::from_rgb(0x14, 0x10, 0x1c),
                egui::Color32::from_rgb(0xc9, 0x7b, 0xff),
            )
            .build(),
        ),
        _ => Arc::new(DarkTheme::new()),
    }
}

impl eframe::App for CatalogueApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.add(Heading::display("lumen-ui — catalogue"));
        ui.add(Label::muted("Audio controls + live theme presets."));
        ui.add_space(12.0);

        let before = self.theme;
        SegmentedControl::new(&mut self.theme)
            .segment("Dark")
            .segment("Light")
            .segment("Nord")
            .segment("Solarized")
            .segment("Custom")
            .show(ui);
        if self.theme != before {
            set_theme(ui.ctx(), theme_for(self.theme));
        }

        ui.add_space(12.0);
        Card::new().show(ui, |ui| {
            ui.add(Heading::new("Audio"));
            ui.horizontal(|ui| {
                ui.add(Knob::new(&mut self.cutoff, 0.0..=1.0));
                ui.add(Fader::new(&mut self.gain, -60.0..=6.0));
                ui.add(VuMeter::new(self.level).peak((self.level + 0.1).min(1.0)));
                ui.add(XyPad::new(&mut self.x, &mut self.y, 0.0..=1.0, 0.0..=1.0));
            });
            ui.add(LevelBar::new(self.level));
            if let Some(action) = Transport::new()
                .playing(self.playing)
                .recording(self.recording)
                .show(ui)
            {
                match action {
                    TransportAction::PlayPause => self.playing = !self.playing,
                    TransportAction::Stop => self.playing = false,
                    TransportAction::Record => self.recording = !self.recording,
                }
            }
            ui.add_space(8.0);
            ui.add(Label::muted("Drive the meters:"));
            ui.add(Slider::new(&mut self.level, 0.0..=1.0));
            let wave: Vec<f32> = (0..256)
                .map(|i| (i as f32 * 0.12).sin() * self.level)
                .collect();
            ui.add(Waveform::new(&wave));
        });
    }
}
