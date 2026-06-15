//! `lumen-ui-motion` — animation engine for lumen-ui: spring physics, easings, and
//! (from v0.5 slice 2) transitions.
//!
//! Enable via the `motion` feature of the `lumen-ui` façade. This is the richer
//! counterpart to `lumen-ui-core::anim` (the dependency-free minimal motion used by
//! widgets): same call-site shape, spring/easing-backed implementation (ADR-0003).
//!
//! ```ignore
//! use eframe::egui;
//! use lumen_ui_motion::{Spring, ease, Easing};
//!
//! fn ui(ctx: &egui::Context, open: bool) {
//!     let id = egui::Id::new("panel_w");
//!     let width = Spring::SMOOTH.animate(ctx, id, if open { 240.0 } else { 0.0 });
//!     let fade = ease(ctx, egui::Id::new("fade"), if open { 1.0 } else { 0.0 }, 0.2, Easing::EaseOut);
//!     let _ = (width, fade);
//! }
//! ```

#![forbid(unsafe_code)]

mod easing;
mod spring;
mod transitions;

pub use easing::Easing;
pub use spring::Spring;
pub use transitions::fade;

use egui::{Context, Id};

#[derive(Clone, Copy, Default)]
struct TweenState {
    from: f32,
    to: f32,
    start_time: f64,
    initialized: bool,
}

/// Tween a scalar towards `target` over `duration` seconds along an [`Easing`] curve.
///
/// Returns the current eased value. When `target` changes, the tween restarts from
/// the value reached so far. `id` must be stable across frames.
pub fn ease(ctx: &Context, id: Id, target: f32, duration: f32, easing: Easing) -> f32 {
    let now = ctx.input(|i| i.time);
    let mut state: TweenState = ctx.data_mut(|d| d.get_temp(id).unwrap_or_default());

    // (Re)start when uninitialized or the target moved.
    if !state.initialized || (state.to - target).abs() > f32::EPSILON {
        let current = current_value(state, now, duration, easing);
        state = TweenState {
            from: current,
            to: target,
            start_time: now,
            initialized: true,
        };
    }

    let value = current_value(state, now, duration, easing);
    ctx.data_mut(|d| d.insert_temp(id, state));

    if duration > 0.0 && (now - state.start_time) < f64::from(duration) {
        ctx.request_repaint();
    }
    value
}

fn current_value(state: TweenState, now: f64, duration: f32, easing: Easing) -> f32 {
    if !state.initialized {
        return state.to;
    }
    if duration <= 0.0 {
        return state.to;
    }
    let elapsed = (now - state.start_time) as f32;
    let t = (elapsed / duration).clamp(0.0, 1.0);
    state.from + (state.to - state.from) * easing.eval(t)
}
