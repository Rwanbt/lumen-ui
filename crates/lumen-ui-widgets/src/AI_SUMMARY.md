# AI_SUMMARY — src

> **Auto-generated 2026-06-16 03:28** — do not edit manually.
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
| `alert.rs` | 60 | |
| `avatar.rs` | 50 | |
| `badge.rs` | 52 | |
| `button.rs` | 105 | |
| `card.rs` | 28 | |
| `checkbox.rs` | 63 | |
| `chip.rs` | 64 | |
| `divider.rs` | 51 | |
| `focus.rs` | 27 | |
| `kbd.rs` | 34 | |
| `lib.rs` | 53 | |
| `modal.rs` | 57 | |
| `overlay.rs` | 29 | |
| `progress.rs` | 31 | |
| `radio.rs` | 81 | |
| `select.rs` | 41 | |
| `skeleton.rs` | 24 | |
| `slider.rs` | 98 | |
| `spinner.rs` | 25 | |
| `switch.rs` | 58 | |
| `tabs.rs` | 46 | |
| `text.rs` | 63 | |
| `text_field.rs` | 61 | |
| `toast.rs` | 100 | |
| **Total** | **1332** | |

## Rust API
- `Accordion` (struct)
- `Alert` (struct)
- `Avatar` (struct)
- `Badge` (struct)
- `Button` (struct)
- `Card` (struct)
- `Checkbox` (struct)
- `Chip` (struct)
- `ChipResponse` (struct)
- `Divider` (struct)
- `Heading` (struct)
- `Kbd` (struct)
- `Label` (struct)
- `Modal` (struct)
- `Progress` (struct)
- `RadioGroup` (struct)
- `Select` (struct)
- `Skeleton` (struct)
- `Slider` (struct)
- `Spinner` (struct)
- `Switch` (struct)
- `Tabs` (struct)
- `TextField` (struct)
- `ToastVariant` (enum)

## Rust Functions
- `close_modal()`
- `context_menu()`
- `open_modal()`
- `popover()`
- `show_toasts()`
- `toast()`
- `toast_error()`
- `toast_success()`
- `toast_warning()`
- `tooltip()`
