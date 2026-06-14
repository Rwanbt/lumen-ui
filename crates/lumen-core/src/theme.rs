//! The [`Theme`] trait and the [`UiThemeExt`] accessor injected into `egui::Ui`.
//!
//! A theme owns the tokens and maps `(variant, state, ctx)` to recipes. It is
//! installed once into `egui::Context` data and read back lock-free by widgets.

use std::sync::Arc;

use crate::context::{Density, UiContext};
use crate::recipe::{
    BadgeRecipe, BadgeVariant, ButtonRecipe, ButtonVariant, CardRecipe, TextRecipe, TextRole,
    ToggleRecipe, WidgetState,
};
use crate::tokens::Tokens;

const THEME_KEY: &str = "lumen_theme";
const CONTEXT_KEY: &str = "lumen_ctx";

/// A complete visual theme: tokens + per-widget recipe resolution + egui mapping.
///
/// `Send + Sync` so it can live behind an `Arc` in the persisted egui data store
/// and be read from any pass.
pub trait Theme: Send + Sync {
    /// Raw design tokens backing this theme.
    fn tokens(&self) -> &Tokens;

    /// Resolve the button recipe for a given variant/state under the current context.
    fn button_recipe(
        &self,
        variant: ButtonVariant,
        state: WidgetState,
        ctx: &UiContext,
    ) -> ButtonRecipe;

    /// Resolve the text recipe (color + size) for a semantic role.
    fn text_recipe(&self, role: TextRole, ctx: &UiContext) -> TextRecipe;

    /// Resolve the card container recipe.
    fn card_recipe(&self, ctx: &UiContext) -> CardRecipe;

    /// Resolve the badge recipe for a semantic variant.
    fn badge_recipe(&self, variant: BadgeVariant, ctx: &UiContext) -> BadgeRecipe;

    /// Resolve the toggle recipe (switch, checkbox) for a given on/off + interaction state.
    fn toggle_recipe(&self, on: bool, state: WidgetState, ctx: &UiContext) -> ToggleRecipe;

    /// Map the tokens onto egui's global `Style`/`Visuals`/`Spacing` so that even
    /// stock egui widgets pick up the theme.
    fn apply_to_ctx(&self, ctx: &egui::Context);
}

/// Read-only access to the installed theme and UI context from any `egui::Ui`.
pub trait UiThemeExt {
    /// The currently installed theme.
    ///
    /// # Panics
    /// Panics if [`install`] was never called on this context.
    fn theme(&self) -> Arc<dyn Theme>;
    /// The currently installed UI context (defaults to comfortable density).
    fn ui_ctx(&self) -> UiContext;
}

impl UiThemeExt for egui::Ui {
    fn theme(&self) -> Arc<dyn Theme> {
        self.ctx()
            .data_mut(|d| d.get_persisted::<Arc<dyn Theme>>(egui::Id::new(THEME_KEY)))
            .expect("lumen-ui not initialised: call lumen_ui::install(ctx, theme, ui_ctx)")
    }

    fn ui_ctx(&self) -> UiContext {
        self.ctx()
            .data_mut(|d| d.get_persisted::<UiContext>(egui::Id::new(CONTEXT_KEY)))
            .unwrap_or(UiContext {
                density: Density::Comfortable,
            })
    }
}

/// Install a theme and UI context into an `egui::Context`. Call once at startup,
/// or again to swap themes live — the whole app restyles without touching logic.
pub fn install(ctx: &egui::Context, theme: Arc<dyn Theme>, ui_ctx: UiContext) {
    // max_passes = 2: lets widgets read the previous frame's response for hover/active
    // before the final paint pass (cf. ROADMAP.md §Corrections d'API).
    ctx.options_mut(|o| o.max_passes = std::num::NonZeroUsize::new(2).unwrap());
    theme.apply_to_ctx(ctx);
    ctx.data_mut(|d| {
        d.insert_persisted(egui::Id::new(THEME_KEY), theme);
        d.insert_persisted(egui::Id::new(CONTEXT_KEY), ui_ctx);
    });
}

/// Swap the theme while preserving the existing UI context.
pub fn set_theme(ctx: &egui::Context, theme: Arc<dyn Theme>) {
    let ui_ctx = ctx
        .data_mut(|d| d.get_persisted::<UiContext>(egui::Id::new(CONTEXT_KEY)))
        .unwrap_or(UiContext {
            density: Density::Comfortable,
        });
    install(ctx, theme, ui_ctx);
}
