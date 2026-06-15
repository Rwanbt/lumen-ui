//! Anchored overlays: [`tooltip`], [`popover`], and [`context_menu`].
//!
//! Thin, themed helpers over egui's `Response` overlays and the `egui::Popup`
//! API. They follow the installed theme through the global visuals.

use egui::{InnerResponse, Popup, Response, Ui};

use crate::text::Label;

/// Attach a hover tooltip to a response. Returns the (same) response for optional chaining.
pub fn tooltip(response: Response, text: impl Into<String>) -> Response {
    let text = text.into();
    response.on_hover_ui(|ui| {
        ui.add(Label::new(text));
    })
}

/// Show a click-toggled popover anchored to `trigger` (e.g. a button response).
/// Returns the inner response while open. Closes on outside click.
pub fn popover<R>(
    trigger: &Response,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<InnerResponse<R>> {
    Popup::from_toggle_button_response(trigger).show(add_contents)
}

/// Attach a right-click context menu to `trigger`. Returns the inner response
/// while the menu is open.
pub fn context_menu(
    trigger: &Response,
    add_contents: impl FnOnce(&mut Ui),
) -> Option<InnerResponse<()>> {
    trigger.context_menu(add_contents)
}
