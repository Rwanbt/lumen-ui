//! Enter/exit transitions built on the easing/spring primitives.
//!
//! v0.5 ships [`fade`]. Slide/scale need sub-tree transforms that stable egui
//! doesn't expose ergonomically; build them at the app level from [`crate::Spring`]
//! / [`crate::ease`] for now (tracked for a later version).

use std::hash::Hash;

use egui::{Id, Ui};

use crate::{ease, Easing};

const FADE_DURATION: f32 = 0.18;
const HIDDEN_EPS: f32 = 0.001;

/// Fade `content` in/out by `visible`, animating opacity. Returns the closure's
/// value while at all visible, and `None` once fully faded out (so the content is
/// not laid out when hidden). `id_source` must be stable across frames.
pub fn fade<R>(
    ui: &mut Ui,
    id_source: impl Hash,
    visible: bool,
    content: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let id = Id::new(id_source).with("lumen_fade");
    let opacity = ease(
        ui.ctx(),
        id,
        if visible { 1.0 } else { 0.0 },
        FADE_DURATION,
        Easing::EaseOut,
    );
    if opacity <= HIDDEN_EPS {
        return None;
    }
    let inner = ui
        .scope(|ui| {
            ui.set_opacity(opacity);
            content(ui)
        })
        .inner;
    Some(inner)
}
