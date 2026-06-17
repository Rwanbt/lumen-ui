//! [`FileUpload`] — a drop zone reporting files dropped onto it via egui's native drag-and-drop.
//!
//! Zero extra dependency (no `rfd`): it reads `ctx.input().raw.dropped_files`. A native "browse"
//! dialog would need an optional `rfd` feature and is intentionally left out of this version.

use egui::{DroppedFile, Frame, Label, Response, RichText, Ui};
use lumen_ui_core::{FileUploadRecipe, UiThemeExt};

/// The outcome of showing a [`FileUpload`]: the zone's [`Response`] plus the files dropped on the
/// window this frame (empty on most frames). On desktop each file has a `path`; on web, `bytes`.
#[derive(Debug)]
pub struct FileUploadResponse {
    pub response: Response,
    pub dropped: Vec<DroppedFile>,
}

/// A drag-and-drop file zone. Build with [`FileUpload::new`] (+ optional [`FileUpload::prompt`]),
/// then [`FileUpload::show`]; handle [`FileUploadResponse::dropped`] when non-empty.
///
/// Stateless: it reads egui's global dropped/hovered files, so it needs no id.
#[derive(Debug, Clone)]
pub struct FileUpload {
    prompt: String,
}

impl Default for FileUpload {
    fn default() -> Self {
        Self {
            prompt: String::from("Drop files here"),
        }
    }
}

impl FileUpload {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Override the prompt text (default `"Drop files here"`).
    #[must_use]
    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = prompt.into();
        self
    }

    /// Draw the drop zone and return the files dropped this frame (if any).
    pub fn show(self, ui: &mut Ui) -> FileUploadResponse {
        let recipe = FileUploadRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        // egui exposes hovered/dropped files globally; a single zone reads them directly.
        let files_hovering = ui.input(|i| !i.raw.hovered_files.is_empty());
        let fill = if files_hovering {
            recipe.hover_fill
        } else {
            recipe.fill
        };

        let response = Frame::NONE
            .fill(fill)
            .stroke(recipe.border)
            .corner_radius(recipe.corner_radius)
            .inner_margin(crate::util::margin(recipe.inner_margin))
            .show(ui, |ui| {
                ui.set_min_height(recipe.min_height);
                ui.vertical_centered(|ui| {
                    ui.add(Label::new(
                        RichText::new(&self.prompt).color(recipe.text_color),
                    ));
                });
            })
            .response;

        let dropped = ui.input(|i| i.raw.dropped_files.clone());
        FileUploadResponse { response, dropped }
    }
}
