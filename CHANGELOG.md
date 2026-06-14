# Changelog

All notable changes to lumen-ui are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html) (strict from v1.0).

## [Unreleased]

### Added — v0.3 (in progress)

- **`Tabs`** headless tab bar — selection persisted in `ctx.data` (no external index);
  `Tabs::new(id).tab(..).show(ui) -> usize`. Composes themed `Button`s.
- **`Accordion`** themed collapsible section over `egui::CollapsingHeader`.

### Added — v0.2

- **`LightTheme`** — a light counterpart to `DarkTheme`. Recipe rules are centralized in a
  shared `builder` module parameterized by tokens + an emphasis fn (lighten/darken), so a theme
  is now just a palette (DRY).
- **`examples/gallery.rs`** — every widget on one screen with a **live Dark/Light theme switch**
  (the v0.2 exit criterion: toggling restyles everything, zero widget changes).
- **`TextField`** themed single-line input (`&mut String`, hint, password), focus-highlighted
  border (frame N-1), via `TextFieldRecipe` + `Theme::text_field_recipe`.
- **`RadioGroup`** generic single-selection (`&mut T`, builder `.option(value, label)`).
- **`WidgetState::Focused`** added for text input (handled across recipes).
- **`Slider`** draggable value control (`&mut f32` over a range), via new `SliderRecipe` +
  `Theme::slider_recipe(state, ctx)`.
- **`Switch`** (animated knob via `animate_bool_with_time`) and **`Checkbox`** (labelled),
  bound to `&mut bool`, styled by a new `ToggleRecipe` + `Theme::toggle_recipe(on, state, ctx)`.
- **`Card`** themed surface container (`Card::show`) and **`Badge`** status label
  (Neutral/Primary/Success/Warning/Danger), via new `CardRecipe` / `BadgeRecipe` +
  `Theme::card_recipe` / `Theme::badge_recipe`.
- **Semantic color tokens**: `success`/`on_success`, `warning`/`on_warning` added to `Colors`.
- **Minimal motion wired into `Button`**: the fill now interpolates toward its target state
  color via `anim::lerp_color` (ADR-0003). Swaps to `lumen-motion` springs in v0.5 with no API
  change.
- **Text primitives**: `Label` (+ `Label::muted`) and `Heading` (+ `Heading::display`),
  driven by a new `TextRecipe` / `TextRole` and the `Theme::text_recipe` method.
- Unit tests for `DarkTheme` recipe resolution (text roles, hover, ghost border, touch density).

### Added — v0.1 foundation (2026-06-14)

- **Workspace from day zero** (`lumen-core`, `lumen-widgets`, façade `lumen-ui`).
- `lumen-core`: design `Tokens` (colors, spacing, radius, typography, elevation, motion);
  `Density`/`UiContext` ambient parameters; the `Theme` trait with **state-parameterized
  recipes** `(variant, state, ctx)`; `install()` / `set_theme()` and the `UiThemeExt`
  accessor; `DarkTheme` bootstrap theme; minimal-motion helper `anim::lerp_color`.
- `lumen-widgets`: `Button` (Primary/Secondary/Ghost/Danger) built on the verified egui 0.34
  API (`Frame` for padding+shadow, frame-N-1 interaction state).
- `lumen-ui`: façade with feature flags (`tokens`, `theme`, `widgets`, `serde`, `full`) and a
  `prelude`.
- `examples/minimal.rs` validating live theming end-to-end (eframe 0.34).
- Documentation: ROADMAP, ARCHITECTURE, CONTRIBUTING, glossary, ADR-0001..0004.
- Dual MIT OR Apache-2.0 licensing.
- CI: fmt + clippy `-D warnings` + tests + example build + LOC gate on Linux/macOS/Windows.
- AI-native dev stack wired in (AGENTS.md, `tools/ai_docs`, verify-* skills, AI_CONTEXT per crate).

[Unreleased]: https://github.com/Rwanbt/lumen-ui/commits/main
