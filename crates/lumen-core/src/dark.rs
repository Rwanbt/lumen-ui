//! `DarkTheme` — the bootstrap theme that validates the whole pipeline in v0.1.
//!
//! Lives in `lumen-core` for now; the full theme family (Light, AudioDark,
//! HighContrast) moves to the `lumen-themes` crate in v0.7.

use egui::{Color32, Stroke};

use crate::context::UiContext;
use crate::recipe::{ButtonRecipe, ButtonVariant, WidgetState};
use crate::theme::Theme;
use crate::tokens::{Colors, Elevation, Motion, Radius, Spacing, Tokens, Typography};

/// A calm, neutral dark theme.
#[derive(Clone, Debug)]
pub struct DarkTheme {
    tokens: Tokens,
}

impl Default for DarkTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl DarkTheme {
    #[must_use]
    pub fn new() -> Self {
        let colors = Colors {
            background: Color32::from_rgb(0x12, 0x14, 0x18),
            surface: Color32::from_rgb(0x1b, 0x1e, 0x24),
            surface_variant: Color32::from_rgb(0x24, 0x28, 0x30),
            primary: Color32::from_rgb(0x5b, 0x8d, 0xef),
            on_primary: Color32::from_rgb(0xf5, 0xf7, 0xfa),
            secondary: Color32::from_rgb(0x39, 0x3f, 0x4a),
            on_secondary: Color32::from_rgb(0xe6, 0xe9, 0xee),
            danger: Color32::from_rgb(0xe5, 0x48, 0x4d),
            on_danger: Color32::from_rgb(0xff, 0xff, 0xff),
            text: Color32::from_rgb(0xe6, 0xe9, 0xee),
            text_muted: Color32::from_rgb(0x9a, 0xa2, 0xb0),
            border: Color32::from_rgb(0x33, 0x39, 0x44),
        };
        Self {
            tokens: Tokens {
                colors,
                spacing: Spacing::default(),
                radius: Radius::default(),
                typography: Typography::default(),
                elevation: Elevation::default(),
                motion: Motion::default(),
            },
        }
    }

    /// Lighten a color towards white by `t` in `[0, 1]` — used for hover states.
    fn lighten(c: Color32, t: f32) -> Color32 {
        let mix = |v: u8| (f32::from(v) + (255.0 - f32::from(v)) * t).round() as u8;
        Color32::from_rgb(mix(c.r()), mix(c.g()), mix(c.b()))
    }
}

impl Theme for DarkTheme {
    fn tokens(&self) -> &Tokens {
        &self.tokens
    }

    fn button_recipe(
        &self,
        variant: ButtonVariant,
        state: WidgetState,
        ctx: &UiContext,
    ) -> ButtonRecipe {
        let c = &self.tokens.colors;
        let (base_fill, text_color, has_border) = match variant {
            ButtonVariant::Primary => (c.primary, c.on_primary, false),
            ButtonVariant::Secondary => (c.secondary, c.on_secondary, false),
            ButtonVariant::Ghost => (Color32::TRANSPARENT, c.text, true),
            ButtonVariant::Danger => (c.danger, c.on_danger, false),
        };

        let fill = match state {
            WidgetState::Normal | WidgetState::Disabled => base_fill,
            WidgetState::Hovered => Self::lighten(base_fill, 0.10),
            WidgetState::Active => Self::lighten(base_fill, 0.18),
        };

        let stroke = if has_border {
            Stroke::new(1.0, c.border)
        } else {
            Stroke::NONE
        };

        let shadow = match (variant, state) {
            (ButtonVariant::Ghost, _) => self.tokens.elevation.none,
            (_, WidgetState::Hovered | WidgetState::Active) => self.tokens.elevation.low,
            _ => self.tokens.elevation.none,
        };

        let scale = ctx.density_scale();
        ButtonRecipe {
            fill,
            text_color,
            stroke,
            corner_radius: self.tokens.radius.md,
            shadow,
            inner_margin: Spacing::pad(
                self.tokens.spacing.md * scale,
                self.tokens.spacing.sm * scale,
            ),
        }
    }

    fn apply_to_ctx(&self, ctx: &egui::Context) {
        let c = &self.tokens.colors;
        ctx.global_style_mut(|style| {
            let v = &mut style.visuals;
            v.dark_mode = true;
            v.panel_fill = c.background;
            v.window_fill = c.surface;
            v.extreme_bg_color = c.background;
            v.override_text_color = Some(c.text);
            v.window_stroke = Stroke::new(1.0, c.border);

            let radius = self.tokens.radius.md;
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
            v.widgets.hovered.bg_fill = DarkTheme::lighten(c.surface_variant, 0.08);
            v.widgets.active.bg_fill = DarkTheme::lighten(c.surface_variant, 0.14);

            let s = &mut style.spacing;
            s.item_spacing = egui::vec2(self.tokens.spacing.sm, self.tokens.spacing.sm);
            s.button_padding = egui::vec2(self.tokens.spacing.md, self.tokens.spacing.sm);
        });
    }
}
