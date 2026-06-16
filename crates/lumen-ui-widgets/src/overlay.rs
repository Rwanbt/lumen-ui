//! Anchored overlays: [`tooltip`], [`hover_card`], [`popover`], and [`context_menu`].
//!
//! Thin, themed helpers over egui's `Response` overlays and the `egui::Popup`
//! API. They follow the installed theme through the global visuals.

use egui::{InnerResponse, Popup, Response, Ui};

use crate::card::Card;
use crate::text::Label;

/// Attach a hover tooltip to a response. Returns the (same) response for optional chaining.
pub fn tooltip(response: Response, text: impl Into<String>) -> Response {
    let text = text.into();
    response.on_hover_ui(|ui| {
        ui.add(Label::new(text));
    })
}

/// Attach a rich hover card (themed [`Card`] content) to a response, for more than
/// the plain text a [`tooltip`] shows. Returns the (same) response for chaining.
pub fn hover_card(response: Response, add_contents: impl FnOnce(&mut Ui)) -> Response {
    response.on_hover_ui(|ui| {
        Card::new().show(ui, add_contents);
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
