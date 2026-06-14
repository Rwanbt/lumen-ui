//! [`LogPanel`] — a scrollable, leveled log view.

use egui::{ScrollArea, Ui};
use lumen_widgets::{Badge, Label};

/// Severity of a log line — drives the leading badge color.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// One log line.
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
}

impl LogEntry {
    fn with(level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            level,
            message: message.into(),
        }
    }

    pub fn debug(message: impl Into<String>) -> Self {
        Self::with(LogLevel::Debug, message)
    }
    pub fn info(message: impl Into<String>) -> Self {
        Self::with(LogLevel::Info, message)
    }
    pub fn warn(message: impl Into<String>) -> Self {
        Self::with(LogLevel::Warn, message)
    }
    pub fn error(message: impl Into<String>) -> Self {
        Self::with(LogLevel::Error, message)
    }
}

fn level_badge(level: LogLevel) -> Badge {
    match level {
        LogLevel::Debug => Badge::new("DEBUG"),
        LogLevel::Info => Badge::primary("INFO"),
        LogLevel::Warn => Badge::warning("WARN"),
        LogLevel::Error => Badge::danger("ERROR"),
    }
}

/// A scrollable log view with a leading severity badge per line.
#[derive(Clone, Copy, Debug, Default)]
pub struct LogPanel;

impl LogPanel {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Render the entries (newest stays in view via stick-to-bottom).
    pub fn show(self, ui: &mut Ui, entries: &[LogEntry]) {
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for entry in entries {
                    ui.horizontal(|ui| {
                        ui.add(level_badge(entry.level));
                        ui.add(Label::new(entry.message.clone()));
                    });
                }
            });
    }
}
