//! `lumen-ui-patterns` — ready-made application patterns for lumen-ui.
//!
//! These compose `lumen-ui-widgets` (and egui panels) into common app structures so
//! that wiring a new app is a handful of lines. Enable via the façade `patterns`
//! feature. Requires a theme installed via `lumen_ui::install(..)`.

#![forbid(unsafe_code)]

mod bars;
mod command_palette;
mod dashboard;
mod logpanel;
mod rows;
mod sidebar;

pub use bars::{StatusBar, Toolbar};
pub use command_palette::{open_command_palette, CommandPalette};
pub use dashboard::DashboardLayout;
pub use logpanel::{LogEntry, LogLevel, LogPanel};
pub use rows::{property_row, InspectorPanel, SettingsPage};
pub use sidebar::Sidebar;
