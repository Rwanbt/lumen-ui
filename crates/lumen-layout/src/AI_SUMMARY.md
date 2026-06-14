# AI_SUMMARY — src

> **Auto-generated 2026-06-15 01:19** — do not edit manually.
> Source: `tools/ai_docs/generate_ai_summary.py`
> For purpose, thread model and constraints, read `AI_CONTEXT.md`.

## Purpose
CSS-style flexbox layout + responsive breakpoints for lumen-ui, over `egui_taffy`
(the roadmap's highest-risk dependency, isolated here behind the façade `layout` feature).
Depends only on `egui` + `egui_taffy` — not on `lumen-core`.

## Files & LOC
| File | LOC | |
|------|-----|--|
| `lib.rs` | 228 | |
| **Total** | **228** | |

## Rust API
- `Flex` (struct)
- `Grid` (struct)
- `Align` (enum)
- `Breakpoint` (enum)
- `Justify` (enum)
- `FlexUiExt` (trait)

## Rust Functions
- `responsive()`
