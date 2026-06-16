//! `lumen-ui-widgets` — themed egui widgets that consume `lumen-ui-core` recipes.
//!
//! Every widget reads the installed [`lumen_ui_core::Theme`] for its recipe; none
//! hard-codes a color or padding. Minimal motion is integrated from v0.2 via
//! `lumen_ui_core::anim`; it switches to the `lumen-ui-motion` spring solver in v0.5
//! with no public API change.

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

mod accordion;
mod alert;
mod avatar;
mod badge;
mod button;
mod card;
mod checkbox;
mod chip;
mod divider;
mod focus;
mod kbd;
mod modal;
mod overlay;
mod progress;
mod radio;
mod select;
mod skeleton;
mod slider;
mod spinner;
mod switch;
mod tabs;
mod text;
mod text_field;
mod toast;

pub use accordion::Accordion;
pub use alert::Alert;
pub use avatar::Avatar;
pub use badge::Badge;
pub use button::Button;
pub use card::Card;
pub use checkbox::Checkbox;
pub use chip::{Chip, ChipResponse};
pub use divider::Divider;
pub use kbd::Kbd;
pub use modal::{close_modal, open_modal, Modal};
pub use overlay::{context_menu, popover, tooltip};
pub use progress::Progress;
pub use radio::RadioGroup;
pub use select::Select;
pub use skeleton::Skeleton;
pub use slider::Slider;
pub use spinner::Spinner;
pub use switch::Switch;
pub use tabs::Tabs;
pub use text::{Heading, Label};
pub use text_field::TextField;
pub use toast::{show_toasts, toast, toast_error, toast_success, toast_warning, ToastVariant};
