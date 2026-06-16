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
mod breadcrumb;
mod button;
mod card;
mod checkbox;
mod chip;
mod circular_progress;
mod code;
#[cfg(feature = "datagrid")]
mod data_grid;
mod divider;
mod drawer;
mod dropdown_menu;
mod empty_state;
mod focus;
mod form_field;
mod icon_button;
mod kbd;
mod link;
mod modal;
mod overlay;
mod pagination;
mod progress;
mod radio;
mod rating;
mod segmented_control;
mod select;
mod skeleton;
mod slider;
mod spinner;
mod stat;
mod stepper;
mod switch;
mod table;
mod tabs;
mod text;
mod text_field;
mod textarea;
mod toast;
mod tree_view;
mod util;

pub use accordion::Accordion;
pub use alert::Alert;
pub use avatar::Avatar;
pub use badge::Badge;
pub use breadcrumb::Breadcrumb;
pub use button::Button;
pub use card::Card;
pub use checkbox::Checkbox;
pub use chip::{Chip, ChipResponse};
pub use circular_progress::CircularProgress;
pub use code::Code;
#[cfg(feature = "datagrid")]
pub use data_grid::{DataGrid, SortDirection, SortState};
pub use divider::Divider;
pub use drawer::{close_drawer, open_drawer, Drawer, DrawerSide};
pub use dropdown_menu::DropdownMenu;
pub use empty_state::EmptyState;
pub use form_field::FormField;
pub use icon_button::IconButton;
pub use kbd::Kbd;
pub use link::Link;
pub use modal::{close_modal, open_modal, Modal};
pub use overlay::{context_menu, hover_card, popover, tooltip};
pub use pagination::Pagination;
pub use progress::Progress;
pub use radio::RadioGroup;
pub use rating::Rating;
pub use segmented_control::SegmentedControl;
pub use select::Select;
pub use skeleton::Skeleton;
pub use slider::Slider;
pub use spinner::Spinner;
pub use stat::Stat;
pub use stepper::Stepper;
pub use switch::Switch;
pub use table::Table;
pub use tabs::Tabs;
pub use text::{Heading, Label};
pub use text_field::TextField;
pub use textarea::Textarea;
pub use toast::{show_toasts, toast, toast_error, toast_success, toast_warning, ToastVariant};
pub use tree_view::{TreeNode, TreeView};
