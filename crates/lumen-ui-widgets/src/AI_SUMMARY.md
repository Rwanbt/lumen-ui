# AI_SUMMARY — src

> **Auto-generated 2026-06-16 11:23** — do not edit manually.
> Source: `tools/ai_docs/generate_ai_summary.py`
> For purpose, thread model and constraints, read `AI_CONTEXT.md`.

## Purpose
Themed egui widgets that consume `lumen-ui-core` recipes. v0.1 ships `Button`; v0.2 adds the
foundational set (Input, Card, Badge, Switch, Checkbox, RadioGroup, Slider, Label, Heading).
Widgets are the only place that calls egui's drawing API besides `lumen-ui-core::theme`.

## Files & LOC
| File | LOC | |
|------|-----|--|
| `accordion.rs` | 31 | |
| `alert.rs` | 57 | |
| `avatar.rs` | 50 | |
| `badge.rs` | 49 | |
| `breadcrumb.rs` | 58 | |
| `button.rs` | 102 | |
| `card.rs` | 25 | |
| `checkbox.rs` | 63 | |
| `chip.rs` | 61 | |
| `circular_progress.rs` | 47 | |
| `code.rs` | 31 | |
| `divider.rs` | 51 | |
| `dropdown_menu.rs` | 45 | |
| `empty_state.rs` | 43 | |
| `focus.rs` | 27 | |
| `form_field.rs` | 62 | |
| `icon_button.rs` | 43 | |
| `kbd.rs` | 31 | |
| `lib.rs` | 86 | |
| `link.rs` | 23 | |
| `modal.rs` | 57 | |
| `overlay.rs` | 37 | |
| `pagination.rs` | 63 | |
| `progress.rs` | 33 | |
| `radio.rs` | 81 | |
| `rating.rs` | 50 | |
| `segmented_control.rs` | 66 | |
| `select.rs` | 41 | |
| `skeleton.rs` | 24 | |
| `slider.rs` | 98 | |
| `spinner.rs` | 25 | |
| `stat.rs` | 50 | |
| `stepper.rs` | 78 | |
| `switch.rs` | 58 | |
| `table.rs` | 80 | |
| `tabs.rs` | 46 | |
| `text.rs` | 63 | |
| `text_field.rs` | 58 | |
| `textarea.rs` | 61 | |
| `toast.rs` | 100 | |
| `tree_view.rs` | 114 | |
| `util.rs` | 37 | |
| **Total** | **2305** | |

## Rust API
- `Accordion` (struct)
- `Alert` (struct)
- `Avatar` (struct)
- `Badge` (struct)
- `Breadcrumb` (struct)
- `Button` (struct)
- `Card` (struct)
- `Checkbox` (struct)
- `Chip` (struct)
- `ChipResponse` (struct)
- `CircularProgress` (struct)
- `Code` (struct)
- `Divider` (struct)
- `DropdownMenu` (struct)
- `EmptyState` (struct)
- `FormField` (struct)
- `Heading` (struct)
- `IconButton` (struct)
- `Kbd` (struct)
- `Label` (struct)
- `Link` (struct)
- `Modal` (struct)
- `Pagination` (struct)
- `Progress` (struct)
- `RadioGroup` (struct)
- `Rating` (struct)
- `SegmentedControl` (struct)
- `Select` (struct)
- `Skeleton` (struct)
- `Slider` (struct)
- `Spinner` (struct)
- `Stat` (struct)
- `Stepper` (struct)
- `Switch` (struct)
- `Table` (struct)
- `Tabs` (struct)
- `TextField` (struct)
- `Textarea` (struct)
- `TreeNode` (struct)
- `TreeView` (struct)
- `ToastVariant` (enum)

## Rust Functions
- `close_modal()`
- `context_menu()`
- `hover_card()`
- `open_modal()`
- `popover()`
- `show_toasts()`
- `toast()`
- `toast_error()`
- `toast_success()`
- `toast_warning()`
- `tooltip()`
