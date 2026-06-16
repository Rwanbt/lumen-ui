//! `lumen-ui-core` — the foundation layer of **lumen-ui**.
//!
//! Owns the design tokens, the [`Density`]/[`UiContext`] ambient parameters,
//! the [`Theme`] trait with **state-parameterized recipes**, the minimal motion
//! helpers, and the [`install`] entry point that wires a theme into an
//! `egui::Context`.
//!
//! Widgets (in `lumen-ui-widgets`) never read tokens directly — they read a
//! *recipe* resolved by the theme for `(variant, state, ctx)`. This indirection
//! is what lets a theme swap restyle an entire app without touching app logic.
//!
//! See `ROADMAP.md` at the repository root for the full plan and the egui 0.34
//! API corrections that shape these signatures.

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

pub mod a11y;
pub mod anim;
mod builder;
pub mod context;
pub mod dark;
pub mod light;
pub mod palette;
pub mod recipe;
pub mod theme;
pub mod tokens;

pub use a11y::{
    audit_colors, contrast_ratio, meets, meets_aa, relative_luminance, AuditReport, ContrastCheck,
    ContrastLevel,
};
pub use context::{Density, UiContext};
pub use dark::DarkTheme;
pub use light::LightTheme;
pub use palette::{PaletteTheme, ThemeMode};
pub use recipe::{
    AlertRecipe, AlertVariant, AvatarRecipe, BadgeRecipe, BadgeVariant, BreadcrumbRecipe,
    ButtonRecipe, ButtonVariant, CardRecipe, ChipRecipe, CircularProgressRecipe, CodeRecipe,
    DataGridRecipe, DividerRecipe, DrawerRecipe, EmptyStateRecipe, FormFieldRecipe,
    IconButtonRecipe, KbdRecipe, LinkRecipe, MenuRecipe, PaginationRecipe, ProgressRecipe,
    RatingRecipe, SegmentedRecipe, SkeletonRecipe, SliderRecipe, SpinnerRecipe, StatRecipe,
    StepperRecipe, TableRecipe, TextFieldRecipe, TextRecipe, TextRole, ToggleRecipe,
    TreeViewRecipe, WidgetState,
};
pub use theme::{install, set_theme, Theme, UiThemeExt};
pub use tokens::{Colors, Elevation, Motion, Radius, Spacing, Tokens, Typography};
