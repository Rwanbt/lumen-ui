//! `DarkTheme` — the default dark theme.
//!
//! Holds a dark palette and delegates all recipe resolution to [`crate::builder`]
//! with `lighten` as the emphasis. Lives in `lumen-ui-core` for now; the full theme
//! family (Light, AudioDark, HighContrast) consolidates in `lumen-ui-themes` (v0.7).

use egui::Color32;

use crate::builder;
use crate::context::UiContext;
use crate::recipe::{
    BadgeRecipe, BadgeVariant, ButtonRecipe, ButtonVariant, CardRecipe, SliderRecipe,
    TextFieldRecipe, TextRecipe, TextRole, ToggleRecipe, WidgetState,
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
            // WCAG: white-on-primary clears AA (4.5:1) at rest — see a11y::audit_colors test.
            primary: Color32::from_rgb(0x44, 0x69, 0xb2),
            on_primary: Color32::from_rgb(0xf5, 0xf7, 0xfa),
            secondary: Color32::from_rgb(0x39, 0x3f, 0x4a),
            on_secondary: Color32::from_rgb(0xe6, 0xe9, 0xee),
            success: Color32::from_rgb(0x3f, 0xb9, 0x50),
            on_success: Color32::from_rgb(0x06, 0x1a, 0x0b),
            warning: Color32::from_rgb(0xe0, 0xa4, 0x2b),
            on_warning: Color32::from_rgb(0x1a, 0x12, 0x00),
            danger: Color32::from_rgb(0xc7, 0x3e, 0x43),
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
        builder::button(&self.tokens, builder::lighten, variant, state, ctx)
    }

    fn text_recipe(&self, role: TextRole, _ctx: &UiContext) -> TextRecipe {
        builder::text(&self.tokens, role)
    }

    fn card_recipe(&self, ctx: &UiContext) -> CardRecipe {
        builder::card(&self.tokens, ctx)
    }

    fn badge_recipe(&self, variant: BadgeVariant, ctx: &UiContext) -> BadgeRecipe {
        builder::badge(&self.tokens, variant, ctx)
    }

    fn toggle_recipe(&self, on: bool, state: WidgetState, _ctx: &UiContext) -> ToggleRecipe {
        builder::toggle(&self.tokens, builder::lighten, on, state)
    }

    fn slider_recipe(&self, state: WidgetState, _ctx: &UiContext) -> SliderRecipe {
        builder::slider(&self.tokens, builder::lighten, state)
    }

    fn text_field_recipe(&self, state: WidgetState, ctx: &UiContext) -> TextFieldRecipe {
        builder::text_field(&self.tokens, builder::lighten, state, ctx)
    }

    fn apply_to_ctx(&self, ctx: &egui::Context) {
        builder::apply_visuals(&self.tokens, true, builder::lighten, ctx);
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
    fn text_field_focus_highlights_border() {
        let theme = DarkTheme::new();
        let ctx = UiContext::default();
        let focused = theme.text_field_recipe(WidgetState::Focused, &ctx);
        let normal = theme.text_field_recipe(WidgetState::Normal, &ctx);
        assert_eq!(focused.border.color, theme.tokens().colors.primary);
        assert!(focused.border.width > normal.border.width);
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
