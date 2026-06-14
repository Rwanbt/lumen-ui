//! `lumen-themes` — extra themes for lumen-ui beyond the core `DarkTheme` /
//! `LightTheme`. Each is a [`lumen_core::PaletteTheme`]: a palette + a mode.
//!
//! Enable via the façade `themes` feature.
//!
//! ```ignore
//! use std::sync::Arc;
//! lumen_ui::install(ctx, Arc::new(lumen_themes::audio_dark()), Default::default());
//! ```

#![forbid(unsafe_code)]

use egui::Color32;
use lumen_core::{
    Colors, Elevation, Motion, PaletteTheme, Radius, Spacing, ThemeMode, Tokens, Typography,
};

fn tokens(colors: Colors) -> Tokens {
    Tokens {
        colors,
        spacing: Spacing::default(),
        radius: Radius::default(),
        typography: Typography::default(),
        elevation: Elevation::default(),
        motion: Motion::default(),
    }
}

/// A near-black, teal-accented dark theme tuned for audio / creative tools.
#[must_use]
pub fn audio_dark() -> PaletteTheme {
    let colors = Colors {
        background: Color32::from_rgb(0x0c, 0x0e, 0x10),
        surface: Color32::from_rgb(0x15, 0x18, 0x1b),
        surface_variant: Color32::from_rgb(0x1f, 0x23, 0x27),
        primary: Color32::from_rgb(0x2d, 0xd4, 0xbf),
        on_primary: Color32::from_rgb(0x03, 0x1a, 0x17),
        secondary: Color32::from_rgb(0x2a, 0x30, 0x36),
        on_secondary: Color32::from_rgb(0xe8, 0xed, 0xf0),
        success: Color32::from_rgb(0x3f, 0xb9, 0x50),
        on_success: Color32::from_rgb(0x03, 0x18, 0x0a),
        warning: Color32::from_rgb(0xe0, 0xa4, 0x2b),
        on_warning: Color32::from_rgb(0x1a, 0x12, 0x00),
        danger: Color32::from_rgb(0xe5, 0x48, 0x4d),
        on_danger: Color32::from_rgb(0xff, 0xff, 0xff),
        text: Color32::from_rgb(0xe8, 0xed, 0xf0),
        text_muted: Color32::from_rgb(0x8a, 0x94, 0x9c),
        border: Color32::from_rgb(0x29, 0x2f, 0x35),
    };
    PaletteTheme::new(tokens(colors), ThemeMode::Dark)
}

/// A maximum-contrast dark theme (WCAG-friendly): pure black, white text, bright accent.
#[must_use]
pub fn high_contrast() -> PaletteTheme {
    let colors = Colors {
        background: Color32::BLACK,
        surface: Color32::from_rgb(0x0a, 0x0a, 0x0a),
        surface_variant: Color32::from_rgb(0x1a, 0x1a, 0x1a),
        primary: Color32::from_rgb(0x4d, 0xa3, 0xff),
        on_primary: Color32::BLACK,
        secondary: Color32::from_rgb(0x2a, 0x2a, 0x2a),
        on_secondary: Color32::WHITE,
        success: Color32::from_rgb(0x3d, 0xff, 0x6e),
        on_success: Color32::BLACK,
        warning: Color32::from_rgb(0xff, 0xd5, 0x4d),
        on_warning: Color32::BLACK,
        danger: Color32::from_rgb(0xff, 0x5a, 0x5f),
        on_danger: Color32::BLACK,
        text: Color32::WHITE,
        text_muted: Color32::from_rgb(0xc8, 0xc8, 0xc8),
        border: Color32::from_rgb(0x88, 0x88, 0x88),
    };
    PaletteTheme::new(tokens(colors), ThemeMode::Dark)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lumen_core::{Theme, UiContext};

    #[test]
    fn themes_expose_their_accent() {
        let ctx = UiContext::default();
        // audio_dark uses a teal primary; high_contrast a bright blue — both non-black.
        assert_ne!(audio_dark().tokens().colors.primary, Color32::BLACK);
        assert_ne!(high_contrast().tokens().colors.text, Color32::BLACK);
        // Recipe resolution works through PaletteTheme.
        let r = audio_dark().button_recipe(
            lumen_core::ButtonVariant::Primary,
            lumen_core::WidgetState::Normal,
            &ctx,
        );
        assert_eq!(r.fill, audio_dark().tokens().colors.primary);
    }
}
