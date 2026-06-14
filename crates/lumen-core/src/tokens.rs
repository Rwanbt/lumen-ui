//! Design tokens — the single source of truth for every visual constant.
//!
//! Tokens are raw values (colors, spacings, radii…). A [`crate::Theme`] turns
//! tokens into per-widget *recipes*; widgets never read tokens directly.

use egui::{Color32, CornerRadius, Shadow, Vec2};

/// The complete set of design tokens for a theme.
#[derive(Clone, Debug)]
pub struct Tokens {
    pub colors: Colors,
    pub spacing: Spacing,
    pub radius: Radius,
    pub typography: Typography,
    pub elevation: Elevation,
    pub motion: Motion,
}

/// Semantic color roles (not raw palette swatches).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Colors {
    pub background: Color32,
    pub surface: Color32,
    pub surface_variant: Color32,
    pub primary: Color32,
    pub on_primary: Color32,
    pub secondary: Color32,
    pub on_secondary: Color32,
    pub success: Color32,
    pub on_success: Color32,
    pub warning: Color32,
    pub on_warning: Color32,
    pub danger: Color32,
    pub on_danger: Color32,
    pub text: Color32,
    pub text_muted: Color32,
    pub border: Color32,
}

/// Spacing scale, in logical points.
#[derive(Clone, Copy, Debug)]
pub struct Spacing {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

/// Corner radii, expressed directly as egui [`CornerRadius`].
#[derive(Clone, Copy, Debug)]
pub struct Radius {
    pub sm: CornerRadius,
    pub md: CornerRadius,
    pub lg: CornerRadius,
    pub full: CornerRadius,
}

/// Typographic scale, in logical points.
#[derive(Clone, Copy, Debug)]
pub struct Typography {
    pub body: f32,
    pub label: f32,
    pub heading: f32,
    pub display: f32,
}

/// Elevation tokens, expressed directly as egui [`Shadow`].
#[derive(Clone, Copy, Debug)]
pub struct Elevation {
    pub none: Shadow,
    pub low: Shadow,
    pub high: Shadow,
}

/// Motion durations, in seconds — consumed by `crate::anim`.
#[derive(Clone, Copy, Debug)]
pub struct Motion {
    pub fast: f32,
    pub base: f32,
    pub slow: f32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
        }
    }
}

impl Default for Radius {
    fn default() -> Self {
        Self {
            sm: CornerRadius::same(4),
            md: CornerRadius::same(8),
            lg: CornerRadius::same(12),
            full: CornerRadius::same(255),
        }
    }
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            body: 14.0,
            label: 12.0,
            heading: 18.0,
            display: 28.0,
        }
    }
}

impl Elevation {
    /// A soft, downward shadow used for the `low` elevation default.
    fn soft(blur: u8, alpha: u8) -> Shadow {
        Shadow {
            offset: [0, 2],
            blur,
            spread: 0,
            color: Color32::from_black_alpha(alpha),
        }
    }
}

impl Default for Elevation {
    fn default() -> Self {
        Self {
            none: Shadow::NONE,
            low: Self::soft(8, 60),
            high: Shadow {
                offset: [0, 6],
                blur: 18,
                spread: 0,
                color: Color32::from_black_alpha(90),
            },
        }
    }
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            fast: 0.08,
            base: 0.15,
            slow: 0.30,
        }
    }
}

impl Spacing {
    /// Convenience accessor for a symmetric inner margin from a single scale step.
    #[must_use]
    pub fn pad(x: f32, y: f32) -> Vec2 {
        Vec2::new(x, y)
    }
}
