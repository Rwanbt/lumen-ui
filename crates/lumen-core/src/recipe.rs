//! Recipes — the resolved, ready-to-draw style for one widget in one state.
//!
//! Recipes are parameterized by `(variant, state, ctx)` from v0.1 so adding
//! interaction states later is *not* a breaking change to the [`crate::Theme`]
//! trait (cf. ROADMAP.md §Changelog — the most fundamental layer must be stable).

use egui::{Color32, CornerRadius, Shadow, Stroke, Vec2};

/// Visual variants of a button.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Danger,
}

/// Interaction state of a widget. Read from the *previous* frame, because
/// egui only knows hover/active after allocation (cf. ROADMAP.md §Corrections d'API).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WidgetState {
    Normal,
    Hovered,
    Active,
    Disabled,
}

/// Semantic role of a piece of text — drives color and size, not layout.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextRole {
    Display,
    Heading,
    Body,
    Label,
    Muted,
}

/// Fully resolved style for a run of text.
#[derive(Clone, Copy, Debug)]
pub struct TextRecipe {
    pub color: Color32,
    pub size: f32,
}

/// Fully resolved style for a button.
///
/// Note the deliberate split, dictated by egui 0.34's API: `egui::Button`
/// exposes neither `.padding()` nor `.shadow()`, so `shadow` + `inner_margin`
/// are applied by wrapping the button in an `egui::Frame`.
#[derive(Clone, Copy, Debug)]
pub struct ButtonRecipe {
    pub fill: Color32,
    pub text_color: Color32,
    pub stroke: Stroke,
    pub corner_radius: CornerRadius,
    /// Applied via `Frame`, **not** `Button` (egui 0.34 has no `Button::shadow`).
    pub shadow: Shadow,
    /// Padding — maps to `Frame::inner_margin`, **not** `Button::padding`.
    pub inner_margin: Vec2,
}
