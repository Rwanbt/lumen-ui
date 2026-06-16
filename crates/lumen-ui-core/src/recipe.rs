//! Recipes — the resolved, ready-to-draw style for one widget in one state.
//!
//! Recipes are parameterized by `(variant, state, ctx)` from v0.1 so adding
//! interaction states later is *not* a breaking change to the [`crate::Theme`]
//! trait (cf. ROADMAP.md §Changelog — the most fundamental layer must be stable).

use egui::{Color32, CornerRadius, Shadow, Stroke, Vec2};

use crate::context::UiContext;
use crate::tokens::Tokens;

/// Base diameter of a spinner before density scaling, in points.
const SPINNER_BASE_SIZE: f32 = 24.0;
/// Base height of a linear progress bar before density scaling, in points.
const PROGRESS_BASE_HEIGHT: f32 = 8.0;
/// Thickness of a divider rule, in points.
const DIVIDER_THICKNESS: f32 = 1.0;

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
    /// Keyboard focus — primarily for text input.
    Focused,
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

/// Fully resolved style for a text input field.
#[derive(Clone, Copy, Debug)]
pub struct TextFieldRecipe {
    pub fill: Color32,
    pub text_color: Color32,
    pub border: Stroke,
    pub corner_radius: CornerRadius,
    pub inner_margin: Vec2,
}

/// Fully resolved style for a slider.
#[derive(Clone, Copy, Debug)]
pub struct SliderRecipe {
    /// The inactive part of the track (right of the knob).
    pub track: Color32,
    /// The filled part of the track (left of the knob).
    pub fill: Color32,
    /// The draggable knob.
    pub knob: Color32,
}

/// Fully resolved style for a boolean toggle (switch, checkbox).
///
/// Geometry (pill vs rounded box) is chosen by the widget; the recipe carries
/// the colors and border so on/off and hover states stay theme-driven.
#[derive(Clone, Copy, Debug)]
pub struct ToggleRecipe {
    /// Track/box fill for the resolved on/off state.
    pub track: Color32,
    /// Knob (switch) or check mark (checkbox) color.
    pub knob: Color32,
    /// Border around the track/box.
    pub border: Stroke,
}

/// Fully resolved style for a card container.
#[derive(Clone, Copy, Debug)]
pub struct CardRecipe {
    pub fill: Color32,
    pub stroke: Stroke,
    pub corner_radius: CornerRadius,
    pub shadow: Shadow,
    pub inner_margin: Vec2,
}

/// Semantic flavor of a badge.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BadgeVariant {
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
}

/// Fully resolved style for a badge (small pill-shaped status label).
#[derive(Clone, Copy, Debug)]
pub struct BadgeRecipe {
    pub fill: Color32,
    pub text_color: Color32,
    pub corner_radius: CornerRadius,
    pub inner_margin: Vec2,
    pub text_size: f32,
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

/// Resolved style for an indeterminate spinner.
#[derive(Clone, Copy, Debug)]
pub struct SpinnerRecipe {
    pub color: Color32,
    pub size: f32,
}

impl SpinnerRecipe {
    /// Pure resolution from tokens (cf. ADR-0009): a spinner has no states.
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        Self {
            color: tokens.colors.primary,
            size: SPINNER_BASE_SIZE * ctx.density_scale(),
        }
    }
}

/// Resolved style for a linear (determinate) progress bar.
#[derive(Clone, Copy, Debug)]
pub struct ProgressRecipe {
    pub fill: Color32,
    pub track: Color32,
    pub height: f32,
    pub corner_radius: CornerRadius,
}

impl ProgressRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        Self {
            fill: tokens.colors.primary,
            track: tokens.colors.surface_variant,
            height: PROGRESS_BASE_HEIGHT * ctx.density_scale(),
            corner_radius: tokens.radius.full,
        }
    }
}

/// Resolved style for a divider / separator rule.
#[derive(Clone, Copy, Debug)]
pub struct DividerRecipe {
    pub color: Color32,
    pub thickness: f32,
}

impl DividerRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens) -> Self {
        Self {
            color: tokens.colors.border,
            thickness: DIVIDER_THICKNESS,
        }
    }
}
