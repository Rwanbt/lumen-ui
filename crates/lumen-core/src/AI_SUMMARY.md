# AI_SUMMARY — src

> **Auto-generated 2026-06-14 23:16** — do not edit manually.
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
| `builder.rs` | 200 | |
| `context.rs` | 27 | |
| `dark.rs` | 183 | |
| `lib.rs` | 30 | |
| `light.rs` | 98 | |
| `recipe.rs` | 104 | |
| `theme.rs` | 88 | |
| `tokens.rs` | 138 | |
| **Total** | **891** | |

## Rust API
- `BadgeRecipe` (struct)
- `ButtonRecipe` (struct)
- `CardRecipe` (struct)
- `Colors` (struct)
- `DarkTheme` (struct)
- `Elevation` (struct)
- `LightTheme` (struct)
- `Motion` (struct)
- `Radius` (struct)
- `SliderRecipe` (struct)
- `Spacing` (struct)
- `TextFieldRecipe` (struct)
- `TextRecipe` (struct)
- `ToggleRecipe` (struct)
- `Tokens` (struct)
- `Typography` (struct)
- `UiContext` (struct)
- `BadgeVariant` (enum)
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
