//! GPU pixel snapshot of the audio controls, rendered through `egui_kittest`'s wgpu renderer.
//!
//! Marked `#[ignore]` so the normal (GPU-less) test matrix skips it: it is run only by the
//! dedicated `snapshots` workflow on a Linux software adapter (lavapipe/llvmpipe), where the
//! reference PNGs under `tests/snapshots/` are generated and verified in the same environment.
//! Update locally/in CI with `UPDATE_SNAPSHOTS=1 cargo test -p lumen-ui-audio -- --ignored`.

use std::sync::Arc;

use egui_kittest::Harness;
use lumen_ui_audio::{Fader, Knob, LevelBar, Transport, VuMeter, Waveform, XyPad};
use lumen_ui_core::{install, DarkTheme, Theme, UiContext};

#[test]
#[ignore = "needs a wgpu adapter; run via the snapshots workflow"]
fn audio_controls_snapshot_dark() {
    let mut knob = 0.6_f32;
    let mut gain = -6.0_f32;
    let mut x = 0.3_f32;
    let mut y = 0.7_f32;
    let samples: Vec<f32> = (0..256).map(|i| (i as f32 * 0.12).sin()).collect();

    let mut harness = Harness::builder().wgpu().build_ui(move |ui| {
        install(
            ui.ctx(),
            Arc::new(DarkTheme::new()) as Arc<dyn Theme>,
            UiContext::default(),
        );
        ui.horizontal(|ui| {
            ui.add(Knob::new(&mut knob, 0.0..=1.0));
            ui.add(Fader::new(&mut gain, -60.0..=6.0));
            ui.add(VuMeter::new(0.7).peak(0.9));
            ui.add(XyPad::new(&mut x, &mut y, 0.0..=1.0, 0.0..=1.0));
        });
        ui.add(LevelBar::new(0.4));
        Transport::new().playing(true).recording(false).show(ui);
        ui.add(Waveform::new(&samples));
    });

    harness.run();
    harness.snapshot("audio_controls_dark");
}
