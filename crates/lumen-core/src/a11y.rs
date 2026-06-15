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

use crate::tokens::Colors;

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

/// One foreground/background pair evaluated against a target conformance level.
#[derive(Clone, Copy, Debug)]
pub struct ContrastCheck {
    /// Human-readable name of the pair, e.g. `"text on surface"`.
    pub label: &'static str,
    pub foreground: Color32,
    pub background: Color32,
    /// Measured ratio (1.0–21.0).
    pub ratio: f32,
    /// The bar this pair is held to.
    pub level: ContrastLevel,
}

impl ContrastCheck {
    /// Does the measured ratio clear the required level?
    #[must_use]
    pub fn passes(&self) -> bool {
        self.ratio >= self.level.min_ratio()
    }
}

/// Result of auditing a [`Colors`] palette: one [`ContrastCheck`] per
/// text-bearing semantic pair.
#[derive(Clone, Debug)]
pub struct AuditReport {
    pub checks: Vec<ContrastCheck>,
}

impl AuditReport {
    /// True when every checked pair clears its level.
    #[must_use]
    pub fn all_pass(&self) -> bool {
        self.checks.iter().all(ContrastCheck::passes)
    }

    /// The pairs that failed, for diagnostics.
    pub fn failures(&self) -> impl Iterator<Item = &ContrastCheck> {
        self.checks.iter().filter(|c| !c.passes())
    }
}

/// Audit a palette's text-bearing pairs against WCAG AA.
///
/// Checks every place lumen widgets paint text on a fill: body/label text on the
/// background and surfaces, muted text, and each semantic `on_*` color over its
/// fill (button labels, badges). Foreground text is held to AA (4.5:1); large
/// display text would only need AaLarge, so AA is the stricter, safe bar.
///
/// `text_muted` is intentionally held to AA as well: it is used for real content
/// (captions, secondary labels), not disabled/incidental text exempt from 1.4.3.
#[must_use]
pub fn audit_colors(colors: &Colors) -> AuditReport {
    let aa = ContrastLevel::Aa;
    let pairs = [
        ("text on background", colors.text, colors.background),
        ("text on surface", colors.text, colors.surface),
        (
            "text on surface_variant",
            colors.text,
            colors.surface_variant,
        ),
        (
            "text_muted on background",
            colors.text_muted,
            colors.background,
        ),
        ("text_muted on surface", colors.text_muted, colors.surface),
        ("on_primary on primary", colors.on_primary, colors.primary),
        (
            "on_secondary on secondary",
            colors.on_secondary,
            colors.secondary,
        ),
        ("on_success on success", colors.on_success, colors.success),
        ("on_warning on warning", colors.on_warning, colors.warning),
        ("on_danger on danger", colors.on_danger, colors.danger),
    ];
    let checks = pairs
        .into_iter()
        .map(|(label, fg, bg)| ContrastCheck {
            label,
            foreground: fg,
            background: bg,
            ratio: contrast_ratio(fg, bg),
            level: aa,
        })
        .collect();
    AuditReport { checks }
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

    fn assert_palette_passes_aa(name: &str, colors: &Colors) {
        let report = audit_colors(colors);
        let failures: Vec<String> = report
            .failures()
            .map(|c| {
                format!(
                    "  {} — {:.2}:1 (need {:.1})",
                    c.label,
                    c.ratio,
                    c.level.min_ratio()
                )
            })
            .collect();
        assert!(
            report.all_pass(),
            "{name} fails WCAG AA on:\n{}",
            failures.join("\n")
        );
    }

    #[test]
    fn dark_theme_passes_wcag_aa() {
        use crate::{DarkTheme, Theme};
        assert_palette_passes_aa("DarkTheme", &DarkTheme::new().tokens().colors);
    }

    #[test]
    fn light_theme_passes_wcag_aa() {
        use crate::{LightTheme, Theme};
        assert_palette_passes_aa("LightTheme", &LightTheme::new().tokens().colors);
    }

    #[test]
    fn audit_flags_a_low_contrast_pair() {
        use crate::{DarkTheme, Theme};
        // A deliberately broken palette: muted text barely off the background.
        let mut colors = DarkTheme::new().tokens().colors.clone();
        colors.text_muted = colors.background;
        let report = audit_colors(&colors);
        assert!(!report.all_pass());
        assert!(report.failures().any(|c| c.label.contains("text_muted")));
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
