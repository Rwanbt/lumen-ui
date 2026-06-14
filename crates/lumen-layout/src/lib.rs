//! `lumen-layout` — CSS-style flexbox layout and responsive breakpoints for
//! lumen-ui, built on [`egui_taffy`](https://crates.io/crates/egui_taffy).
//!
//! Enable via the `layout` feature of the `lumen-ui` façade. Requires
//! `lumen_ui::install(..)` to have run (it sets `max_passes = 2`, which taffy
//! needs to resolve sizes before painting).
//!
//! ```no_run
//! use eframe::egui;
//! use lumen_layout::{Flex, FlexUiExt};
//!
//! fn ui(ui: &mut egui::Ui) {
//!     Flex::row().gap(8.0).show(ui, "toolbar", |t| {
//!         t.item(|ui| { ui.label("left"); });
//!         t.item_grow(|ui| { ui.label("stretches"); });
//!         t.item(|ui| { ui.label("right"); });
//!     });
//! }
//! ```

#![forbid(unsafe_code)]

use std::hash::Hash;

use egui::{Id, Ui};
use egui_taffy::taffy::prelude::length;
use egui_taffy::taffy::{self, Display, FlexDirection};
use egui_taffy::{tui, Tui, TuiBuilderLogic};

/// Main-axis distribution of flex items.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Justify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Cross-axis alignment of flex items.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Align {
    Start,
    Center,
    End,
    Stretch,
}

fn map_justify(j: Justify) -> taffy::JustifyContent {
    match j {
        Justify::Start => taffy::JustifyContent::Start,
        Justify::Center => taffy::JustifyContent::Center,
        Justify::End => taffy::JustifyContent::End,
        Justify::SpaceBetween => taffy::JustifyContent::SpaceBetween,
        Justify::SpaceAround => taffy::JustifyContent::SpaceAround,
        Justify::SpaceEvenly => taffy::JustifyContent::SpaceEvenly,
    }
}

fn map_align(a: Align) -> taffy::AlignItems {
    match a {
        Align::Start => taffy::AlignItems::Start,
        Align::Center => taffy::AlignItems::Center,
        Align::End => taffy::AlignItems::End,
        Align::Stretch => taffy::AlignItems::Stretch,
    }
}

/// A flexbox container. Build with [`Flex::row`] / [`Flex::column`], configure,
/// then [`Flex::show`] with a closure that adds items via [`FlexUiExt`].
#[derive(Clone, Copy, Debug)]
pub struct Flex {
    direction: FlexDirection,
    gap: f32,
    justify: Option<Justify>,
    align: Option<Align>,
    fill_width: bool,
}

impl Flex {
    fn new(direction: FlexDirection) -> Self {
        Self {
            direction,
            gap: 0.0,
            justify: None,
            align: None,
            fill_width: false,
        }
    }

    #[must_use]
    pub fn row() -> Self {
        Self::new(FlexDirection::Row)
    }

    #[must_use]
    pub fn column() -> Self {
        Self::new(FlexDirection::Column)
    }

    /// Space between items, in points.
    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    #[must_use]
    pub fn justify(mut self, justify: Justify) -> Self {
        self.justify = Some(justify);
        self
    }

    #[must_use]
    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    /// Reserve the full available width for this container (useful for top-level rows).
    #[must_use]
    pub fn fill_width(mut self) -> Self {
        self.fill_width = true;
        self
    }

    fn style(self) -> taffy::Style {
        taffy::Style {
            display: Display::Flex,
            flex_direction: self.direction,
            gap: length(self.gap),
            justify_content: self.justify.map(map_justify),
            align_items: self.align.map(map_align),
            ..Default::default()
        }
    }

    /// Lay out the children. `id_source` must be stable and unique in the parent `Ui`.
    pub fn show(self, ui: &mut Ui, id_source: impl Hash, content: impl FnOnce(&mut Tui)) {
        let style = self.style();
        let init = tui(ui, Id::new(id_source));
        let init = if self.fill_width {
            init.reserve_available_width()
        } else {
            init
        };
        init.style(style).show(content);
    }
}

/// Adds flex items inside a [`Flex::show`] closure.
pub trait FlexUiExt {
    /// Add an item sized to its content.
    fn item(&mut self, content: impl FnOnce(&mut Ui));
    /// Add an item that grows to fill remaining space (`flex-grow: 1`).
    fn item_grow(&mut self, content: impl FnOnce(&mut Ui));
    /// Add a nested flex container.
    fn nest(&mut self, flex: Flex, content: impl FnOnce(&mut Tui));
}

impl FlexUiExt for Tui {
    fn item(&mut self, content: impl FnOnce(&mut Ui)) {
        (&mut *self).ui(content);
    }

    fn item_grow(&mut self, content: impl FnOnce(&mut Ui)) {
        let grow = taffy::Style {
            flex_grow: 1.0,
            ..Default::default()
        };
        (&mut *self).style(grow).ui(content);
    }

    fn nest(&mut self, flex: Flex, content: impl FnOnce(&mut Tui)) {
        (&mut *self).style(flex.style()).add(content);
    }
}

/// Responsive breakpoints (CSS-ish). Resolved from available width.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Breakpoint {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl Breakpoint {
    /// Classify a width (in points) into a breakpoint.
    #[must_use]
    pub fn from_width(width: f32) -> Self {
        match width {
            w if w < 640.0 => Breakpoint::Xs,
            w if w < 768.0 => Breakpoint::Sm,
            w if w < 1024.0 => Breakpoint::Md,
            w if w < 1280.0 => Breakpoint::Lg,
            _ => Breakpoint::Xl,
        }
    }
}

/// Run `f` with the breakpoint for the current available width.
pub fn responsive<R>(ui: &Ui, f: impl FnOnce(Breakpoint) -> R) -> R {
    f(Breakpoint::from_width(ui.available_width()))
}

#[cfg(test)]
mod tests {
    use super::Breakpoint;

    #[test]
    fn breakpoints_classify_widths() {
        assert_eq!(Breakpoint::from_width(320.0), Breakpoint::Xs);
        assert_eq!(Breakpoint::from_width(700.0), Breakpoint::Sm);
        assert_eq!(Breakpoint::from_width(900.0), Breakpoint::Md);
        assert_eq!(Breakpoint::from_width(1200.0), Breakpoint::Lg);
        assert_eq!(Breakpoint::from_width(1600.0), Breakpoint::Xl);
        assert!(Breakpoint::Xs < Breakpoint::Xl);
    }
}
