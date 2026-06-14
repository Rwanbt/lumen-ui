# Changelog

All notable changes to lumen-ui are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html) (strict from v1.0).

## [Unreleased]

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
