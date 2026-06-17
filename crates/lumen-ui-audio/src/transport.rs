//! [`Transport`] — a play/pause/stop/record control bar for audio/DAW UIs.

use egui::{pos2, vec2, Rect, Response, Sense, Shape, Ui, WidgetInfo, WidgetType};
use lumen_ui_core::{TransportRecipe, UiThemeExt};

/// The button a user pressed on a [`Transport`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TransportAction {
    /// The play/pause toggle (its glyph reflects the current `playing` state).
    PlayPause,
    Stop,
    Record,
}

/// A transport bar with play/pause, stop and record buttons. The play/pause glyph follows the
/// `playing` flag and the record button lights up while `recording`. [`Transport::show`] returns
/// the [`TransportAction`] pressed this frame, if any. Icons are painter-drawn (no font glyphs).
#[derive(Clone, Copy, Debug, Default)]
pub struct Transport {
    playing: bool,
    recording: bool,
}

impl Transport {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Show the play/pause button as "pause" when `playing` is true.
    #[must_use]
    pub fn playing(mut self, playing: bool) -> Self {
        self.playing = playing;
        self
    }

    /// Light the record button while `recording` is true.
    #[must_use]
    pub fn recording(mut self, recording: bool) -> Self {
        self.recording = recording;
        self
    }

    /// Draw the bar; returns the action pressed this frame, if any.
    pub fn show(self, ui: &mut Ui) -> Option<TransportAction> {
        let recipe = TransportRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let mut action = None;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = recipe.gap;
            let play_label = if self.playing { "pause" } else { "play" };
            if button(
                ui,
                &recipe,
                Icon::PlayPause {
                    playing: self.playing,
                },
                play_label,
            )
            .clicked()
            {
                action = Some(TransportAction::PlayPause);
            }
            if button(ui, &recipe, Icon::Stop, "stop").clicked() {
                action = Some(TransportAction::Stop);
            }
            if button(
                ui,
                &recipe,
                Icon::Record {
                    active: self.recording,
                },
                "record",
            )
            .clicked()
            {
                action = Some(TransportAction::Record);
            }
        });
        action
    }
}

/// Which icon a transport button draws.
#[derive(Clone, Copy)]
enum Icon {
    PlayPause { playing: bool },
    Stop,
    Record { active: bool },
}

/// Draw one themed transport button (background + painter icon) and return its click [`Response`].
fn button(ui: &mut Ui, recipe: &TransportRecipe, icon: Icon, label: &str) -> Response {
    let (rect, response) = ui.allocate_exact_size(vec2(recipe.size, recipe.size), Sense::click());
    let enabled = ui.is_enabled();
    let painter = ui.painter();
    painter.rect_filled(rect, recipe.corner_radius, recipe.button);

    // Icons fit inside a centered square of half the button size.
    let g = rect.size().min_elem() * 0.25;
    let c = rect.center();
    match icon {
        Icon::PlayPause { playing: false } => {
            // Right-pointing triangle.
            painter.add(Shape::convex_polygon(
                vec![
                    pos2(c.x - g, c.y - g),
                    pos2(c.x - g, c.y + g),
                    pos2(c.x + g, c.y),
                ],
                recipe.glyph,
                egui::Stroke::NONE,
            ));
        }
        Icon::PlayPause { playing: true } => {
            // Two vertical bars.
            let bar = g * 0.5;
            painter.rect_filled(
                Rect::from_min_max(pos2(c.x - g, c.y - g), pos2(c.x - g + bar, c.y + g)),
                0.0,
                recipe.glyph,
            );
            painter.rect_filled(
                Rect::from_min_max(pos2(c.x + g - bar, c.y - g), pos2(c.x + g, c.y + g)),
                0.0,
                recipe.glyph,
            );
        }
        Icon::Stop => {
            painter.rect_filled(
                Rect::from_center_size(c, vec2(g * 2.0, g * 2.0)),
                0.0,
                recipe.glyph,
            );
        }
        Icon::Record { active } => {
            let color = if active {
                recipe.record_active
            } else {
                recipe.glyph
            };
            painter.circle_filled(c, g, color);
        }
    }

    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, label));
    response
}
