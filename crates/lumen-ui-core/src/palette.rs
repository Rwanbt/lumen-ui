//! [`PaletteTheme`] — a reusable [`Theme`] that is fully determined by its
//! [`Tokens`] palette plus a light/dark [`ThemeMode`].
//!
//! This is how new themes are built without re-implementing recipe resolution:
//! provide a palette, pick a mode, done. `DarkTheme`/`LightTheme` and the
//! `lumen-ui-themes` family are all thin wrappers around this.

use crate::builder;
use crate::context::UiContext;
use crate::recipe::{
    BadgeRecipe, BadgeVariant, ButtonRecipe, ButtonVariant, CardRecipe, SliderRecipe,
    TextFieldRecipe, TextRecipe, TextRole, ToggleRecipe, WidgetState,
};
use crate::theme::Theme;
use crate::tokens::Tokens;

/// Whether a palette is light or dark — selects the hover/active emphasis
/// direction (dark themes lighten, light themes darken) and egui's `dark_mode`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThemeMode {
    Dark,
    Light,
}

impl ThemeMode {
    pub(crate) fn emphasis(self) -> builder::Emphasis {
        match self {
            ThemeMode::Dark => builder::lighten,
            ThemeMode::Light => builder::darken,
        }
    }

    fn is_dark(self) -> bool {
        matches!(self, ThemeMode::Dark)
    }
}

/// A theme defined entirely by a [`Tokens`] palette + a [`ThemeMode`].
#[derive(Clone, Debug)]
pub struct PaletteTheme {
    tokens: Tokens,
    mode: ThemeMode,
}

impl PaletteTheme {
    #[must_use]
    pub fn new(tokens: Tokens, mode: ThemeMode) -> Self {
        Self { tokens, mode }
    }
}

impl Theme for PaletteTheme {
    fn tokens(&self) -> &Tokens {
        &self.tokens
    }

    fn mode(&self) -> ThemeMode {
        self.mode
    }

    fn button_recipe(
        &self,
        variant: ButtonVariant,
        state: WidgetState,
        ctx: &UiContext,
    ) -> ButtonRecipe {
        builder::button(&self.tokens, self.mode.emphasis(), variant, state, ctx)
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
        builder::toggle(&self.tokens, self.mode.emphasis(), on, state)
    }

    fn slider_recipe(&self, state: WidgetState, _ctx: &UiContext) -> SliderRecipe {
        builder::slider(&self.tokens, self.mode.emphasis(), state)
    }

    fn text_field_recipe(&self, state: WidgetState, ctx: &UiContext) -> TextFieldRecipe {
        builder::text_field(&self.tokens, self.mode.emphasis(), state, ctx)
    }

    fn apply_to_ctx(&self, ctx: &egui::Context) {
        builder::apply_visuals(&self.tokens, self.mode.is_dark(), self.mode.emphasis(), ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DarkTheme, LightTheme};

    #[test]
    fn default_mode_classifies_by_background_luminance() {
        // DarkTheme/LightTheme do not override `mode()`; they rely on the luminance default.
        assert_eq!(DarkTheme::new().mode(), ThemeMode::Dark);
        assert_eq!(LightTheme::new().mode(), ThemeMode::Light);
    }

    #[test]
    fn palette_theme_honors_explicit_mode() {
        // A PaletteTheme must report its declared mode regardless of background luminance.
        let dark_palette = PaletteTheme::new(DarkTheme::new().tokens().clone(), ThemeMode::Dark);
        let light_palette = PaletteTheme::new(LightTheme::new().tokens().clone(), ThemeMode::Light);
        assert_eq!(dark_palette.mode(), ThemeMode::Dark);
        assert_eq!(light_palette.mode(), ThemeMode::Light);
    }
}
