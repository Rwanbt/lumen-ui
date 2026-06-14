//! `DarkTheme` — the bootstrap theme that validates the whole pipeline in v0.1.
//!
//! Lives in `lumen-core` for now; the full theme family (Light, AudioDark,
//! HighContrast) moves to the `lumen-themes` crate in v0.7.

use egui::{Color32, Stroke};

use crate::context::UiContext;
use crate::recipe::{
    BadgeRecipe, BadgeVariant, ButtonRecipe, ButtonVariant, CardRecipe, SliderRecipe, TextRecipe,
    TextRole, ToggleRecipe, WidgetState,
};
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
            success: Color32::from_rgb(0x3f, 0xb9, 0x50),
            on_success: Color32::from_rgb(0x06, 0x1a, 0x0b),
            warning: Color32::from_rgb(0xe0, 0xa4, 0x2b),
            on_warning: Color32::from_rgb(0x1a, 0x12, 0x00),
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

    fn text_recipe(&self, role: TextRole, _ctx: &UiContext) -> TextRecipe {
        let c = &self.tokens.colors;
        let t = &self.tokens.typography;
        match role {
            TextRole::Display => TextRecipe {
                color: c.text,
                size: t.display,
            },
            TextRole::Heading => TextRecipe {
                color: c.text,
                size: t.heading,
            },
            TextRole::Body => TextRecipe {
                color: c.text,
                size: t.body,
            },
            TextRole::Label => TextRecipe {
                color: c.text,
                size: t.label,
            },
            TextRole::Muted => TextRecipe {
                color: c.text_muted,
                size: t.body,
            },
        }
    }

    fn card_recipe(&self, ctx: &UiContext) -> CardRecipe {
        let c = &self.tokens.colors;
        let scale = ctx.density_scale();
        CardRecipe {
            fill: c.surface,
            stroke: Stroke::new(1.0, c.border),
            corner_radius: self.tokens.radius.lg,
            shadow: self.tokens.elevation.low,
            inner_margin: Spacing::pad(
                self.tokens.spacing.lg * scale,
                self.tokens.spacing.lg * scale,
            ),
        }
    }

    fn badge_recipe(&self, variant: BadgeVariant, ctx: &UiContext) -> BadgeRecipe {
        let c = &self.tokens.colors;
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
            corner_radius: self.tokens.radius.full,
            inner_margin: Spacing::pad(
                self.tokens.spacing.sm * scale,
                self.tokens.spacing.xs * scale,
            ),
            text_size: self.tokens.typography.label,
        }
    }

    fn toggle_recipe(&self, on: bool, state: WidgetState, _ctx: &UiContext) -> ToggleRecipe {
        let c = &self.tokens.colors;
        let base_track = if on { c.primary } else { c.surface_variant };
        let track = match state {
            WidgetState::Hovered | WidgetState::Active => Self::lighten(base_track, 0.10),
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

    fn slider_recipe(&self, state: WidgetState, _ctx: &UiContext) -> SliderRecipe {
        let c = &self.tokens.colors;
        let (fill, knob) = match state {
            WidgetState::Hovered | WidgetState::Active => {
                (Self::lighten(c.primary, 0.10), Color32::WHITE)
            }
            _ => (c.primary, c.on_primary),
        };
        SliderRecipe {
            track: c.surface_variant,
            fill,
            knob,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Density;

    #[test]
    fn text_recipe_maps_role_to_tokens() {
        let theme = DarkTheme::new();
        let ctx = UiContext::default();
        let t = theme.tokens();

        let display = theme.text_recipe(TextRole::Display, &ctx);
        assert_eq!(display.size, t.typography.display);
        assert_eq!(display.color, t.colors.text);

        let muted = theme.text_recipe(TextRole::Muted, &ctx);
        assert_eq!(muted.color, t.colors.text_muted);
    }

    #[test]
    fn button_hover_brightens_fill() {
        let theme = DarkTheme::new();
        let ctx = UiContext::default();
        let normal = theme.button_recipe(ButtonVariant::Primary, WidgetState::Normal, &ctx);
        let hovered = theme.button_recipe(ButtonVariant::Primary, WidgetState::Hovered, &ctx);
        assert_ne!(
            normal.fill, hovered.fill,
            "hover state must change the fill"
        );
    }

    #[test]
    fn ghost_button_is_transparent_with_border() {
        let theme = DarkTheme::new();
        let ctx = UiContext::default();
        let ghost = theme.button_recipe(ButtonVariant::Ghost, WidgetState::Normal, &ctx);
        assert_eq!(ghost.fill, egui::Color32::TRANSPARENT);
        assert!(
            ghost.stroke.width > 0.0,
            "ghost variant must have a visible border"
        );
    }

    #[test]
    fn badge_variants_map_to_semantic_colors() {
        let theme = DarkTheme::new();
        let ctx = UiContext::default();
        let t = theme.tokens();
        assert_eq!(
            theme.badge_recipe(BadgeVariant::Success, &ctx).fill,
            t.colors.success
        );
        assert_eq!(
            theme.badge_recipe(BadgeVariant::Danger, &ctx).fill,
            t.colors.danger
        );
        // Neutral badges use the muted text color, not the strong text color.
        assert_eq!(
            theme.badge_recipe(BadgeVariant::Neutral, &ctx).text_color,
            t.colors.text_muted
        );
    }

    #[test]
    fn toggle_on_uses_primary_track_no_border() {
        let theme = DarkTheme::new();
        let ctx = UiContext::default();
        let on = theme.toggle_recipe(true, WidgetState::Normal, &ctx);
        let off = theme.toggle_recipe(false, WidgetState::Normal, &ctx);
        assert_eq!(on.track, theme.tokens().colors.primary);
        assert_eq!(on.border.width, 0.0, "the on state has no border");
        assert!(off.border.width > 0.0, "the off state has a visible border");
        assert_ne!(on.track, off.track);
    }

    #[test]
    fn slider_fill_uses_primary() {
        let theme = DarkTheme::new();
        let ctx = UiContext::default();
        let r = theme.slider_recipe(WidgetState::Normal, &ctx);
        assert_eq!(r.fill, theme.tokens().colors.primary);
        assert_ne!(r.track, r.fill, "track and fill must be visually distinct");
    }

    #[test]
    fn touch_density_enlarges_padding() {
        let theme = DarkTheme::new();
        let comfortable = theme.button_recipe(
            ButtonVariant::Primary,
            WidgetState::Normal,
            &UiContext {
                density: Density::Comfortable,
            },
        );
        let touch = theme.button_recipe(
            ButtonVariant::Primary,
            WidgetState::Normal,
            &UiContext {
                density: Density::Touch,
            },
        );
        assert!(touch.inner_margin.x > comfortable.inner_margin.x);
    }
}
