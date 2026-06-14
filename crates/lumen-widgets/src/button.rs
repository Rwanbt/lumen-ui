//! The lumen [`Button`] — validation widget for the v0.1 pipeline.
//!
//! egui 0.34 reality (cf. ROADMAP.md §Corrections d'API):
//! * `egui::Button` has neither `.padding()` nor `.shadow()` — padding + shadow
//!   are applied by wrapping it in an `egui::Frame`.
//! * hover/active are only known *after* allocation, so the interaction state
//!   that drives the recipe is read from the **previous** frame via
//!   `ctx.read_response(id)`. `install()` sets `max_passes = 2` so this is stable.

use egui::{Frame, Margin, Response, RichText, Ui, Widget};
use lumen_core::{anim, ButtonVariant, UiThemeExt, WidgetState};

/// A themed button. Build it with [`Button::primary`] / [`Button::secondary`] /
/// [`Button::ghost`] / [`Button::danger`], then `ui.add(button)`.
#[derive(Clone, Debug)]
pub struct Button {
    label: String,
    variant: ButtonVariant,
    enabled: bool,
}

impl Button {
    fn new(label: impl Into<String>, variant: ButtonVariant) -> Self {
        Self {
            label: label.into(),
            variant,
            enabled: true,
        }
    }

    #[must_use]
    pub fn primary(label: impl Into<String>) -> Self {
        Self::new(label, ButtonVariant::Primary)
    }

    #[must_use]
    pub fn secondary(label: impl Into<String>) -> Self {
        Self::new(label, ButtonVariant::Secondary)
    }

    #[must_use]
    pub fn ghost(label: impl Into<String>) -> Self {
        Self::new(label, ButtonVariant::Ghost)
    }

    #[must_use]
    pub fn danger(label: impl Into<String>) -> Self {
        Self::new(label, ButtonVariant::Danger)
    }

    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = ui.theme();
        let ui_ctx = ui.ui_ctx();
        let id = ui.next_auto_id();

        // hover is only known after allocation → read the previous frame's response.
        let prev = ui.ctx().read_response(id);
        let state = if !self.enabled {
            WidgetState::Disabled
        } else if prev
            .as_ref()
            .is_some_and(egui::Response::is_pointer_button_down_on)
        {
            WidgetState::Active
        } else if prev.as_ref().is_some_and(egui::Response::hovered) {
            WidgetState::Hovered
        } else {
            WidgetState::Normal
        };

        let recipe = theme.button_recipe(self.variant, state, &ui_ctx);

        // Minimal motion (v0.2): interpolate the fill toward its target state color.
        // Swaps to the lumen-motion spring solver in v0.5 with no API change (ADR-0003).
        let fill = anim::lerp_color(
            ui.ctx(),
            id.with("fill"),
            recipe.fill,
            theme.tokens().motion.base,
        );

        // padding + shadow via Frame; fill/stroke/corner_radius on the Button.
        Frame::NONE
            .inner_margin(Margin::symmetric(
                recipe.inner_margin.x as i8,
                recipe.inner_margin.y as i8,
            ))
            .shadow(recipe.shadow)
            .corner_radius(recipe.corner_radius)
            .fill(fill)
            .show(ui, |ui| {
                ui.add_enabled(
                    self.enabled,
                    egui::Button::new(RichText::new(&self.label).color(recipe.text_color))
                        .fill(fill)
                        .stroke(recipe.stroke)
                        .corner_radius(recipe.corner_radius),
                )
            })
            .inner
    }
}
