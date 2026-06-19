//! lumen-ui WASM gallery — a small showcase that runs both natively and in the browser.
//!
//! - Native: `cargo run` (in this `web/` directory).
//! - Web: `trunk serve` / `trunk build --release` (Trunk reads `index.html`).

use std::sync::Arc;

use eframe::egui;
use lumen_ui::prelude::*;

fn setup(ctx: &egui::Context) {
    install(ctx, Arc::new(DarkTheme::new()), UiContext::default());
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "lumen-ui — gallery",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            setup(&cc.egui_ctx);
            Ok(Box::<Gallery>::default())
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        let canvas = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document")
            .get_element_by_id("lumen_canvas")
            .expect("missing #lumen_canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("#lumen_canvas is not a canvas");
        eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| {
                    setup(&cc.egui_ctx);
                    Ok(Box::<Gallery>::default())
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}

#[derive(Default)]
struct Gallery {
    theme: usize,
    clicks: u32,
    notify: bool,
    knob: f32,
    gain: f32,
    level: f32,
    x: f32,
    y: f32,
    playing: bool,
    recording: bool,
}

fn theme_for(index: usize) -> Arc<dyn Theme> {
    match index {
        1 => Arc::new(LightTheme::new()),
        2 => Arc::new(nord()),
        3 => Arc::new(solarized_dark()),
        4 => Arc::new(seno_night()),
        5 => Arc::new(seno_dawn()),
        _ => Arc::new(DarkTheme::new()),
    }
}

impl eframe::App for Gallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.add(Heading::display("lumen-ui"));
        ui.add(Label::muted("A token-driven, themeable design system for egui — in your browser."));
        ui.add_space(10.0);

        let before = self.theme;
        SegmentedControl::new(&mut self.theme)
            .segment("Dark")
            .segment("Light")
            .segment("Nord")
            .segment("Solarized")
            .segment("Seno")
            .segment("Dawn")
            .show(ui);
        if self.theme != before {
            set_theme(ui.ctx(), theme_for(self.theme));
        }

        ui.add_space(12.0);
        Card::new().show(ui, |ui| {
            ui.add(Heading::new("Widgets"));
            ui.horizontal(|ui| {
                if ui.add(Button::primary("Primary")).clicked() {
                    self.clicks += 1;
                }
                ui.add(Button::secondary("Secondary"));
                ui.add(Button::ghost("Ghost"));
                ui.add(Button::danger("Danger"));
            });
            ui.horizontal(|ui| {
                ui.add(Switch::new(&mut self.notify));
                ui.add(Label::new("Notifications"));
                ui.add(Badge::success("OK"));
                ui.add(Badge::warning("Beta"));
            });
            ui.add(Label::muted(format!("clicks: {}", self.clicks)));
        });

        ui.add_space(12.0);
        Card::new().show(ui, |ui| {
            ui.add(Heading::new("Audio"));
            ui.horizontal(|ui| {
                ui.add(Knob::new(&mut self.knob, 0.0..=1.0));
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
            ui.add(Label::muted("Drive the meters:"));
            ui.add(Slider::new(&mut self.level, 0.0..=1.0));
        });
    }
}
