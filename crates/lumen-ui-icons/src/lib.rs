//! `lumen-ui-icons` — a small set of crisp vector icons drawn with the egui painter
//! (no font asset). Each [`Icon`] is an egui `Widget`: it sizes to the row height
//! and uses the theme's text color by default.
//!
//! Enable via the façade `icons` feature.
//!
//! ```ignore
//! ui.add(lumen_ui_icons::Icon::search());
//! ui.add(lumen_ui_icons::Icon::close().size(20.0));
//! ```

#![forbid(unsafe_code)]

use egui::{pos2, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};
use lumen_ui_core::UiThemeExt;

/// Which glyph an [`Icon`] draws.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IconKind {
    Check,
    Close,
    ChevronDown,
    ChevronRight,
    Plus,
    Minus,
    Search,
    Menu,
}

/// A themed vector icon. Build with a constructor (e.g. [`Icon::check`]), then
/// `ui.add(icon)`. Defaults: row-height size, theme text color.
#[derive(Clone, Copy, Debug)]
pub struct Icon {
    kind: IconKind,
    size: Option<f32>,
    color: Option<Color32>,
}

impl Icon {
    #[must_use]
    pub fn new(kind: IconKind) -> Self {
        Self {
            kind,
            size: None,
            color: None,
        }
    }

    /// Override the square size (points).
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Override the stroke color (defaults to the theme's text color).
    #[must_use]
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
}

macro_rules! icon_ctor {
    ($name:ident, $kind:ident) => {
        impl Icon {
            #[must_use]
            pub fn $name() -> Self {
                Self::new(IconKind::$kind)
            }
        }
    };
}
icon_ctor!(check, Check);
icon_ctor!(close, Close);
icon_ctor!(chevron_down, ChevronDown);
icon_ctor!(chevron_right, ChevronRight);
icon_ctor!(plus, Plus);
icon_ctor!(minus, Minus);
icon_ctor!(search, Search);
icon_ctor!(menu, Menu);

impl Widget for Icon {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = self.size.unwrap_or_else(|| ui.spacing().interact_size.y);
        let color = self
            .color
            .unwrap_or_else(|| ui.theme().tokens().colors.text);
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
        let stroke = Stroke::new((size * 0.09).max(1.5), color);
        paint(self.kind, ui.painter(), rect, stroke);
        response
    }
}

// Normalized point (0..1) inside `rect`.
fn at(rect: Rect, x: f32, y: f32) -> Pos2 {
    pos2(
        rect.left() + x * rect.width(),
        rect.top() + y * rect.height(),
    )
}

fn paint(kind: IconKind, painter: &egui::Painter, rect: Rect, stroke: Stroke) {
    let seg = |x1, y1, x2, y2| painter.line_segment([at(rect, x1, y1), at(rect, x2, y2)], stroke);
    match kind {
        IconKind::Check => {
            seg(0.20, 0.55, 0.42, 0.75);
            seg(0.42, 0.75, 0.80, 0.28);
        }
        IconKind::Close => {
            seg(0.25, 0.25, 0.75, 0.75);
            seg(0.75, 0.25, 0.25, 0.75);
        }
        IconKind::ChevronDown => {
            seg(0.25, 0.40, 0.50, 0.65);
            seg(0.50, 0.65, 0.75, 0.40);
        }
        IconKind::ChevronRight => {
            seg(0.40, 0.25, 0.65, 0.50);
            seg(0.65, 0.50, 0.40, 0.75);
        }
        IconKind::Plus => {
            seg(0.50, 0.22, 0.50, 0.78);
            seg(0.22, 0.50, 0.78, 0.50);
        }
        IconKind::Minus => {
            seg(0.22, 0.50, 0.78, 0.50);
        }
        IconKind::Search => {
            painter.circle_stroke(at(rect, 0.42, 0.42), 0.22 * rect.width(), stroke);
            seg(0.60, 0.60, 0.82, 0.82);
        }
        IconKind::Menu => {
            seg(0.20, 0.32, 0.80, 0.32);
            seg(0.20, 0.50, 0.80, 0.50);
            seg(0.20, 0.68, 0.80, 0.68);
        }
    }
}
