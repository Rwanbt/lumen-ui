# AI_SUMMARY — src

> **Auto-generated 2026-06-15 06:46** — do not edit manually.
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
| `button.rs` | 91 | |
| `card.rs` | 28 | |
| `checkbox.rs` | 55 | |
| `lib.rs` | 36 | |
| `modal.rs` | 57 | |
| `overlay.rs` | 29 | |
| `radio.rs` | 68 | |
| `select.rs` | 41 | |
| `slider.rs` | 66 | |
| `switch.rs` | 46 | |
| `tabs.rs` | 46 | |
| `text.rs` | 63 | |
| `text_field.rs` | 61 | |
| `toast.rs` | 100 | |
| **Total** | **870** | |

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
