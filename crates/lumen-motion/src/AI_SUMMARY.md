# AI_SUMMARY — src

> **Auto-generated 2026-06-15 06:46** — do not edit manually.
> Source: `tools/ai_docs/generate_ai_summary.py`
> For purpose, thread model and constraints, read `AI_CONTEXT.md`.

## Purpose
Animation engine for lumen-ui: spring physics + easing curves (+ transitions in slice 2).
The richer counterpart to `lumen-core::anim` (the dependency-free minimal motion baked into
widgets). Enabled via the façade `motion` feature. Depends only on `egui`.

## Files & LOC
| File | LOC | |
|------|-----|--|
| `easing.rs` | 93 | |
| `lib.rs` | 66 | |
| `spring.rs` | 118 | |
| `transitions.rs` | 38 | |
| **Total** | **315** | |

## Rust API
- `Spring` (struct)
- `Easing` (enum)

## Rust Functions
- `ease()`
- `fade()`
