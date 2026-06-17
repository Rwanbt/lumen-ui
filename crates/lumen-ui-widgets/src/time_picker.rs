//! [`TimePicker`] — pick an `hour:minute`, bound to a `&mut Time`.

use egui::{DragValue, Response, Ui};
use lumen_ui_core::Time;

/// A compact time editor bound to a `&mut Time`: two `DragValue`s (hour `0..=23`, minute `0..=59`)
/// separated by a colon, each zero-padded. Build with [`TimePicker::new`], then [`TimePicker::show`].
#[derive(Debug)]
pub struct TimePicker<'a> {
    time: &'a mut Time,
}

impl<'a> TimePicker<'a> {
    #[must_use]
    pub fn new(time: &'a mut Time) -> Self {
        Self { time }
    }

    /// Draw the time picker. Returns the union of the two field responses (changed if either did).
    pub fn show(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            // Zero-pad both fields so the value reads like a clock (09:05, not 9:5).
            let hour = ui.add(
                DragValue::new(&mut self.time.hour)
                    .range(0..=23)
                    .custom_formatter(|n, _| format!("{:02}", n as u32)),
            );
            ui.label(":");
            let minute = ui.add(
                DragValue::new(&mut self.time.minute)
                    .range(0..=59)
                    .custom_formatter(|n, _| format!("{:02}", n as u32)),
            );
            hour | minute
        })
        .inner
    }
}
