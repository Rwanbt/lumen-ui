//! Accessibility helpers — WCAG 2.1 contrast math over [`egui::Color32`].
//!
//! These are pure functions: given two colors, decide whether their contrast
//! meets a [`ContrastLevel`]. The theme layer uses them to *audit* its semantic
//! color pairs (text-on-surface, on_primary-on-primary…) so a theme that fails
//! AA is caught by a test rather than by a user who cannot read it.
//!
//! Reference: <https://www.w3.org/TR/WCAG21/#contrast-minimum> (1.4.3) and
//! <https://www.w3.org/TR/WCAG21/#contrast-enhanced> (1.4.6).

use egui::Color32;

/// WCAG conformance target for a text/background pair.
///
/// The minimum contrast ratio depends on text size: "large" text (≥ 18 pt, or
/// ≥ 14 pt bold) is held to a lower bar than body text.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ContrastLevel {
    /// AA, normal text — 4.5:1.
    Aa,
    /// AA, large text (≥ 18 pt / 14 pt bold) — 3:1.
    AaLarge,
    /// AAA, normal text — 7:1.
    Aaa,
    /// AAA, large text — 4.5:1.
    AaaLarge,
}

impl ContrastLevel {
    /// The minimum contrast ratio required to satisfy this level.
    #[must_use]
    pub fn min_ratio(self) -> f32 {
        match self {
            ContrastLevel::Aa | ContrastLevel::AaaLarge => 4.5,
            ContrastLevel::AaLarge => 3.0,
            ContrastLevel::Aaa => 7.0,
        }
    }
}

/// Relative luminance of a single sRGB channel, per WCAG 2.1.
fn linearize(channel: u8) -> f32 {
    let c = f32::from(channel) / 255.0;
    if c <= 0.039_28 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Relative luminance of a color in `[0.0, 1.0]`, per WCAG 2.1 (alpha ignored).
///
/// `0.0` is black, `1.0` is white. This is the perceptual lightness used by the
/// contrast-ratio formula, not a naive average of the channels.
#[must_use]
pub fn relative_luminance(color: Color32) -> f32 {
    0.2126 * linearize(color.r()) + 0.7152 * linearize(color.g()) + 0.0722 * linearize(color.b())
}

/// Contrast ratio between two colors, in `[1.0, 21.0]` (WCAG 2.1).
///
/// Order-independent: `contrast_ratio(a, b) == contrast_ratio(b, a)`. A ratio of
/// 1.0 means identical luminance; 21.0 is pure black against pure white.
#[must_use]
pub fn contrast_ratio(a: Color32, b: Color32) -> f32 {
    let la = relative_luminance(a);
    let lb = relative_luminance(b);
    let (lighter, darker) = if la >= lb { (la, lb) } else { (lb, la) };
    (lighter + 0.05) / (darker + 0.05)
}

/// Whether `foreground` over `background` meets the given conformance `level`.
#[must_use]
pub fn meets(foreground: Color32, background: Color32, level: ContrastLevel) -> bool {
    contrast_ratio(foreground, background) >= level.min_ratio()
}

/// Convenience: does this pair meet AA for normal body text (4.5:1)?
#[must_use]
pub fn meets_aa(foreground: Color32, background: Color32) -> bool {
    meets(foreground, background, ContrastLevel::Aa)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_on_white_is_max_contrast() {
        let ratio = contrast_ratio(Color32::BLACK, Color32::WHITE);
        assert!((ratio - 21.0).abs() < 0.01, "expected ~21, got {ratio}");
    }

    #[test]
    fn identical_colors_have_ratio_one() {
        let ratio = contrast_ratio(
            Color32::from_rgb(0x33, 0x66, 0x99),
            Color32::from_rgb(0x33, 0x66, 0x99),
        );
        assert!((ratio - 1.0).abs() < 0.0001, "expected 1.0, got {ratio}");
    }

    #[test]
    fn contrast_is_symmetric() {
        let a = Color32::from_rgb(0x12, 0x34, 0x56);
        let b = Color32::from_rgb(0xab, 0xcd, 0xef);
        assert!((contrast_ratio(a, b) - contrast_ratio(b, a)).abs() < 1e-6);
    }

    #[test]
    fn luminance_is_ordered() {
        assert!(relative_luminance(Color32::BLACK) < relative_luminance(Color32::from_gray(128)));
        assert!(relative_luminance(Color32::from_gray(128)) < relative_luminance(Color32::WHITE));
    }

    #[test]
    fn level_thresholds_match_wcag() {
        assert_eq!(ContrastLevel::Aa.min_ratio(), 4.5);
        assert_eq!(ContrastLevel::AaLarge.min_ratio(), 3.0);
        assert_eq!(ContrastLevel::Aaa.min_ratio(), 7.0);
        assert_eq!(ContrastLevel::AaaLarge.min_ratio(), 4.5);
    }

    #[test]
    fn meets_aa_passes_and_fails_at_boundary() {
        // White on a mid-grey: should pass AA-large but we assert the helper plumbing.
        assert!(meets_aa(Color32::WHITE, Color32::BLACK));
        assert!(!meets_aa(Color32::from_gray(180), Color32::WHITE));
    }

    #[test]
    fn aa_large_is_weaker_than_aa() {
        // A pair that clears 3:1 but not 4.5:1 satisfies AaLarge only.
        let fg = Color32::from_gray(120);
        let bg = Color32::WHITE;
        let ratio = contrast_ratio(fg, bg);
        assert!(
            ratio >= 3.0 && ratio < 4.5,
            "ratio {ratio} not in the AA-large band"
        );
        assert!(meets(fg, bg, ContrastLevel::AaLarge));
        assert!(!meets(fg, bg, ContrastLevel::Aa));
    }
}
