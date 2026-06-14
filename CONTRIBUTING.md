# Contributing to lumen-ui

Thanks for helping build lumen-ui. This project follows a strict-but-light engineering bar so
the codebase stays understandable as it grows. Read [ARCHITECTURE.md](ARCHITECTURE.md) and
[ROADMAP.md](ROADMAP.md) first.

## Golden rule: never hallucinate the egui API

The single biggest risk for this project (see [ROADMAP.md §E](ROADMAP.md)) is committing code
against an egui method that doesn't exist. **Every egui signature must be verified against
docs.rs *and* by local compilation before commit.** The CI build is the backstop, not the
first check.

## Before you push

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --workspace --examples
```

All four must pass. CI runs the same on Linux/macOS/Windows + WASM.

## Conventions

- **Language:** English everywhere in code, comments, commits, and PRs.
- **Naming:** Rust `snake_case` / `CamelCase`; explicit names (`button_recipe`, not `br`).
- **Comments document *why*, never *what*.** If a comment restates the code, rename instead.
- **No magic numbers/strings:** name every non-trivial literal as a `const`.
- **Error handling:** no `unwrap()`/`expect()` in library code unless the invariant is proven;
  annotate survivors with `// SAFETY: …`. `lumen-core` is `#![forbid(unsafe_code)]`.
- **File size:** signal at >500 LOC, refactor before adding at >800 LOC. Functions ≤ 50 LOC,
  cyclomatic complexity ≤ 10.

## Adding a widget

1. It lives in `lumen-widgets` and **must** resolve its style from a theme recipe via
   `ui.theme()` — never hard-code a color or padding.
2. If it needs a new recipe shape, add it to `lumen-core::recipe` and a method on the `Theme`
   trait. **This is a breaking change** — open an ADR (`docs/adr/`) first.
3. Read interaction state from the previous frame (`ctx.read_response(id)`); widgets are
   stateless.
4. Add it to `examples/gallery.rs` and write an `egui_kittest` snapshot test where practical.

## Adding a crate

Create the crate only when its roadmap version arrives (no speculative empty crates).
Dependencies must flow *down* toward `lumen-core` — see ARCHITECTURE.md. Wire its feature into
the `lumen-ui` façade in the same PR.

## ADRs

Non-trivial architectural decisions (new central pattern, dependency choice, trait change,
data-format change) get an ADR in [docs/adr/](docs/adr/) using the existing template numbering.

## Pull requests

- ≤ 400 LOC changed per PR; split larger work into independently mergeable PRs.
- Each PR must keep the build green on its own (no "preparation" PR that breaks the build).
- Squash-merge; conventional-commit-style title (`feat:`, `fix:`, `docs:`, `refactor:`).
- Update [CHANGELOG.md](CHANGELOG.md) under `## [Unreleased]`.

## License

By contributing you agree your work is dual-licensed under MIT OR Apache-2.0.
