# AI_SUMMARY — src

> **Auto-generated 2026-06-16 03:27** — do not edit manually.
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
| `a11y.rs` | 230 | |
| `anim.rs` | 23 | |
| `builder.rs` | 210 | |
| `context.rs` | 57 | |
| `dark.rs` | 184 | |
| `lib.rs` | 38 | |
| `light.rs` | 99 | |
| `palette.rs` | 94 | |
| `recipe.rs` | 280 | |
| `theme.rs` | 107 | |
| `tokens.rs` | 138 | |
| **Total** | **1460** | |

## Rust API
- `AlertRecipe` (struct)
- `AuditReport` (struct)
- `AvatarRecipe` (struct)
- `BadgeRecipe` (struct)
- `ButtonRecipe` (struct)
- `CardRecipe` (struct)
- `ChipRecipe` (struct)
- `Colors` (struct)
- `ContrastCheck` (struct)
- `DarkTheme` (struct)
- `DividerRecipe` (struct)
- `Elevation` (struct)
- `KbdRecipe` (struct)
- `LightTheme` (struct)
- `Motion` (struct)
- `PaletteTheme` (struct)
- `ProgressRecipe` (struct)
- `Radius` (struct)
- `SkeletonRecipe` (struct)
- `SliderRecipe` (struct)
- `Spacing` (struct)
- `SpinnerRecipe` (struct)
- `TextFieldRecipe` (struct)
- `TextRecipe` (struct)
- `ToggleRecipe` (struct)
- `Tokens` (struct)
- `Typography` (struct)
- `UiContext` (struct)
- `AlertVariant` (enum)
- `BadgeVariant` (enum)
- `ButtonVariant` (enum)
- `ContrastLevel` (enum)
- `Density` (enum)
- `TextRole` (enum)
- `ThemeMode` (enum)
- `WidgetState` (enum)
- `Theme` (trait)
- `UiThemeExt` (trait)

## Rust Functions
- `audit_colors()`
- `contrast_ratio()`
- `install()`
- `lerp_color()`
- `meets()`
- `meets_aa()`
- `relative_luminance()`
- `set_theme()`
