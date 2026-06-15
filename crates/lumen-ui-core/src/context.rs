//! UI context — ambient, theme-independent display parameters.
//!
//! [`Density`] is wired in from v0.1 even though only `Comfortable` is fully
//! implemented. Reserving it now avoids retro-fitting a second parameter onto
//! every recipe signature later (cf. ROADMAP.md §Changelog).

/// Display density. Recipes receive this so a single theme can adapt its
/// paddings / hit-targets to compact desktop, comfortable default, or touch.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Density {
    Compact,
    #[default]
    Comfortable,
    Touch,
}

/// Ambient UI context passed to every recipe. Extensible (high_contrast, …)
/// without breaking the [`crate::Theme`] trait, since recipes already take it.
#[derive(Clone, Copy, Debug, Default)]
pub struct UiContext {
    pub density: Density,
}

impl UiContext {
    /// Multiplier applied to base paddings/hit-targets for the current density.
    #[must_use]
    pub fn density_scale(&self) -> f32 {
        match self.density {
            Density::Compact => 0.75,
            Density::Comfortable => 1.0,
            Density::Touch => 1.4,
        }
    }

    /// Minimum interactive (hit-target) edge in logical points for this density.
    ///
    /// `Touch` returns **44.0** — the WCAG 2.1 §2.5.5 / Apple HIG / Material
    /// recommended minimum so a finger can reliably hit the control. `Comfortable`
    /// and `Compact` relax to mouse-friendly sizes (still ≥ the WCAG 2.5.8 AA floor
    /// of 24 px). Widgets pass this to egui as a `min_size` so labels never shrink a
    /// control below its target.
    #[must_use]
    pub fn min_interactive_size(&self) -> f32 {
        match self.density {
            Density::Compact => 28.0,
            Density::Comfortable => 36.0,
            Density::Touch => 44.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touch_meets_wcag_44px_target() {
        let touch = UiContext {
            density: Density::Touch,
        };
        assert!(touch.min_interactive_size() >= 44.0);
    }

    #[test]
    fn hit_target_grows_with_density() {
        let size = |d| UiContext { density: d }.min_interactive_size();
        assert!(size(Density::Compact) < size(Density::Comfortable));
        assert!(size(Density::Comfortable) < size(Density::Touch));
        // Every density clears the WCAG 2.5.8 AA floor of 24 px.
        assert!(size(Density::Compact) >= 24.0);
    }
}
