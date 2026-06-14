//! Shared recipe-resolution logic, parameterized by [`Tokens`] and an emphasis
//! function. Themes (`DarkTheme`, `LightTheme`, …) hold a palette and delegate
//! here, so the recipe rules live in **one** place (DRY).
//!
//! `emph(color, t)` shifts a base color for hover/active states: dark themes
//! lighten (towards white), light themes darken (towards black).

use egui::{Color32, Stroke};

use crate::context::UiContext;
use crate::recipe::{
    BadgeRecipe, BadgeVariant, ButtonRecipe, ButtonVariant, CardRecipe, SliderRecipe,
    TextFieldRecipe, TextRecipe, TextRole, ToggleRecipe, WidgetState,
};
use crate::tokens::{Spacing, Tokens};

/// Shifts a base color for emphasis (hover/active). `t` in `[0, 1]`.
pub(crate) type Emphasis = fn(Color32, f32) -> Color32;

/// Mix a color towards white by `t`.
pub(crate) fn lighten(c: Color32, t: f32) -> Color32 {
    let mix = |v: u8| (f32::from(v) + (255.0 - f32::from(v)) * t).round() as u8;
    Color32::from_rgb(mix(c.r()), mix(c.g()), mix(c.b()))
}

/// Mix a color towards black by `t`.
pub(crate) fn darken(c: Color32, t: f32) -> Color32 {
    let mix = |v: u8| (f32::from(v) * (1.0 - t)).round() as u8;
    Color32::from_rgb(mix(c.r()), mix(c.g()), mix(c.b()))
}

pub(crate) fn button(
    t: &Tokens,
    emph: Emphasis,
    variant: ButtonVariant,
    state: WidgetState,
    ctx: &UiContext,
) -> ButtonRecipe {
    let c = &t.colors;
    let (base_fill, text_color, has_border) = match variant {
        ButtonVariant::Primary => (c.primary, c.on_primary, false),
        ButtonVariant::Secondary => (c.secondary, c.on_secondary, false),
        ButtonVariant::Ghost => (Color32::TRANSPARENT, c.text, true),
        ButtonVariant::Danger => (c.danger, c.on_danger, false),
    };
    let fill = match state {
        WidgetState::Normal | WidgetState::Disabled | WidgetState::Focused => base_fill,
        WidgetState::Hovered => emph(base_fill, 0.10),
        WidgetState::Active => emph(base_fill, 0.18),
    };
    let stroke = if has_border {
        Stroke::new(1.0, c.border)
    } else {
        Stroke::NONE
    };
    let shadow = match (variant, state) {
        (ButtonVariant::Ghost, _) => t.elevation.none,
        (_, WidgetState::Hovered | WidgetState::Active) => t.elevation.low,
        _ => t.elevation.none,
    };
    let scale = ctx.density_scale();
    ButtonRecipe {
        fill,
        text_color,
        stroke,
        corner_radius: t.radius.md,
        shadow,
        inner_margin: Spacing::pad(t.spacing.md * scale, t.spacing.sm * scale),
    }
}

pub(crate) fn text(t: &Tokens, role: TextRole) -> TextRecipe {
    let c = &t.colors;
    let ty = &t.typography;
    match role {
        TextRole::Display => TextRecipe {
            color: c.text,
            size: ty.display,
        },
        TextRole::Heading => TextRecipe {
            color: c.text,
            size: ty.heading,
        },
        TextRole::Body => TextRecipe {
            color: c.text,
            size: ty.body,
        },
        TextRole::Label => TextRecipe {
            color: c.text,
            size: ty.label,
        },
        TextRole::Muted => TextRecipe {
            color: c.text_muted,
            size: ty.body,
        },
    }
}

pub(crate) fn card(t: &Tokens, ctx: &UiContext) -> CardRecipe {
    let c = &t.colors;
    let scale = ctx.density_scale();
    CardRecipe {
        fill: c.surface,
        stroke: Stroke::new(1.0, c.border),
        corner_radius: t.radius.lg,
        shadow: t.elevation.low,
        inner_margin: Spacing::pad(t.spacing.lg * scale, t.spacing.lg * scale),
    }
}

pub(crate) fn badge(t: &Tokens, variant: BadgeVariant, ctx: &UiContext) -> BadgeRecipe {
    let c = &t.colors;
    let (fill, text_color) = match variant {
        BadgeVariant::Neutral => (c.surface_variant, c.text_muted),
        BadgeVariant::Primary => (c.primary, c.on_primary),
        BadgeVariant::Success => (c.success, c.on_success),
        BadgeVariant::Warning => (c.warning, c.on_warning),
        BadgeVariant::Danger => (c.danger, c.on_danger),
    };
    let scale = ctx.density_scale();
    BadgeRecipe {
        fill,
        text_color,
        corner_radius: t.radius.full,
        inner_margin: Spacing::pad(t.spacing.sm * scale, t.spacing.xs * scale),
        text_size: t.typography.label,
    }
}

pub(crate) fn toggle(t: &Tokens, emph: Emphasis, on: bool, state: WidgetState) -> ToggleRecipe {
    let c = &t.colors;
    let base_track = if on { c.primary } else { c.surface_variant };
    let track = match state {
        WidgetState::Hovered | WidgetState::Active => emph(base_track, 0.10),
        _ => base_track,
    };
    let knob = if on { c.on_primary } else { c.text_muted };
    let border = if on {
        Stroke::NONE
    } else {
        Stroke::new(1.0, c.border)
    };
    ToggleRecipe {
        track,
        knob,
        border,
    }
}

pub(crate) fn slider(t: &Tokens, emph: Emphasis, state: WidgetState) -> SliderRecipe {
    let c = &t.colors;
    let fill = match state {
        WidgetState::Hovered | WidgetState::Active => emph(c.primary, 0.10),
        _ => c.primary,
    };
    SliderRecipe {
        track: c.surface_variant,
        fill,
        knob: c.on_primary,
    }
}

pub(crate) fn text_field(
    t: &Tokens,
    emph: Emphasis,
    state: WidgetState,
    ctx: &UiContext,
) -> TextFieldRecipe {
    let c = &t.colors;
    let border = match state {
        WidgetState::Focused => Stroke::new(1.5, c.primary),
        WidgetState::Hovered => Stroke::new(1.0, emph(c.border, 0.15)),
        _ => Stroke::new(1.0, c.border),
    };
    let scale = ctx.density_scale();
    TextFieldRecipe {
        fill: c.surface_variant,
        text_color: c.text,
        border,
        corner_radius: t.radius.md,
        inner_margin: Spacing::pad(t.spacing.sm * scale, t.spacing.sm * scale),
    }
}

/// Map tokens onto egui's global style so stock egui widgets follow the theme too.
pub(crate) fn apply_visuals(t: &Tokens, dark_mode: bool, emph: Emphasis, ctx: &egui::Context) {
    let c = &t.colors;
    ctx.global_style_mut(|style| {
        let v = &mut style.visuals;
        v.dark_mode = dark_mode;
        v.panel_fill = c.background;
        v.window_fill = c.surface;
        v.extreme_bg_color = c.background;
        v.override_text_color = Some(c.text);
        v.window_stroke = Stroke::new(1.0, c.border);

        let radius = t.radius.md;
        for w in [
            &mut v.widgets.noninteractive,
            &mut v.widgets.inactive,
            &mut v.widgets.hovered,
            &mut v.widgets.active,
            &mut v.widgets.open,
        ] {
            w.corner_radius = radius;
        }
        v.widgets.inactive.bg_fill = c.surface_variant;
        v.widgets.hovered.bg_fill = emph(c.surface_variant, 0.08);
        v.widgets.active.bg_fill = emph(c.surface_variant, 0.14);

        let s = &mut style.spacing;
        s.item_spacing = egui::vec2(t.spacing.sm, t.spacing.sm);
        s.button_padding = egui::vec2(t.spacing.md, t.spacing.sm);
    });
}
