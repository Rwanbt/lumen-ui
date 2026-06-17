//! `lumen-ui-themes` — extra themes for lumen-ui beyond the core `DarkTheme` /
//! `LightTheme`. Each is a [`lumen_ui_core::PaletteTheme`]: a palette + a mode.
//!
//! Enable via the façade `themes` feature.
//!
//! ```ignore
//! use std::sync::Arc;
//! lumen_ui::install(ctx, Arc::new(lumen_ui_themes::audio_dark()), Default::default());
//! ```

#![forbid(unsafe_code)]

use egui::Color32;
use lumen_ui_core::{
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
        // WCAG: white-on-danger clears AA at rest — see the audit test below.
        danger: Color32::from_rgb(0xc7, 0x3e, 0x43),
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

/// The [Nord](https://www.nordtheme.com/) palette as a dark theme (Polar Night surfaces, Frost
/// accent, Aurora semantics). Accent/semantic colors are tuned where needed so every pair clears
/// WCAG AA (the `danger` red is darkened from Aurora `nord11` so white text passes).
#[must_use]
pub fn nord() -> PaletteTheme {
    let colors = Colors {
        background: Color32::from_rgb(0x2e, 0x34, 0x40), // nord0
        surface: Color32::from_rgb(0x3b, 0x42, 0x52),    // nord1
        surface_variant: Color32::from_rgb(0x43, 0x4c, 0x5e), // nord2
        primary: Color32::from_rgb(0x88, 0xc0, 0xd0),    // nord8
        on_primary: Color32::from_rgb(0x2e, 0x34, 0x40),
        secondary: Color32::from_rgb(0x4c, 0x56, 0x6a), // nord3
        on_secondary: Color32::from_rgb(0xec, 0xef, 0xf4), // nord6
        success: Color32::from_rgb(0xa3, 0xbe, 0x8c),   // nord14
        on_success: Color32::from_rgb(0x2e, 0x34, 0x40),
        warning: Color32::from_rgb(0xeb, 0xcb, 0x8b), // nord13
        on_warning: Color32::from_rgb(0x2e, 0x34, 0x40),
        danger: Color32::from_rgb(0x9c, 0x3c, 0x44), // darkened nord11 so white clears AA
        on_danger: Color32::from_rgb(0xec, 0xef, 0xf4),
        text: Color32::from_rgb(0xec, 0xef, 0xf4), // nord6
        text_muted: Color32::from_rgb(0xc8, 0xce, 0xda), // lightened nord4 for AA on surfaces
        border: Color32::from_rgb(0x4c, 0x56, 0x6a),
    };
    PaletteTheme::new(tokens(colors), ThemeMode::Dark)
}

/// The [Solarized](https://ethanschoonover.com/solarized/) palette as a dark theme (base03 ground,
/// base0 body text). Accents/semantics tuned where needed to clear WCAG AA.
#[must_use]
pub fn solarized_dark() -> PaletteTheme {
    let colors = Colors {
        background: Color32::from_rgb(0x00, 0x2b, 0x36), // base03
        surface: Color32::from_rgb(0x07, 0x36, 0x42),    // base02
        surface_variant: Color32::from_rgb(0x0c, 0x41, 0x4f),
        primary: Color32::from_rgb(0x4a, 0xa3, 0xd6), // brightened blue for AA on dark + dark text
        on_primary: Color32::from_rgb(0x00, 0x2b, 0x36),
        secondary: Color32::from_rgb(0x58, 0x6e, 0x75), // base01
        on_secondary: Color32::from_rgb(0xfd, 0xf6, 0xe3), // base3
        success: Color32::from_rgb(0x85, 0x99, 0x00),   // green
        on_success: Color32::from_rgb(0x00, 0x2b, 0x36),
        warning: Color32::from_rgb(0xb5, 0x89, 0x00), // yellow
        on_warning: Color32::from_rgb(0x00, 0x2b, 0x36), // dark text on yellow for AA
        danger: Color32::from_rgb(0xa4, 0x31, 0x2a),  // darkened red so light text clears AA
        on_danger: Color32::from_rgb(0xfd, 0xf6, 0xe3),
        text: Color32::from_rgb(0xee, 0xe8, 0xd5), // base2 (brighter than base0 for AA)
        text_muted: Color32::from_rgb(0x93, 0xa1, 0xa1), // base1
        border: Color32::from_rgb(0x0c, 0x41, 0x4f),
    };
    PaletteTheme::new(tokens(colors), ThemeMode::Dark)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lumen_ui_core::{Theme, UiContext};

    #[test]
    fn themes_expose_their_accent() {
        let ctx = UiContext::default();
        // audio_dark uses a teal primary; high_contrast a bright blue — both non-black.
        assert_ne!(audio_dark().tokens().colors.primary, Color32::BLACK);
        assert_ne!(high_contrast().tokens().colors.text, Color32::BLACK);
        // Recipe resolution works through PaletteTheme.
        let r = audio_dark().button_recipe(
            lumen_ui_core::ButtonVariant::Primary,
            lumen_ui_core::WidgetState::Normal,
            &ctx,
        );
        assert_eq!(r.fill, audio_dark().tokens().colors.primary);
    }

    fn assert_aa(name: &str, theme: &PaletteTheme) {
        let report = lumen_ui_core::audit_colors(&theme.tokens().colors);
        let failures: Vec<String> = report
            .failures()
            .map(|c| format!("  {} — {:.2}:1", c.label, c.ratio))
            .collect();
        assert!(
            report.all_pass(),
            "{name} fails WCAG AA:\n{}",
            failures.join("\n")
        );
    }

    #[test]
    fn audio_dark_passes_wcag_aa() {
        assert_aa("audio_dark", &audio_dark());
    }

    #[test]
    fn high_contrast_passes_wcag_aa() {
        assert_aa("high_contrast", &high_contrast());
    }

    #[test]
    fn nord_passes_wcag_aa() {
        assert_aa("nord", &nord());
    }

    #[test]
    fn solarized_dark_passes_wcag_aa() {
        assert_aa("solarized_dark", &solarized_dark());
    }
}
