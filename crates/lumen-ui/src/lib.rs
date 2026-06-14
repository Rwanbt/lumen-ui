//! # lumen-ui
//!
//! A token-driven, themeable design system for [egui](https://github.com/emilk/egui).
//!
//! This is the **façade** crate: it re-exports the internal crates behind feature
//! flags and exposes a [`prelude`]. Depend on `lumen-ui` (not the internal crates)
//! and pick the features you need — see `ROADMAP.md` for the feature matrix.
//!
//! ```no_run
//! use eframe::egui;
//! use lumen_ui::prelude::*;
//! use std::sync::Arc;
//!
//! fn setup(ctx: &egui::Context) {
//!     install(ctx, Arc::new(DarkTheme::new()), UiContext::default());
//! }
//!
//! fn ui(ui: &mut egui::Ui) {
//!     if ui.add(Button::primary("Save")).clicked() {
//!         // ...
//!     }
//! }
//! ```

#![forbid(unsafe_code)]

#[cfg(feature = "tokens")]
pub use lumen_core as core;

#[cfg(feature = "theme")]
#[doc(inline)]
pub use lumen_core::{
    anim, install, set_theme, BadgeRecipe, BadgeVariant, ButtonRecipe, ButtonVariant, CardRecipe,
    Colors, DarkTheme, Density, Elevation, LightTheme, Motion, PaletteTheme, Radius, SliderRecipe,
    Spacing, TextFieldRecipe, TextRecipe, TextRole, Theme, ThemeMode, ToggleRecipe, Tokens,
    Typography, UiContext, UiThemeExt, WidgetState,
};

#[cfg(feature = "themes")]
#[doc(inline)]
pub use lumen_themes::{audio_dark, high_contrast};

#[cfg(feature = "icons")]
#[doc(inline)]
pub use lumen_icons::{Icon, IconKind};

#[cfg(feature = "widgets")]
#[doc(inline)]
pub use lumen_widgets::{
    close_modal, context_menu, open_modal, popover, show_toasts, toast, toast_error, toast_success,
    toast_warning, tooltip, Accordion, Badge, Button, Card, Checkbox, Heading, Label, Modal,
    RadioGroup, Select, Slider, Switch, Tabs, TextField, ToastVariant,
};

#[cfg(feature = "layout")]
#[doc(inline)]
pub use lumen_layout::{responsive, Align, Breakpoint, Flex, FlexUiExt, Grid, Justify};

#[cfg(feature = "motion")]
#[doc(inline)]
pub use lumen_motion::{ease, fade, Easing, Spring};

#[cfg(feature = "patterns")]
#[doc(inline)]
pub use lumen_patterns::{
    open_command_palette, property_row, CommandPalette, DashboardLayout, InspectorPanel, LogEntry,
    LogLevel, LogPanel, SettingsPage, Sidebar, StatusBar, Toolbar,
};

/// Glob-importable essentials.
pub mod prelude {
    #[cfg(feature = "theme")]
    pub use lumen_core::{
        install, set_theme, BadgeVariant, ButtonVariant, DarkTheme, Density, LightTheme,
        PaletteTheme, TextRole, Theme, ThemeMode, UiContext, UiThemeExt, WidgetState,
    };
    #[cfg(feature = "icons")]
    pub use lumen_icons::{Icon, IconKind};
    #[cfg(feature = "layout")]
    pub use lumen_layout::{responsive, Align, Breakpoint, Flex, FlexUiExt, Grid, Justify};
    #[cfg(feature = "motion")]
    pub use lumen_motion::{ease, fade, Easing, Spring};
    #[cfg(feature = "patterns")]
    pub use lumen_patterns::{
        open_command_palette, property_row, CommandPalette, DashboardLayout, InspectorPanel,
        LogEntry, LogLevel, LogPanel, SettingsPage, Sidebar, StatusBar, Toolbar,
    };
    #[cfg(feature = "themes")]
    pub use lumen_themes::{audio_dark, high_contrast};
    #[cfg(feature = "widgets")]
    pub use lumen_widgets::{
        close_modal, context_menu, open_modal, popover, show_toasts, toast, toast_error,
        toast_success, toast_warning, tooltip, Accordion, Badge, Button, Card, Checkbox, Heading,
        Label, Modal, RadioGroup, Select, Slider, Switch, Tabs, TextField, ToastVariant,
    };
}
