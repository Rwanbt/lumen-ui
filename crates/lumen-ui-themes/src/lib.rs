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
    contrast_ratio, relative_luminance, Colors, Elevation, Motion, PaletteTheme, Radius, Spacing,
    ThemeMode, Tokens, Typography,
};

/// Near-white used as a derived foreground when the background is dark.
const NEAR_WHITE: Color32 = Color32::from_rgb(0xf2, 0xf4, 0xf6);
/// Near-black used as a derived foreground when the background is light.
const NEAR_BLACK: Color32 = Color32::from_rgb(0x0c, 0x0e, 0x10);
/// Luminance threshold separating "dark" from "light" backgrounds.
const DARK_LUMINANCE_MAX: f32 = 0.5;

/// Linearly interpolate two colors per RGB channel (`t` in `0..=1`).
fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let chan = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * t).round() as u8;
    Color32::from_rgb(chan(a.r(), b.r()), chan(a.g(), b.g()), chan(a.b(), b.b()))
}

/// Pick near-white or near-black for the best contrast over `background`.
fn readable_on(background: Color32) -> Color32 {
    if contrast_ratio(NEAR_WHITE, background) >= contrast_ratio(NEAR_BLACK, background) {
        NEAR_WHITE
    } else {
        NEAR_BLACK
    }
}

/// The `ThemeMode` that matches the OS preference reported by egui, defaulting to dark when the
/// system theme is unknown (e.g. headless). Use to drive `prefers-color-scheme`-style theming.
#[must_use]
pub fn system_mode(ctx: &egui::Context) -> ThemeMode {
    match ctx.system_theme() {
        Some(egui::Theme::Light) => ThemeMode::Light,
        _ => ThemeMode::Dark,
    }
}

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

/// Derives a complete, WCAG-AA-oriented [`PaletteTheme`] from just a **background** and an
/// **accent** color: surfaces/borders are interpolated from the background toward a readable
/// foreground, the text and every `on_*` color are picked (near-white/near-black) for contrast,
/// and the semantic colors use sensible defaults. The mode is inferred from the background's
/// luminance unless set with [`ThemeBuilder::mode`].
///
/// For typical (clearly dark or light) backgrounds the result clears AA; very mid-tone backgrounds
/// can't satisfy contrast both ways, so audit the result with `lumen_ui_core::audit_colors` if you
/// feed unusual seeds.
#[derive(Clone, Copy, Debug)]
pub struct ThemeBuilder {
    background: Color32,
    accent: Color32,
    mode: Option<ThemeMode>,
}

impl ThemeBuilder {
    /// Start from a background and an accent (primary) color.
    #[must_use]
    pub fn new(background: Color32, accent: Color32) -> Self {
        Self {
            background,
            accent,
            mode: None,
        }
    }

    /// Force the emphasis mode (otherwise inferred from the background luminance).
    #[must_use]
    pub fn mode(mut self, mode: ThemeMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Build the derived theme.
    #[must_use]
    pub fn build(self) -> PaletteTheme {
        let mode = self.mode.unwrap_or(
            if relative_luminance(self.background) < DARK_LUMINANCE_MAX {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
        );
        let bg = self.background;
        let text = readable_on(bg);
        // Fixed semantic hues (dark enough that derived on_* text clears AA).
        let success = Color32::from_rgb(0x3f, 0xb9, 0x50);
        let warning = Color32::from_rgb(0xe0, 0xa4, 0x2b);
        let danger = Color32::from_rgb(0xc7, 0x3e, 0x43);
        let surface_variant = mix(bg, text, 0.11);
        let colors = Colors {
            background: bg,
            surface: mix(bg, text, 0.05),
            surface_variant,
            primary: self.accent,
            on_primary: readable_on(self.accent),
            secondary: surface_variant,
            on_secondary: text,
            success,
            on_success: readable_on(success),
            warning,
            on_warning: readable_on(warning),
            danger,
            on_danger: readable_on(danger),
            text,
            text_muted: mix(text, bg, 0.35),
            border: mix(bg, text, 0.22),
        };
        PaletteTheme::new(tokens(colors), mode)
    }
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

    #[test]
    fn builder_derives_aa_themes_from_seeds() {
        // A dark seed and a light seed both yield AA-passing palettes.
        let dark = ThemeBuilder::new(
            Color32::from_rgb(0x10, 0x12, 0x16),
            Color32::from_rgb(0x6c, 0x8c, 0xff),
        )
        .build();
        assert_aa("builder/dark", &dark);

        let light = ThemeBuilder::new(
            Color32::from_rgb(0xf7, 0xf8, 0xfa),
            Color32::from_rgb(0x14, 0x52, 0xcc), // a deep accent so on_primary clears AA
        )
        .build();
        assert_aa("builder/light", &light);
    }

    #[test]
    fn builder_infers_mode_from_background_luminance() {
        let dark = ThemeBuilder::new(Color32::from_rgb(0x10, 0x12, 0x16), Color32::WHITE).build();
        assert_eq!(dark.mode(), ThemeMode::Dark);
        let light = ThemeBuilder::new(Color32::from_rgb(0xf7, 0xf8, 0xfa), Color32::BLACK).build();
        assert_eq!(light.mode(), ThemeMode::Light);
    }
}
