# AI_SUMMARY — src

> **Auto-generated 2026-06-14 18:27** — do not edit manually.
> Source: `tools/ai_docs/generate_ai_summary.py`
> For purpose, thread model and constraints, read `AI_CONTEXT.md`.

## Purpose
Foundation layer of lumen-ui. Owns design **tokens**, the ambient **Density/UiContext**, the
**`Theme` trait** with state-parameterized **recipes**, the minimal-motion helper, and the
`install()` entry point that wires a theme into an `egui::Context`. Everything else in the
workspace depends on this crate; it depends only on `egui`.

## Files & LOC
| File | LOC | |
|------|-----|--|
| `anim.rs` | 23 | |
| `context.rs` | 27 | |
| `dark.rs` | 203 | |
| `lib.rs` | 24 | |
| `recipe.rs` | 49 | |
| `theme.rs` | 75 | |
| `tokens.rs` | 134 | |
| **Total** | **535** | |

## Rust API
- `ButtonRecipe` (struct)
- `Colors` (struct)
- `DarkTheme` (struct)
- `Elevation` (struct)
- `Motion` (struct)
- `Radius` (struct)
- `Spacing` (struct)
- `TextRecipe` (struct)
- `Tokens` (struct)
- `Typography` (struct)
- `UiContext` (struct)
- `ButtonVariant` (enum)
- `Density` (enum)
- `TextRole` (enum)
- `WidgetState` (enum)
- `Theme` (trait)
- `UiThemeExt` (trait)

## Rust Functions
- `install()`
- `lerp_color()`
- `set_theme()`
