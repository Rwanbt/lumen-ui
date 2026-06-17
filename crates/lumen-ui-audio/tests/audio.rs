//! Functional tests for the audio controls via `egui_kittest` (headless).

use std::sync::Arc;

use egui::accesskit::Role;
use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use lumen_ui_audio::{Fader, Knob, LevelBar, VuMeter};
use lumen_ui_core::{install, DarkTheme, LightTheme, Theme, UiContext};

fn theme_ctx(ctx: &egui::Context, theme: &Arc<dyn Theme>) {
    install(ctx, theme.clone(), UiContext::default());
}

#[test]
fn knob_and_fader_render_under_dark_and_light() {
    let themes: [Arc<dyn Theme>; 2] = [Arc::new(DarkTheme::new()), Arc::new(LightTheme::new())];
    for theme in themes {
        let mut knob = 0.5_f32;
        let mut gain = -6.0_f32;
        let mut harness = Harness::new_ui(move |ui| {
            theme_ctx(ui.ctx(), &theme);
            ui.add(Knob::new(&mut knob, 0.0..=1.0));
            ui.add(Fader::new(&mut gain, -60.0..=6.0));
            ui.add(VuMeter::new(0.7).peak(0.9));
            ui.add(LevelBar::new(0.4));
        });
        harness.run(); // a panic here fails the test
    }
}

#[test]
fn knob_and_fader_expose_slider_role() {
    let mut harness = Harness::new_ui(|ui| {
        theme_ctx(ui.ctx(), &(Arc::new(DarkTheme::new()) as Arc<dyn Theme>));
        let mut knob = 0.5_f32;
        let mut gain = -6.0_f32;
        ui.add(Knob::new(&mut knob, 0.0..=1.0));
        ui.add(Fader::new(&mut gain, -60.0..=6.0));
    });

    harness.run();
    // Both controls expose their value to a11y as a slider.
    assert_eq!(
        harness.query_all_by_role(Role::Slider).count(),
        2,
        "Knob and Fader each expose a Slider role"
    );
}
