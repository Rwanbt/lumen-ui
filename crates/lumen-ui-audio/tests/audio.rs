//! Functional tests for the audio **displays** via `egui_kittest` (headless).
//! The generic controls (Knob/Fader/XyPad/Transport) now live in `lumen-ui-widgets` and are
//! tested there.

use std::sync::Arc;

use egui_kittest::Harness;
use lumen_ui_audio::{LevelBar, VuMeter, Waveform};
use lumen_ui_core::{install, DarkTheme, LightTheme, Theme, UiContext};

fn theme_ctx(ctx: &egui::Context, theme: &Arc<dyn Theme>) {
    install(ctx, theme.clone(), UiContext::default());
}

#[test]
fn displays_render_under_dark_and_light() {
    let themes: [Arc<dyn Theme>; 2] = [Arc::new(DarkTheme::new()), Arc::new(LightTheme::new())];
    for theme in themes {
        let samples: Vec<f32> = (0..256).map(|i| (i as f32 * 0.1).sin()).collect();
        let mut harness = Harness::new_ui(move |ui| {
            theme_ctx(ui.ctx(), &theme);
            ui.add(VuMeter::new(0.7).peak(0.9));
            ui.add(LevelBar::new(0.4));
            ui.add(Waveform::new(&samples));
        });
        harness.run(); // a panic here fails the test
    }
}
