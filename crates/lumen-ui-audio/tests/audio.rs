//! Functional tests for the audio controls via `egui_kittest` (headless).

use std::sync::Arc;

use egui::accesskit::Role;
use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use lumen_ui_audio::{Fader, Knob, LevelBar, Transport, TransportAction, VuMeter, Waveform, XyPad};
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
        let mut x = 0.3_f32;
        let mut y = 0.7_f32;
        let samples: Vec<f32> = (0..256).map(|i| (i as f32 * 0.1).sin()).collect();
        let mut harness = Harness::new_ui(move |ui| {
            theme_ctx(ui.ctx(), &theme);
            ui.add(Knob::new(&mut knob, 0.0..=1.0));
            ui.add(Fader::new(&mut gain, -60.0..=6.0));
            ui.add(VuMeter::new(0.7).peak(0.9));
            ui.add(LevelBar::new(0.4));
            ui.add(Waveform::new(&samples));
            ui.add(XyPad::new(&mut x, &mut y, 0.0..=1.0, 0.0..=1.0));
            Transport::new().playing(true).recording(false).show(ui);
        });
        harness.run(); // a panic here fails the test
    }
}

#[test]
fn transport_emits_action_on_click() {
    #[derive(Default)]
    struct State {
        last: Option<TransportAction>,
    }

    let mut harness = Harness::new_ui_state(
        |ui, state: &mut State| {
            theme_ctx(ui.ctx(), &(Arc::new(DarkTheme::new()) as Arc<dyn Theme>));
            if let Some(action) = Transport::new().show(ui) {
                state.last = Some(action);
            }
        },
        State::default(),
    );

    harness.run();
    harness.get_by_label("play").click();
    harness.run();
    assert_eq!(harness.state().last, Some(TransportAction::PlayPause));
    harness.get_by_label("record").click();
    harness.run();
    assert_eq!(harness.state().last, Some(TransportAction::Record));
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
