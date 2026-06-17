//! [`DatePicker`] — a button showing the current date that opens a [`Calendar`] popup.

use std::hash::Hash;

use egui::{Id, Popup, PopupCloseBehavior, Response, Ui};
use lumen_ui_core::{month_name, Date};

use crate::{Button, Calendar};

/// A date input bound to a `&mut Date`. Renders a button labelled with the date; clicking it opens
/// a [`Calendar`] in a popup. Picking a day updates the date and closes the popup; the calendar's
/// month arrows keep it open. Build with [`DatePicker::new`], then [`DatePicker::show`].
#[derive(Debug)]
pub struct DatePicker<'a> {
    id: Id,
    date: &'a mut Date,
}

impl<'a> DatePicker<'a> {
    /// `id_source` must be stable and unique within the parent `Ui`.
    #[must_use]
    pub fn new(id_source: impl Hash, date: &'a mut Date) -> Self {
        Self {
            id: Id::new(id_source),
            date,
        }
    }

    /// Draw the date picker. Returns the trigger button's response.
    pub fn show(self, ui: &mut Ui) -> Response {
        let label = format!(
            "{} {} {}",
            self.date.day,
            month_name(self.date.month),
            self.date.year
        );
        let before = *self.date;
        let response = ui.add(Button::secondary(label));

        // CloseOnClickOutside (not the default CloseOnClick) so paging months with the calendar's
        // ‹/› arrows doesn't dismiss the popup; we close it ourselves once the day changes.
        let popup_id = self.id.with("popup");
        Popup::menu(&response)
            .id(popup_id)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                Calendar::new(self.id.with("cal"), self.date).show(ui);
            });

        if *self.date != before {
            Popup::close_id(ui.ctx(), popup_id);
        }
        response
    }
}
