# AI_SUMMARY — src

> **Auto-generated 2026-06-15 12:31** — do not edit manually.
> Source: `tools/ai_docs/generate_ai_summary.py`
> For purpose, thread model and constraints, read `AI_CONTEXT.md`.

## Purpose
Themed egui widgets that consume `lumen-core` recipes. v0.1 ships `Button`; v0.2 adds the
foundational set (Input, Card, Badge, Switch, Checkbox, RadioGroup, Slider, Label, Heading).
Widgets are the only place that calls egui's drawing API besides `lumen-core::theme`.

## Files & LOC
| File | LOC | |
|------|-----|--|
| `accordion.rs` | 31 | |
| `badge.rs` | 52 | |
| `button.rs` | 105 | |
| `card.rs` | 28 | |
| `checkbox.rs` | 63 | |
| `focus.rs` | 27 | |
| `lib.rs` | 37 | |
| `modal.rs` | 57 | |
| `overlay.rs` | 29 | |
| `radio.rs` | 81 | |
| `select.rs` | 41 | |
| `slider.rs` | 98 | |
| `switch.rs` | 58 | |
| `tabs.rs` | 46 | |
| `text.rs` | 63 | |
| `text_field.rs` | 61 | |
| `toast.rs` | 100 | |
| **Total** | **977** | |

## Rust API
- `Accordion` (struct)
- `Badge` (struct)
- `Button` (struct)
- `Card` (struct)
- `Checkbox` (struct)
- `Heading` (struct)
- `Label` (struct)
- `Modal` (struct)
- `RadioGroup` (struct)
- `Select` (struct)
- `Slider` (struct)
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
