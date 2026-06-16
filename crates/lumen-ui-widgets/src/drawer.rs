//! [`Drawer`] — an off-canvas side panel over a scrim, whose open state lives in
//! `egui` memory (no external boolean).
//!
//! Open it with [`open_drawer`], render it with [`Drawer::show`] (returns `None` while closed).
//! It auto-closes on scrim click or Esc. Built on `egui::Modal` (which provides the backdrop,
//! input blocking and Esc handling) with the modal area anchored to a screen edge and the panel
//! drawn full-height. Motion is instant — a slide transition waits for motion v2 (v1.6).

use std::hash::Hash;

use egui::{Align2, Context, Frame, Id, Modal, Ui, Vec2};
use lumen_ui_core::{DrawerRecipe, UiThemeExt};

/// Which screen edge the drawer is anchored to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DrawerSide {
    #[default]
    Left,
    Right,
}

fn open_key(id: Id) -> Id {
    id.with("lumen_drawer_open")
}

/// Open the drawer identified by `id_source` (same value passed to [`Drawer::new`]).
pub fn open_drawer(ctx: &Context, id_source: impl Hash) {
    ctx.data_mut(|d| d.insert_temp(open_key(Id::new(id_source)), true));
}

/// Close the drawer identified by `id_source`.
pub fn close_drawer(ctx: &Context, id_source: impl Hash) {
    ctx.data_mut(|d| d.insert_temp(open_key(Id::new(id_source)), false));
}

/// An off-canvas side panel. Build with [`Drawer::new`] (+ optional [`Drawer::side`]), then call
/// [`Drawer::show`] every frame; it draws only while open and returns the closure's value then.
///
/// ```ignore
/// if ui.add(Button::primary("Menu")).clicked() { open_drawer(ui.ctx(), "nav"); }
/// Drawer::new("nav").show(ui.ctx(), |ui| { ui.label("Navigation"); });
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Drawer {
    id: Id,
    side: DrawerSide,
}

impl Drawer {
    #[must_use]
    pub fn new(id_source: impl Hash) -> Self {
        Self {
            id: Id::new(id_source),
            side: DrawerSide::Left,
        }
    }

    /// Anchor the drawer to the given screen edge (default: [`DrawerSide::Left`]).
    #[must_use]
    pub fn side(mut self, side: DrawerSide) -> Self {
        self.side = side;
        self
    }

    /// Draw the drawer if open. Returns the closure's value while shown, else `None`.
    /// Auto-closes on scrim click or Esc.
    pub fn show<R>(self, ctx: &Context, add_contents: impl FnOnce(&mut Ui) -> R) -> Option<R> {
        let open = ctx.data_mut(|d| d.get_temp::<bool>(open_key(self.id)).unwrap_or(false));
        if !open {
            return None;
        }

        let align = match self.side {
            DrawerSide::Left => Align2::LEFT_CENTER,
            DrawerSide::Right => Align2::RIGHT_CENTER,
        };
        // Reuse Modal's machinery (backdrop scrim, input blocking, Esc) but anchor its area to the
        // edge instead of center; the panel fill/width come from the recipe inside the closure.
        let area = Modal::default_area(self.id).anchor(align, Vec2::ZERO);

        let response = Modal::new(self.id)
            .area(area)
            .frame(Frame::NONE)
            .show(ctx, |ui| {
                let recipe = DrawerRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
                let screen_height = ui.ctx().content_rect().height();
                Frame::NONE
                    .fill(recipe.fill)
                    .inner_margin(crate::util::margin(recipe.inner_margin))
                    .show(ui, |ui| {
                        ui.set_width(recipe.width);
                        ui.set_min_height(screen_height);
                        add_contents(ui)
                    })
                    .inner
            });

        if response.should_close() {
            ctx.data_mut(|d| d.insert_temp(open_key(self.id), false));
        }
        Some(response.inner)
    }
}
