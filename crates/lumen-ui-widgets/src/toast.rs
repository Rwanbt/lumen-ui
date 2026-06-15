//! Toasts — transient notifications with a queue and auto-dismiss.
//!
//! No external state: push with [`toast`] (or a variant helper) from anywhere
//! with a `&Context`, then call [`show_toasts`] once per frame to render and
//! expire them. The queue lives in `ctx.data`; expiry uses egui's frame time.

use egui::{Align2, Area, Color32, Context, Frame, Id, Margin, Order, Stroke, Vec2};
use lumen_ui_core::{Theme, UiThemeExt};

use crate::text::Label;

const QUEUE_KEY: &str = "lumen_toasts";
const DEFAULT_DURATION: f64 = 4.0;

/// Severity of a toast — drives its accent color.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ToastVariant {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone)]
struct ToastItem {
    text: String,
    variant: ToastVariant,
    created: f64,
    duration: f64,
}

#[derive(Clone, Default)]
struct ToastQueue {
    items: Vec<ToastItem>,
}

fn queue_id() -> Id {
    Id::new(QUEUE_KEY)
}

/// Push a neutral/info toast.
pub fn toast(ctx: &Context, text: impl Into<String>) {
    push(ctx, text, ToastVariant::Info);
}

/// Push a success toast.
pub fn toast_success(ctx: &Context, text: impl Into<String>) {
    push(ctx, text, ToastVariant::Success);
}

/// Push a warning toast.
pub fn toast_warning(ctx: &Context, text: impl Into<String>) {
    push(ctx, text, ToastVariant::Warning);
}

/// Push an error toast.
pub fn toast_error(ctx: &Context, text: impl Into<String>) {
    push(ctx, text, ToastVariant::Error);
}

fn push(ctx: &Context, text: impl Into<String>, variant: ToastVariant) {
    let now = ctx.input(|i| i.time);
    let item = ToastItem {
        text: text.into(),
        variant,
        created: now,
        duration: DEFAULT_DURATION,
    };
    ctx.data_mut(|d| {
        d.get_temp_mut_or_default::<ToastQueue>(queue_id())
            .items
            .push(item)
    });
    ctx.request_repaint();
}

fn accent(theme: &dyn Theme, variant: ToastVariant) -> Color32 {
    let c = &theme.tokens().colors;
    match variant {
        ToastVariant::Info => c.primary,
        ToastVariant::Success => c.success,
        ToastVariant::Warning => c.warning,
        ToastVariant::Error => c.danger,
    }
}

/// Render and expire queued toasts. Call once per frame (e.g. at the end of your
/// update). Stacks bottom-right.
pub fn show_toasts(ctx: &Context) {
    let now = ctx.input(|i| i.time);
    let mut queue: ToastQueue = ctx.data_mut(|d| d.get_temp(queue_id()).unwrap_or_default());
    queue.items.retain(|t| now - t.created < t.duration);
    let still_active = !queue.items.is_empty();

    Area::new(queue_id())
        .order(Order::Foreground)
        .anchor(Align2::RIGHT_BOTTOM, Vec2::new(-12.0, -12.0))
        .show(ctx, |ui| {
            let theme = ui.theme();
            let card = theme.card_recipe(&ui.ui_ctx());
            for item in &queue.items {
                Frame::NONE
                    .fill(card.fill)
                    .stroke(Stroke::new(1.5, accent(theme.as_ref(), item.variant)))
                    .corner_radius(card.corner_radius)
                    .shadow(card.shadow)
                    .inner_margin(Margin::symmetric(12, 10))
                    .show(ui, |ui| {
                        ui.add(Label::new(item.text.clone()));
                    });
                ui.add_space(6.0);
            }
        });

    ctx.data_mut(|d| d.insert_temp(queue_id(), queue));
    if still_active {
        ctx.request_repaint(); // keep ticking so toasts expire on time
    }
}
