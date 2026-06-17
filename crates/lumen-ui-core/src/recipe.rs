//! Recipes — the resolved, ready-to-draw style for one widget in one state.
//!
//! Recipes are parameterized by `(variant, state, ctx)` from v0.1 so adding
//! interaction states later is *not* a breaking change to the [`crate::Theme`]
//! trait (cf. ROADMAP.md §Changelog — the most fundamental layer must be stable).

use egui::{Color32, CornerRadius, Shadow, Stroke, Vec2};

use crate::context::UiContext;
use crate::palette::ThemeMode;
use crate::tokens::Tokens;

/// Background tint opacity for an alert over the surface, in `0..=255`.
const ALERT_TINT_ALPHA: u8 = 28;
/// Base square size of an icon button before density scaling, in points.
const ICON_BUTTON_BASE_SIZE: f32 = 32.0;
/// Emphasis amount applied to the icon-button fill while pressed.
const ICON_BUTTON_ACTIVE_EMPHASIS: f32 = 0.14;
/// Base diameter of a stepper step circle before density scaling, in points.
const STEPPER_BASE_CIRCLE: f32 = 24.0;
/// Base diameter of a circular progress ring before density scaling, in points.
const CIRCULAR_PROGRESS_BASE_SIZE: f32 = 36.0;
/// Ring stroke thickness as a fraction of the ring diameter.
const CIRCULAR_PROGRESS_THICKNESS_RATIO: f32 = 0.12;
/// Base diameter of an avatar before density scaling, in points.
const AVATAR_BASE_SIZE: f32 = 36.0;
/// Avatar initials font size as a fraction of the avatar diameter.
const AVATAR_FONT_RATIO: f32 = 0.4;
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

/// Resolved style for an `IconButton` (square, ghost-style icon trigger).
///
/// First recipe to consume [`ThemeMode`] emphasis through the ADR-0009 pure-resolve
/// path: the pressed fill is derived from `surface_variant` via the theme's emphasis
/// direction (lighten in dark themes, darken in light).
#[derive(Clone, Copy, Debug)]
pub struct IconButtonRecipe {
    pub size: f32,
    pub corner_radius: CornerRadius,
    /// Fill on hover (idle is transparent — ghost style).
    pub hover_fill: Color32,
    /// Fill while pressed.
    pub active_fill: Color32,
}

impl IconButtonRecipe {
    /// Pure resolution from tokens + mode (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, mode: ThemeMode, ctx: &UiContext) -> Self {
        let emphasize = mode.emphasis();
        Self {
            size: ICON_BUTTON_BASE_SIZE * ctx.density_scale(),
            corner_radius: tokens.radius.md,
            hover_fill: tokens.colors.surface_variant,
            active_fill: emphasize(tokens.colors.surface_variant, ICON_BUTTON_ACTIVE_EMPHASIS),
        }
    }
}

/// Semantic flavor of an inline alert / banner.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlertVariant {
    Info,
    Success,
    Warning,
    Danger,
}

/// Resolved style for an inline alert. `fill` is a low-opacity tint of `accent`
/// so it reads as a colored banner over the surface; `accent` colors the border
/// and the (optional) title.
#[derive(Clone, Copy, Debug)]
pub struct AlertRecipe {
    pub fill: Color32,
    pub accent: Color32,
    pub text_color: Color32,
    pub corner_radius: CornerRadius,
    pub inner_margin: Vec2,
}

impl AlertRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, variant: AlertVariant, ctx: &UiContext) -> Self {
        let c = &tokens.colors;
        let accent = match variant {
            AlertVariant::Info => c.primary,
            AlertVariant::Success => c.success,
            AlertVariant::Warning => c.warning,
            AlertVariant::Danger => c.danger,
        };
        let fill =
            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), ALERT_TINT_ALPHA);
        let scale = ctx.density_scale();
        Self {
            fill,
            accent,
            text_color: c.text,
            corner_radius: tokens.radius.md,
            inner_margin: Vec2::new(tokens.spacing.md * scale, tokens.spacing.sm * scale),
        }
    }
}

/// Resolved style for a skeleton loading placeholder.
#[derive(Clone, Copy, Debug)]
pub struct SkeletonRecipe {
    pub fill: Color32,
    pub corner_radius: CornerRadius,
}

impl SkeletonRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens) -> Self {
        Self {
            fill: tokens.colors.surface_variant,
            corner_radius: tokens.radius.sm,
        }
    }
}

/// Resolved style for an avatar (circular initials badge).
#[derive(Clone, Copy, Debug)]
pub struct AvatarRecipe {
    pub bg: Color32,
    pub text_color: Color32,
    pub size: f32,
    pub font_size: f32,
}

impl AvatarRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        let size = AVATAR_BASE_SIZE * ctx.density_scale();
        Self {
            bg: tokens.colors.primary,
            text_color: tokens.colors.on_primary,
            size,
            font_size: size * AVATAR_FONT_RATIO,
        }
    }
}

/// Resolved style for a chip / tag (pill, optionally removable).
#[derive(Clone, Copy, Debug)]
pub struct ChipRecipe {
    pub fill: Color32,
    pub text_color: Color32,
    pub corner_radius: CornerRadius,
    pub inner_margin: Vec2,
    pub text_size: f32,
}

impl ChipRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        let scale = ctx.density_scale();
        Self {
            fill: tokens.colors.surface_variant,
            text_color: tokens.colors.text,
            corner_radius: tokens.radius.full,
            inner_margin: Vec2::new(tokens.spacing.sm * scale, tokens.spacing.xs * scale),
            text_size: tokens.typography.label,
        }
    }
}

/// Resolved style for a `Kbd` keyboard-key indicator.
#[derive(Clone, Copy, Debug)]
pub struct KbdRecipe {
    pub fill: Color32,
    pub text_color: Color32,
    pub border: Stroke,
    pub corner_radius: CornerRadius,
    pub inner_margin: Vec2,
    pub text_size: f32,
}

impl KbdRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        let scale = ctx.density_scale();
        Self {
            fill: tokens.colors.surface_variant,
            text_color: tokens.colors.text_muted,
            border: Stroke::new(1.0, tokens.colors.border),
            corner_radius: tokens.radius.sm,
            inner_margin: Vec2::new(tokens.spacing.sm * scale, tokens.spacing.xs * scale),
            text_size: tokens.typography.label,
        }
    }
}

/// Resolved style for a `Stat` (metric) block.
#[derive(Clone, Copy, Debug)]
pub struct StatRecipe {
    pub label_color: Color32,
    pub value_color: Color32,
    pub positive_color: Color32,
    pub negative_color: Color32,
    pub label_size: f32,
    pub value_size: f32,
}

impl StatRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens) -> Self {
        Self {
            label_color: tokens.colors.text_muted,
            value_color: tokens.colors.text,
            positive_color: tokens.colors.success,
            negative_color: tokens.colors.danger,
            label_size: tokens.typography.label,
            value_size: tokens.typography.display,
        }
    }
}

/// Resolved style for a `Breadcrumb` trail.
#[derive(Clone, Copy, Debug)]
pub struct BreadcrumbRecipe {
    /// Color of clickable ancestor segments.
    pub link_color: Color32,
    /// Color of the final (current) segment.
    pub current_color: Color32,
    /// Color of the separator glyph between segments.
    pub separator_color: Color32,
    pub text_size: f32,
}

impl BreadcrumbRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens) -> Self {
        Self {
            link_color: tokens.colors.text_muted,
            current_color: tokens.colors.text,
            separator_color: tokens.colors.text_muted,
            text_size: tokens.typography.body,
        }
    }
}

/// Resolved style for a `SegmentedControl` (mutually-exclusive button group).
#[derive(Clone, Copy, Debug)]
pub struct SegmentedRecipe {
    /// Fill of the track behind all segments.
    pub container_fill: Color32,
    /// Fill of the selected segment.
    pub selected_fill: Color32,
    /// Text color of the selected segment.
    pub selected_text: Color32,
    /// Text color of unselected segments.
    pub text: Color32,
    pub corner_radius: CornerRadius,
    pub inner_margin: Vec2,
    pub text_size: f32,
}

impl SegmentedRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        let scale = ctx.density_scale();
        Self {
            container_fill: tokens.colors.surface_variant,
            selected_fill: tokens.colors.primary,
            selected_text: tokens.colors.on_primary,
            text: tokens.colors.text_muted,
            corner_radius: tokens.radius.md,
            inner_margin: Vec2::new(tokens.spacing.md * scale, tokens.spacing.xs * scale),
            text_size: tokens.typography.label,
        }
    }
}

/// Resolved style for a `Pagination` control.
#[derive(Clone, Copy, Debug)]
pub struct PaginationRecipe {
    /// Fill of the current page indicator.
    pub active_fill: Color32,
    /// Text color of the current page indicator.
    pub active_text: Color32,
    /// Text color of other (clickable) pages and the prev/next arrows.
    pub text: Color32,
    pub corner_radius: CornerRadius,
    pub inner_margin: Vec2,
    pub text_size: f32,
}

impl PaginationRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        let scale = ctx.density_scale();
        Self {
            active_fill: tokens.colors.primary,
            active_text: tokens.colors.on_primary,
            text: tokens.colors.text_muted,
            corner_radius: tokens.radius.sm,
            inner_margin: Vec2::new(tokens.spacing.sm * scale, tokens.spacing.xs * scale),
            text_size: tokens.typography.body,
        }
    }
}

/// Resolved style for an `EmptyState` placeholder block.
#[derive(Clone, Copy, Debug)]
pub struct EmptyStateRecipe {
    pub title_color: Color32,
    pub message_color: Color32,
    pub title_size: f32,
    pub message_size: f32,
    /// Vertical gap between the title and the message, in points.
    pub gap: f32,
}

impl EmptyStateRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        Self {
            title_color: tokens.colors.text,
            message_color: tokens.colors.text_muted,
            title_size: tokens.typography.heading,
            message_size: tokens.typography.body,
            gap: tokens.spacing.sm * ctx.density_scale(),
        }
    }
}

/// Resolved style for a textual `Link`.
#[derive(Clone, Copy, Debug)]
pub struct LinkRecipe {
    pub color: Color32,
    pub text_size: f32,
}

impl LinkRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens) -> Self {
        Self {
            color: tokens.colors.primary,
            text_size: tokens.typography.body,
        }
    }
}

/// Resolved style for a circular (determinate) progress ring.
#[derive(Clone, Copy, Debug)]
pub struct CircularProgressRecipe {
    pub track: Color32,
    pub fill: Color32,
    pub diameter: f32,
    pub thickness: f32,
}

impl CircularProgressRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        let diameter = CIRCULAR_PROGRESS_BASE_SIZE * ctx.density_scale();
        Self {
            track: tokens.colors.surface_variant,
            fill: tokens.colors.primary,
            diameter,
            thickness: diameter * CIRCULAR_PROGRESS_THICKNESS_RATIO,
        }
    }
}

/// Resolved style for a star `Rating` control.
#[derive(Clone, Copy, Debug)]
pub struct RatingRecipe {
    /// Color of a filled star.
    pub filled: Color32,
    /// Color of an empty star.
    pub empty: Color32,
    pub size: f32,
}

impl RatingRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens) -> Self {
        Self {
            filled: tokens.colors.warning,
            empty: tokens.colors.text_muted,
            size: tokens.typography.heading,
        }
    }
}

/// Resolved style for a `Stepper` (multi-step progress indicator).
#[derive(Clone, Copy, Debug)]
pub struct StepperRecipe {
    /// Circle fill for reached (done/active) steps.
    pub active_fill: Color32,
    /// Number color on reached steps.
    pub active_text: Color32,
    /// Circle fill for upcoming steps.
    pub inactive_fill: Color32,
    /// Number color on upcoming steps.
    pub inactive_text: Color32,
    /// Connector color before a reached step.
    pub connector_done: Color32,
    /// Connector color before an upcoming step.
    pub connector_todo: Color32,
    /// Label color for reached steps.
    pub label_active: Color32,
    /// Label color for upcoming steps.
    pub label_inactive: Color32,
    pub circle_size: f32,
    pub text_size: f32,
}

impl StepperRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        let c = &tokens.colors;
        Self {
            active_fill: c.primary,
            active_text: c.on_primary,
            inactive_fill: c.surface_variant,
            inactive_text: c.text_muted,
            connector_done: c.primary,
            connector_todo: c.border,
            label_active: c.text,
            label_inactive: c.text_muted,
            circle_size: STEPPER_BASE_CIRCLE * ctx.density_scale(),
            text_size: tokens.typography.label,
        }
    }
}

/// Resolved style for an inline `Code` span.
#[derive(Clone, Copy, Debug)]
pub struct CodeRecipe {
    pub fill: Color32,
    pub text_color: Color32,
    pub corner_radius: CornerRadius,
    pub inner_margin: Vec2,
    pub text_size: f32,
}

impl CodeRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        let scale = ctx.density_scale();
        Self {
            fill: tokens.colors.surface_variant,
            text_color: tokens.colors.text,
            corner_radius: tokens.radius.sm,
            inner_margin: Vec2::new(tokens.spacing.xs * scale, tokens.spacing.xs * scale),
            text_size: tokens.typography.body,
        }
    }
}

/// Resolved style for a `Table`.
#[derive(Clone, Copy, Debug)]
pub struct TableRecipe {
    /// Color of header cell text.
    pub header_color: Color32,
    /// Color of body cell text.
    pub cell_color: Color32,
    pub header_size: f32,
    pub cell_size: f32,
    /// Spacing between cells (x = columns, y = rows).
    pub spacing: Vec2,
}

impl TableRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        let scale = ctx.density_scale();
        Self {
            header_color: tokens.colors.text_muted,
            cell_color: tokens.colors.text,
            header_size: tokens.typography.label,
            cell_size: tokens.typography.body,
            spacing: Vec2::new(tokens.spacing.md * scale, tokens.spacing.sm * scale),
        }
    }
}

/// Resolved style for a `FormField` wrapper (label + control + hint/error).
#[derive(Clone, Copy, Debug)]
pub struct FormFieldRecipe {
    pub label_color: Color32,
    pub hint_color: Color32,
    pub error_color: Color32,
    pub label_size: f32,
    pub hint_size: f32,
    /// Vertical gap between label, control and the hint/error line.
    pub gap: f32,
}

impl FormFieldRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        Self {
            label_color: tokens.colors.text,
            hint_color: tokens.colors.text_muted,
            error_color: tokens.colors.danger,
            label_size: tokens.typography.label,
            hint_size: tokens.typography.label,
            gap: tokens.spacing.xs * ctx.density_scale(),
        }
    }
}

/// Resolved style for a `DropdownMenu` item list.
#[derive(Clone, Copy, Debug)]
pub struct MenuRecipe {
    pub text_color: Color32,
    pub text_size: f32,
}

impl MenuRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens) -> Self {
        Self {
            text_color: tokens.colors.text,
            text_size: tokens.typography.body,
        }
    }
}

/// Base height of a `DataGrid` header row before density scaling, in points.
const DATA_GRID_BASE_HEADER_HEIGHT: f32 = 26.0;
/// Base height of a `DataGrid` body row before density scaling, in points.
const DATA_GRID_BASE_ROW_HEIGHT: f32 = 22.0;

/// Resolved style for a `DataGrid` (virtualized, sortable table).
///
/// `header_height` / `row_height` are fixed row heights the grid needs up front
/// for virtualization (only visible rows are rendered).
#[derive(Clone, Copy, Debug)]
pub struct DataGridRecipe {
    /// Color of header cell text.
    pub header_color: Color32,
    /// Color of body cell text.
    pub cell_color: Color32,
    pub header_size: f32,
    pub cell_size: f32,
    /// Fixed header row height, in points.
    pub header_height: f32,
    /// Fixed body row height, in points (drives virtualization windowing).
    pub row_height: f32,
}

impl DataGridRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        let scale = ctx.density_scale();
        Self {
            header_color: tokens.colors.text_muted,
            cell_color: tokens.colors.text,
            header_size: tokens.typography.label,
            cell_size: tokens.typography.body,
            header_height: DATA_GRID_BASE_HEADER_HEIGHT * scale,
            row_height: DATA_GRID_BASE_ROW_HEIGHT * scale,
        }
    }
}

/// Resolved style for a `TreeView` (hierarchical, collapsible nodes).
///
/// Selection highlight is left to egui's themed selection visuals (set by
/// `install`); this recipe carries only the node text style and indent step.
#[derive(Clone, Copy, Debug)]
pub struct TreeViewRecipe {
    /// Text color of a node row.
    pub text_color: Color32,
    /// Text color of a selected node row.
    pub selected_color: Color32,
    pub text_size: f32,
    /// Horizontal indent applied per nesting level, in points.
    pub indent: f32,
}

impl TreeViewRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        Self {
            text_color: tokens.colors.text,
            selected_color: tokens.colors.primary,
            text_size: tokens.typography.body,
            indent: tokens.spacing.md * ctx.density_scale(),
        }
    }
}

/// Base square size of a `NumberInput` stepper button before density scaling, in points.
const NUMBER_INPUT_BASE_BUTTON: f32 = 24.0;
/// Base square size of a `ColorPicker` swatch trigger before density scaling, in points.
const COLOR_PICKER_BASE_SIZE: f32 = 24.0;

/// Resolved style for a `NumberInput` (a `DragValue` flanked by −/+ stepper buttons).
///
/// The numeric field itself is drawn by egui's `DragValue` (themed through the visuals
/// `install` derives from the tokens); this recipe styles only the stepper buttons and gap.
#[derive(Clone, Copy, Debug)]
pub struct NumberInputRecipe {
    /// Fill of the −/+ stepper buttons.
    pub button_fill: Color32,
    /// Glyph color of the −/+ stepper buttons.
    pub button_text: Color32,
    pub corner_radius: CornerRadius,
    /// Square size of each stepper button, in points.
    pub button_size: f32,
    /// Horizontal gap between the buttons and the numeric field, in points.
    pub gap: f32,
}

impl NumberInputRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        let scale = ctx.density_scale();
        Self {
            button_fill: tokens.colors.surface_variant,
            button_text: tokens.colors.text,
            corner_radius: tokens.radius.sm,
            button_size: NUMBER_INPUT_BASE_BUTTON * scale,
            gap: tokens.spacing.xs * scale,
        }
    }
}

/// Resolved style for a `ColorPicker` swatch trigger.
///
/// The picker popup itself is egui's deep color-picker module (themed by `install`); this
/// recipe styles only the clickable swatch that opens it.
#[derive(Clone, Copy, Debug)]
pub struct ColorPickerRecipe {
    /// Square size of the swatch, in points.
    pub size: f32,
    pub corner_radius: CornerRadius,
    /// Border around the swatch.
    pub border: Stroke,
}

impl ColorPickerRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        Self {
            size: COLOR_PICKER_BASE_SIZE * ctx.density_scale(),
            corner_radius: tokens.radius.sm,
            border: Stroke::new(1.0, tokens.colors.border),
        }
    }
}

/// Base width of a `Drawer` panel before density scaling, in points.
const DRAWER_BASE_WIDTH: f32 = 300.0;

/// Resolved style for a `Drawer` (off-canvas side panel over a scrim).
///
/// The scrim is theme-independent (provided by `egui::Modal`), so this recipe carries only the
/// token-derived panel surface, width and padding.
#[derive(Clone, Copy, Debug)]
pub struct DrawerRecipe {
    /// Panel surface fill.
    pub fill: Color32,
    /// Panel width, in points.
    pub width: f32,
    /// Padding inside the panel.
    pub inner_margin: Vec2,
}

impl DrawerRecipe {
    /// Pure resolution from tokens (cf. ADR-0009).
    #[must_use]
    pub fn resolve(tokens: &Tokens, ctx: &UiContext) -> Self {
        let scale = ctx.density_scale();
        Self {
            fill: tokens.colors.surface,
            width: DRAWER_BASE_WIDTH * scale,
            inner_margin: Vec2::new(tokens.spacing.lg * scale, tokens.spacing.lg * scale),
        }
    }
}
