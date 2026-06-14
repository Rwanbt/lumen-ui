//! [`Modal`] — a centered dialog with a backdrop, whose open state lives in
//! `egui` memory (no external boolean).
//!
//! Open it from anywhere with [`open_modal`]; render it with [`Modal::show`],
//! which returns `None` while closed. It auto-closes on backdrop click or Esc
//! (via `egui::Modal`), and you can close it explicitly with [`close_modal`].

use std::hash::Hash;

use egui::{Context, Id, Ui};

use crate::text::Heading;

fn open_key(id: Id) -> Id {
    id.with("lumen_modal_open")
}

/// Open the modal identified by `id_source` (same value passed to [`Modal::new`]).
pub fn open_modal(ctx: &Context, id_source: impl Hash) {
    ctx.data_mut(|d| d.insert_temp(open_key(Id::new(id_source)), true));
}

/// Close the modal identified by `id_source`.
pub fn close_modal(ctx: &Context, id_source: impl Hash) {
    ctx.data_mut(|d| d.insert_temp(open_key(Id::new(id_source)), false));
}

/// A centered dialog. Build with [`Modal::new`] (+ optional [`Modal::title`]),
/// then call [`Modal::show`] every frame; it draws only while open.
#[derive(Clone, Debug)]
pub struct Modal {
    id: Id,
    title: String,
}

impl Modal {
    #[must_use]
    pub fn new(id_source: impl Hash) -> Self {
        Self {
            id: Id::new(id_source),
            title: String::new(),
        }
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Draw the modal if open. Returns the closure's value while shown, else `None`.
    pub fn show<R>(self, ctx: &Context, add_contents: impl FnOnce(&mut Ui) -> R) -> Option<R> {
        let open = ctx.data_mut(|d| d.get_temp::<bool>(open_key(self.id)).unwrap_or(false));
        if !open {
            return None;
        }

        let title = self.title;
        let response = egui::Modal::new(self.id).show(ctx, |ui| {
            if !title.is_empty() {
                ui.add(Heading::new(title.clone()));
                ui.add_space(8.0);
            }
            add_contents(ui)
        });

        if response.should_close() {
            ctx.data_mut(|d| d.insert_temp(open_key(self.id), false));
        }
        Some(response.inner)
    }
}
