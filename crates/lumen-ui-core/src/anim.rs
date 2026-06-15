//! Minimal motion (v0.2) — no heavy dependency.
//!
//! Widgets interpolate color/opacity through egui's built-in
//! `animate_value_with_time`. In v0.5 the same call sites switch to the
//! `lumen-ui-motion` spring solver **without changing any widget's public API**
//! (cf. ROADMAP.md §C.6).

use egui::{Color32, Context, Id};

/// Smoothly interpolate towards `target` over `dur` seconds, per channel.
///
/// The `id` must be stable across frames for a given widget instance.
#[must_use]
pub fn lerp_color(ctx: &Context, id: Id, target: Color32, dur: f32) -> Color32 {
    let chan = |value: u8, channel: u8| -> u8 {
        ctx.animate_value_with_time(id.with(channel), f32::from(value), dur)
            .round()
            .clamp(0.0, 255.0) as u8
    };
    Color32::from_rgba_unmultiplied(
        chan(target.r(), 0),
        chan(target.g(), 1),
        chan(target.b(), 2),
        chan(target.a(), 3),
    )
}
