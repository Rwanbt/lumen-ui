//! `LightTheme` — a light counterpart to [`crate::DarkTheme`].
//!
//! Same recipe rules (via [`crate::builder`]); only the palette and the emphasis
//! direction differ (light themes `darken` on hover instead of lightening).

use egui::Color32;

use crate::builder;
use crate::context::UiContext;
use crate::recipe::{
    BadgeRecipe, BadgeVariant, ButtonRecipe, ButtonVariant, CardRecipe, SliderRecipe,
    TextFieldRecipe, TextRecipe, TextRole, ToggleRecipe, WidgetState,
};
use crate::theme::Theme;
use crate::tokens::{Colors, Elevation, Motion, Radius, Spacing, Tokens, Typography};

/// A clean, neutral light theme.
#[derive(Clone, Debug)]
pub struct LightTheme {
    tokens: Tokens,
}

impl Default for LightTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl LightTheme {
    #[must_use]
    pub fn new() -> Self {
        let colors = Colors {
            background: Color32::from_rgb(0xf6, 0xf7, 0xf9),
            surface: Color32::from_rgb(0xff, 0xff, 0xff),
            surface_variant: Color32::from_rgb(0xec, 0xee, 0xf2),
            primary: Color32::from_rgb(0x2f, 0x6f, 0xe0),
            on_primary: Color32::from_rgb(0xff, 0xff, 0xff),
            secondary: Color32::from_rgb(0xe4, 0xe7, 0xec),
            on_secondary: Color32::from_rgb(0x1b, 0x1e, 0x24),
            success: Color32::from_rgb(0x1f, 0x9d, 0x3a),
            on_success: Color32::from_rgb(0xff, 0xff, 0xff),
            warning: Color32::from_rgb(0xb8, 0x7a, 0x00),
            on_warning: Color32::from_rgb(0xff, 0xff, 0xff),
            danger: Color32::from_rgb(0xd8, 0x36, 0x3b),
            on_danger: Color32::from_rgb(0xff, 0xff, 0xff),
            text: Color32::from_rgb(0x1b, 0x1e, 0x24),
            text_muted: Color32::from_rgb(0x5b, 0x64, 0x72),
            border: Color32::from_rgb(0xd4, 0xd8, 0xe0),
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

impl Theme for LightTheme {
    fn tokens(&self) -> &Tokens {
        &self.tokens
    }

    fn button_recipe(
        &self,
        variant: ButtonVariant,
        state: WidgetState,
        ctx: &UiContext,
    ) -> ButtonRecipe {
        builder::button(&self.tokens, builder::darken, variant, state, ctx)
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
        builder::toggle(&self.tokens, builder::darken, on, state)
    }

    fn slider_recipe(&self, state: WidgetState, _ctx: &UiContext) -> SliderRecipe {
        builder::slider(&self.tokens, builder::darken, state)
    }

    fn text_field_recipe(&self, state: WidgetState, ctx: &UiContext) -> TextFieldRecipe {
        builder::text_field(&self.tokens, builder::darken, state, ctx)
    }

    fn apply_to_ctx(&self, ctx: &egui::Context) {
        builder::apply_visuals(&self.tokens, false, builder::darken, ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_is_brighter_than_dark() {
        let light = LightTheme::new();
        let dark = crate::DarkTheme::new();
        // A crude luminance proxy: light background channels far exceed dark's.
        assert!(light.tokens().colors.background.r() > dark.tokens().colors.background.r());
        assert!(light.tokens().colors.text.r() < light.tokens().colors.background.r());
    }
}
