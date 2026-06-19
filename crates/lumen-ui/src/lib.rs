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
pub use lumen_ui_core as core;

#[cfg(feature = "theme")]
#[doc(inline)]
pub use lumen_ui_core::{
    a11y, anim, install, set_theme, AlertRecipe, AlertVariant, AvatarRecipe, BadgeRecipe,
    BadgeVariant, BreadcrumbRecipe, ButtonRecipe, ButtonVariant, CardRecipe, ChipRecipe,
    CircularProgressRecipe, CodeRecipe, Colors, ContrastLevel, DarkTheme, Date, Density,
    DividerRecipe, Elevation, EmptyStateRecipe, FormFieldRecipe, IconButtonRecipe, KbdRecipe,
    LightTheme, LinkRecipe, MenuRecipe, Motion, PaginationRecipe, PaletteTheme, ProgressRecipe,
    Radius, RatingRecipe, SegmentedRecipe, SkeletonRecipe, SliderRecipe, Spacing, SpinnerRecipe,
    StatRecipe, StepperRecipe, TableRecipe, TextFieldRecipe, TextRecipe, TextRole, Theme,
    ThemeMode, Time, ToggleRecipe, Tokens, Typography, UiContext, UiThemeExt, WidgetState,
};

#[cfg(feature = "themes")]
#[doc(inline)]
pub use lumen_ui_themes::{
    audio_dark, high_contrast, nord, seno_dawn, seno_night, solarized_dark, system_mode,
    ThemeBuilder,
};

#[cfg(feature = "icons")]
#[doc(inline)]
pub use lumen_ui_icons::{Icon, IconKind};

#[cfg(feature = "audio")]
#[doc(inline)]
pub use lumen_ui_audio::{LevelBar, VuMeter, Waveform};

#[cfg(feature = "widgets")]
#[doc(inline)]
pub use lumen_ui_widgets::{
    close_drawer, close_modal, context_menu, hover_card, open_drawer, open_modal, popover,
    show_toasts, toast, toast_error, toast_success, toast_warning, tooltip, Accordion, Alert,
    Avatar, Badge, Breadcrumb, Button, Calendar, Card, Carousel, Checkbox, Chip, ChipResponse,
    CircularProgress, Code, ColorPicker, Combobox, DatePicker, DescriptionList, Divider, Drawer,
    DrawerSide, DropdownMenu, EmptyState, Fader, FileUpload, FileUploadResponse, FormField,
    Heading, IconButton, Kbd, Knob, Label, Link, Modal, MultiSelect, NumberInput, Pagination,
    Progress, RadioGroup, RangeSlider, Rating, SegmentedControl, Select, Skeleton, Slider, Spinner,
    Stat, Stepper, Switch, Table, Tabs, TextField, Textarea, TimePicker, Timeline, ToastVariant,
    Transport, TransportAction, XyPad,
};

#[cfg(feature = "layout")]
#[doc(inline)]
pub use lumen_ui_layout::{
    responsive, Align, AspectRatio, Breakpoint, Container, Flex, FlexUiExt, Grid, GridTemplate,
    Justify, ResizableSplit, Scroll, Stack, StackUi, Track,
};

#[cfg(feature = "motion")]
#[doc(inline)]
pub use lumen_ui_motion::{ease, fade, reduced_motion, set_reduced_motion, Easing, Spring};

#[cfg(feature = "patterns")]
#[doc(inline)]
pub use lumen_ui_patterns::{
    open_command_palette, property_row, CommandPalette, DashboardLayout, InspectorPanel, LogEntry,
    LogLevel, LogPanel, SettingsPage, Sidebar, StatusBar, Toolbar,
};

/// Glob-importable essentials.
pub mod prelude {
    #[cfg(feature = "audio")]
    pub use lumen_ui_audio::{LevelBar, VuMeter, Waveform};
    #[cfg(feature = "theme")]
    pub use lumen_ui_core::{
        install, set_theme, BadgeVariant, ButtonVariant, DarkTheme, Date, Density, LightTheme,
        PaletteTheme, TextRole, Theme, ThemeMode, Time, UiContext, UiThemeExt, WidgetState,
    };
    #[cfg(feature = "icons")]
    pub use lumen_ui_icons::{Icon, IconKind};
    #[cfg(feature = "layout")]
    pub use lumen_ui_layout::{
        responsive, Align, AspectRatio, Breakpoint, Container, Flex, FlexUiExt, Grid, Justify,
    };
    #[cfg(feature = "motion")]
    pub use lumen_ui_motion::{ease, fade, reduced_motion, set_reduced_motion, Easing, Spring};
    #[cfg(feature = "patterns")]
    pub use lumen_ui_patterns::{
        open_command_palette, property_row, CommandPalette, DashboardLayout, InspectorPanel,
        LogEntry, LogLevel, LogPanel, SettingsPage, Sidebar, StatusBar, Toolbar,
    };
    #[cfg(feature = "themes")]
    pub use lumen_ui_themes::{
        audio_dark, high_contrast, nord, seno_dawn, seno_night, solarized_dark, system_mode,
        ThemeBuilder,
    };
    #[cfg(feature = "widgets")]
    pub use lumen_ui_widgets::{
        close_drawer, close_modal, context_menu, hover_card, open_drawer, open_modal, popover,
        show_toasts, toast, toast_error, toast_success, toast_warning, tooltip, Accordion, Alert,
        Avatar, Badge, Breadcrumb, Button, Calendar, Card, Carousel, Checkbox, Chip, ChipResponse,
        CircularProgress, Code, ColorPicker, Combobox, DatePicker, DescriptionList, Divider,
        Drawer, DrawerSide, DropdownMenu, EmptyState, Fader, FileUpload, FileUploadResponse,
        FormField, Heading, IconButton, Kbd, Knob, Label, Link, Modal, MultiSelect, NumberInput,
        Pagination, Progress, RadioGroup, RangeSlider, Rating, SegmentedControl, Select, Skeleton,
        Slider, Spinner, Stat, Stepper, Switch, Table, Tabs, TextField, Textarea, TimePicker,
        Timeline, ToastVariant, Transport, TransportAction, XyPad,
    };
}
