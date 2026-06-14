//! [`CommandPalette`] — a searchable command list in a modal overlay, with its
//! open state + query kept in `egui` memory (no external state).

use std::hash::Hash;

use egui::{Align, Context, Id, Layout, ScrollArea};
use lumen_widgets::{Button, TextField};

fn open_key(id: Id) -> Id {
    id.with("lumen_palette_open")
}
fn query_key(id: Id) -> Id {
    id.with("lumen_palette_query")
}

/// Open the command palette identified by `id_source` (same value as [`CommandPalette::new`]).
pub fn open_command_palette(ctx: &Context, id_source: impl Hash) {
    ctx.data_mut(|d| d.insert_temp(open_key(Id::new(id_source)), true));
}

/// A searchable command list. Build with [`CommandPalette::new`] + [`CommandPalette::command`],
/// then call [`CommandPalette::show`] every frame.
#[derive(Clone, Debug)]
pub struct CommandPalette {
    id: Id,
    commands: Vec<String>,
}

impl CommandPalette {
    #[must_use]
    pub fn new(id_source: impl Hash) -> Self {
        Self {
            id: Id::new(id_source),
            commands: Vec::new(),
        }
    }

    #[must_use]
    pub fn command(mut self, label: impl Into<String>) -> Self {
        self.commands.push(label.into());
        self
    }

    /// Draw the palette if open. Returns the chosen command index once selected
    /// (and closes), else `None`. Closes on backdrop click / Esc.
    pub fn show(self, ctx: &Context) -> Option<usize> {
        let open = ctx.data_mut(|d| d.get_temp::<bool>(open_key(self.id)).unwrap_or(false));
        if !open {
            return None;
        }

        let mut query: String =
            ctx.data_mut(|d| d.get_temp(query_key(self.id)).unwrap_or_default());
        let mut chosen: Option<usize> = None;

        let response = egui::Modal::new(self.id.with("modal")).show(ctx, |ui| {
            ui.set_min_width(360.0);
            ui.add(TextField::new(&mut query).hint("Type a command…"));
            ui.add_space(8.0);
            let needle = query.to_lowercase();
            ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
                    for (index, command) in self.commands.iter().enumerate() {
                        let matches = needle.is_empty() || command.to_lowercase().contains(&needle);
                        if matches && ui.add(Button::ghost(command.clone())).clicked() {
                            chosen = Some(index);
                        }
                    }
                });
            });
        });

        let closing = chosen.is_some() || response.should_close();
        ctx.data_mut(|d| {
            if closing {
                d.insert_temp(open_key(self.id), false);
                d.insert_temp(query_key(self.id), String::new());
            } else {
                d.insert_temp(query_key(self.id), query);
            }
        });
        chosen
    }
}
