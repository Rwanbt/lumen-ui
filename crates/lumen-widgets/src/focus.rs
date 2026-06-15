//! Shared focus-visible helper (a11y, v0.8).
//!
//! egui makes any `Sense::click` widget keyboard-focusable and triggers
//! `Response::clicked()` on Space/Enter, so navigation works for free. What's
//! missing is a *visible* focus indicator for our painter-drawn widgets — egui's
//! global `widgets.active` ring only reaches widgets that defer to the style.
//! This draws a consistent ring on top of any widget that holds focus.

use egui::{Color32, CornerRadius, Response, Stroke, StrokeKind, Ui};

/// Paint a 2 px ring just outside `response` when it holds keyboard focus.
///
/// Drawn on top so it shows on any fill/variant; `color` is normally the theme's
/// primary. No-op when the widget is not focused.
pub(crate) fn focus_ring(
    ui: &Ui,
    response: &Response,
    corner_radius: impl Into<CornerRadius>,
    color: Color32,
) {
    if response.has_focus() {
        ui.painter().rect_stroke(
            response.rect.expand(2.0),
            corner_radius,
            Stroke::new(2.0, color),
            StrokeKind::Outside,
        );
    }
}
