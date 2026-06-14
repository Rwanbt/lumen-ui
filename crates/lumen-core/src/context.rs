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
}
