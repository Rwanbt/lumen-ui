//! `lumen-ui-layout` — CSS-style flexbox layout and responsive breakpoints for
//! lumen-ui, built on [`egui_taffy`](https://crates.io/crates/egui_taffy).
//!
//! Enable via the `layout` feature of the `lumen-ui` façade. Requires
//! `lumen_ui::install(..)` to have run (it sets `max_passes = 2`, which taffy
//! needs to resolve sizes before painting).
//!
//! ```ignore
//! use eframe::egui;
//! use lumen_ui_layout::{Flex, FlexUiExt};
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

use egui::{vec2, CursorIcon, Id, Response, Sense, Ui};
use egui_taffy::taffy::prelude::{fr, length};
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

/// A grid container with `columns` equal-fraction columns. Cells are added with
/// [`FlexUiExt::item`] (each item is one cell, filled in row-major order).
#[derive(Clone, Copy, Debug)]
pub struct Grid {
    columns: usize,
    gap: f32,
    fill_width: bool,
}

impl Grid {
    /// A grid with `columns` equal columns (minimum 1).
    #[must_use]
    pub fn new(columns: usize) -> Self {
        Self {
            columns: columns.max(1),
            gap: 0.0,
            fill_width: false,
        }
    }

    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    #[must_use]
    pub fn fill_width(mut self) -> Self {
        self.fill_width = true;
        self
    }

    fn style(self) -> taffy::Style {
        taffy::Style {
            display: Display::Grid,
            grid_template_columns: vec![fr(1.0); self.columns],
            gap: length(self.gap),
            align_items: Some(taffy::AlignItems::Stretch),
            justify_items: Some(taffy::AlignItems::Stretch),
            ..Default::default()
        }
    }

    /// Lay out the cells. `id_source` must be stable and unique in the parent `Ui`.
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

/// Adds flex/grid items inside a [`Flex::show`] / [`Grid::show`] closure.
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

/// Compute the inner width and the centering left margin for a [`Container`].
fn container_layout(available: f32, max_width: f32) -> (f32, f32) {
    let width = available.min(max_width);
    let margin = ((available - width) / 2.0).max(0.0);
    (width, margin)
}

/// A horizontally-centered content column capped at a maximum width (CSS `max-width` + auto
/// margins). Below the cap it fills the available width; above it, the content is centered.
/// Build with [`Container::new`], then [`Container::show`].
#[derive(Clone, Copy, Debug)]
pub struct Container {
    max_width: f32,
}

impl Container {
    #[must_use]
    pub fn new(max_width: f32) -> Self {
        Self { max_width }
    }

    /// Lay out `content` centered within the maximum width. Returns the closure's value.
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> R {
        let (width, margin) = container_layout(ui.available_width(), self.max_width);
        ui.horizontal(|ui| {
            if margin > 0.0 {
                ui.add_space(margin);
            }
            ui.vertical(|ui| {
                ui.set_max_width(width);
                content(ui)
            })
            .inner
        })
        .inner
    }
}

/// Compute the `(width, height)` of an [`AspectRatio`] box from the available width.
fn aspect_box(available_width: f32, ratio: f32) -> (f32, f32) {
    let r = ratio.max(f32::EPSILON);
    (available_width, available_width / r)
}

/// Reserve a box of a fixed `width:height` ratio (width taken from the available width) and run
/// `content` inside it — for media, previews and placeholders. Build with [`AspectRatio::new`]
/// (or [`AspectRatio::widescreen`] / [`AspectRatio::square`]), then [`AspectRatio::show`].
#[derive(Clone, Copy, Debug)]
pub struct AspectRatio {
    ratio: f32,
}

impl AspectRatio {
    /// A box whose width is `ratio` times its height (e.g. `16.0 / 9.0`).
    #[must_use]
    pub fn new(ratio: f32) -> Self {
        Self {
            ratio: ratio.max(f32::EPSILON),
        }
    }

    /// 16:9.
    #[must_use]
    pub fn widescreen() -> Self {
        Self::new(16.0 / 9.0)
    }

    /// 1:1.
    #[must_use]
    pub fn square() -> Self {
        Self::new(1.0)
    }

    /// Lay out `content` inside the ratio-sized box. Returns the closure's value.
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> R {
        let (width, height) = aspect_box(ui.available_width(), self.ratio);
        ui.allocate_ui(egui::vec2(width, height), content).inner
    }
}

/// Thickness of a [`ResizableSplit`] divider, in points.
const SPLIT_DIVIDER: f32 = 6.0;

/// Clamp a split fraction so neither pane shrinks below `min`.
fn clamp_fraction(fraction: f32, min: f32) -> f32 {
    let min = min.clamp(0.0, 0.5);
    if fraction.is_finite() {
        fraction.clamp(min, 1.0 - min)
    } else {
        0.5
    }
}

/// Orientation of a [`ResizableSplit`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SplitAxis {
    Horizontal,
    Vertical,
}

/// Two panes separated by a draggable divider. The first pane's fraction of the main axis
/// (`0..1`) is remembered in egui memory keyed by the id, so the split position persists across
/// frames. Build with [`ResizableSplit::horizontal`] / [`ResizableSplit::vertical`], then
/// [`ResizableSplit::show`] with the two pane closures.
///
/// The split fills the available size, so constrain the height (horizontal) or width (vertical)
/// of the parent `Ui` if it is otherwise unbounded.
#[derive(Clone, Copy, Debug)]
pub struct ResizableSplit {
    id: Id,
    axis: SplitAxis,
    default_fraction: f32,
    min_fraction: f32,
}

impl ResizableSplit {
    fn new(id_source: impl Hash, axis: SplitAxis) -> Self {
        Self {
            id: Id::new(id_source),
            axis,
            default_fraction: 0.5,
            min_fraction: 0.1,
        }
    }

    /// A left/right split.
    #[must_use]
    pub fn horizontal(id_source: impl Hash) -> Self {
        Self::new(id_source, SplitAxis::Horizontal)
    }

    /// A top/bottom split.
    #[must_use]
    pub fn vertical(id_source: impl Hash) -> Self {
        Self::new(id_source, SplitAxis::Vertical)
    }

    /// Initial fraction of the main axis given to the first pane (default `0.5`).
    #[must_use]
    pub fn default_fraction(mut self, fraction: f32) -> Self {
        self.default_fraction = fraction;
        self
    }

    /// Smallest fraction either pane may shrink to (default `0.1`).
    #[must_use]
    pub fn min_fraction(mut self, min: f32) -> Self {
        self.min_fraction = min;
        self
    }

    /// Lay out the two panes with a draggable divider. Returns the enclosing layout's response.
    pub fn show(
        self,
        ui: &mut Ui,
        first: impl FnOnce(&mut Ui),
        second: impl FnOnce(&mut Ui),
    ) -> Response {
        let horizontal = self.axis == SplitAxis::Horizontal;
        let fraction_id = self.id.with("fraction");
        let mut fraction = ui
            .ctx()
            .data(|d| d.get_temp::<f32>(fraction_id))
            .unwrap_or(self.default_fraction);
        fraction = clamp_fraction(fraction, self.min_fraction);

        let avail = ui.available_size();
        let (main, cross) = if horizontal {
            (avail.x, avail.y)
        } else {
            (avail.y, avail.x)
        };
        let content_main = (main - SPLIT_DIVIDER).max(0.0);
        let first_main = content_main * fraction;
        let second_main = content_main - first_main;
        let cursor = if horizontal {
            CursorIcon::ResizeHorizontal
        } else {
            CursorIcon::ResizeVertical
        };

        let pane = |main_extent: f32| {
            if horizontal {
                vec2(main_extent, cross)
            } else {
                vec2(cross, main_extent)
            }
        };

        let lay = |ui: &mut Ui| {
            ui.allocate_ui(pane(first_main), first);
            let (rect, divider) = ui.allocate_exact_size(pane(SPLIT_DIVIDER), Sense::drag());
            if divider.dragged() {
                let delta = if horizontal {
                    divider.drag_delta().x
                } else {
                    divider.drag_delta().y
                };
                if content_main > 0.0 {
                    fraction = clamp_fraction(fraction + delta / content_main, self.min_fraction);
                }
            }
            let color = if divider.dragged() || divider.hovered() {
                ui.visuals().selection.bg_fill
            } else {
                ui.visuals().widgets.noninteractive.bg_stroke.color
            };
            ui.painter().rect_filled(rect, 0.0, color);
            divider.on_hover_and_drag_cursor(cursor);
            ui.allocate_ui(pane(second_main), second);
        };

        let response = if horizontal {
            ui.horizontal(lay).response
        } else {
            ui.vertical(lay).response
        };

        ui.ctx().data_mut(|d| d.insert_temp(fraction_id, fraction));
        response
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
    use super::{aspect_box, clamp_fraction, container_layout, Breakpoint};

    #[test]
    fn breakpoints_classify_widths() {
        assert_eq!(Breakpoint::from_width(320.0), Breakpoint::Xs);
        assert_eq!(Breakpoint::from_width(700.0), Breakpoint::Sm);
        assert_eq!(Breakpoint::from_width(900.0), Breakpoint::Md);
        assert_eq!(Breakpoint::from_width(1200.0), Breakpoint::Lg);
        assert_eq!(Breakpoint::from_width(1600.0), Breakpoint::Xl);
        assert!(Breakpoint::Xs < Breakpoint::Xl);
    }

    #[test]
    fn container_fills_below_cap_and_centers_above() {
        // Below the cap: full width, no margin.
        assert_eq!(container_layout(500.0, 960.0), (500.0, 0.0));
        // Above the cap: clamped width, centered (equal margins).
        assert_eq!(container_layout(1200.0, 960.0), (960.0, 120.0));
    }

    #[test]
    fn aspect_box_derives_height_from_ratio() {
        assert_eq!(aspect_box(1600.0, 16.0 / 9.0), (1600.0, 900.0));
        assert_eq!(aspect_box(300.0, 1.0), (300.0, 300.0));
        // A non-positive ratio is clamped to avoid div-by-zero (finite height).
        assert!(aspect_box(100.0, 0.0).1.is_finite());
    }

    #[test]
    fn split_fraction_keeps_both_panes_above_min() {
        assert_eq!(clamp_fraction(0.5, 0.1), 0.5);
        assert_eq!(clamp_fraction(0.02, 0.1), 0.1); // too small -> min
        assert_eq!(clamp_fraction(0.98, 0.1), 0.9); // too large -> 1 - min
        assert_eq!(clamp_fraction(f32::NAN, 0.1), 0.5); // non-finite -> centered
    }
}
