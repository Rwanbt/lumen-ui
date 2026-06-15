# Releasing lumen-ui

This is the playbook for cutting a release and (optionally) publishing to crates.io.
**Publishing is an irreversible, outward-facing action** — crate names are permanent and code
becomes public. Do it deliberately.

## Pre-flight (already done for 1.0.0)

- [x] Public API frozen and documented — [docs/api-freeze.md](docs/api-freeze.md).
- [x] Version set to the target in `[workspace.package]` and the internal `[workspace.dependencies]`.
- [x] `CHANGELOG.md` has a dated section for the version.
- [x] Compatibility matrix current (README) — egui 0.34.x, MSRV 1.92.
- [x] Green CI: fmt, clippy `-D warnings` (all-targets), tests (Linux/macOS/Windows), MSRV, wasm,
      mdbook, LOC gate.

## Owner decisions required before publishing

1. **⚠️ crates.io naming collision — `lumen-core` is already taken.** Checked 2026-06-15:
   `lumen-core` exists on crates.io (v0.5.0, an unrelated "tiny ML framework"), so we **cannot**
   publish under that name. The other names (`lumen-widgets`, `lumen-layout`, `lumen-motion`,
   `lumen-patterns`, `lumen-themes`, `lumen-icons`, `lumen-ui`) appear free, but verify at publish
   time. **A consistent scheme is needed.** Recommended: namespace the internal crates under the
   façade name, which is free:

   | Workspace dir | Current package name | Suggested crates.io name |
   |---------------|----------------------|--------------------------|
   | `crates/lumen-core` | `lumen-core` ❌ taken | `lumen-ui-core` |
   | `crates/lumen-widgets` | `lumen-widgets` | `lumen-ui-widgets` |
   | `crates/lumen-layout` | `lumen-layout` | `lumen-ui-layout` |
   | `crates/lumen-motion` | `lumen-motion` | `lumen-ui-motion` |
   | `crates/lumen-patterns` | `lumen-patterns` | `lumen-ui-patterns` |
   | `crates/lumen-themes` | `lumen-themes` | `lumen-ui-themes` |
   | `crates/lumen-icons` | `lumen-icons` | `lumen-ui-icons` |
   | `crates/lumen-ui` | `lumen-ui` ✅ | `lumen-ui` (façade, unchanged) |

   Renaming touches each crate's `[package].name`, the `[workspace.dependencies]` keys + the
   `package = "…"` field on each internal dependency, and the `use lumen_core::…` paths
   (`lumen_ui_core`). It is a mechanical but cross-cutting change — do it as one dedicated PR
   **before** the first publish. Alternatively pick a different brand entirely if `lumen` is too
   contested. This is a branding decision left to the owner.
2. **Repo public?** — required for GitHub Pages (the book / WASM gallery). Independent of crates.io.
3. **Go/no-go** — publishing cannot be undone (a version can be *yanked* but not deleted).

## Tag the release

```bash
git tag -a v1.0.0 -m "lumen-ui 1.0.0"
git push origin v1.0.0
```

## Publish to crates.io (in dependency order)

Path+version deps must already exist on crates.io, so publish **bottom-up**. Wait for each to be
indexed (usually seconds) before the next. (Use the final package names — e.g. `lumen-ui-core` —
if you applied the renaming from decision 1.)

```bash
cargo login                      # once, with your crates.io token

cargo publish -p lumen-core
cargo publish -p lumen-widgets   # depends on lumen-core
cargo publish -p lumen-layout
cargo publish -p lumen-motion
cargo publish -p lumen-themes
cargo publish -p lumen-icons
cargo publish -p lumen-patterns  # depends on widgets + layout
cargo publish -p lumen-ui        # façade — depends on all of the above
# tools/lumen-theme-gen has publish = false (dev tool) — not published.
```

Dry-run a single leaf crate first to sanity-check packaging:

```bash
cargo publish -p lumen-core --dry-run
```

> Note: `lumen-ui` sets `readme = "../../README.md"`. If `cargo package` rejects a readme outside
> the crate dir, either copy `README.md` into `crates/lumen-ui/` and point `readme` at it, or drop
> the `readme` key (the `description` still shows on crates.io).

## Post-publish

- Create a GitHub Release from the `v1.0.0` tag, pasting the `CHANGELOG.md` 1.0.0 section.
- If the repo is public: enable Pages (Settings → Pages → Source: GitHub Actions) and run the
  **Pages (mdBook)** workflow to publish the book.
- Announce (optional): r/rust, the egui community, This Week in Rust.

## Subsequent releases

Bump `[workspace.package].version` and every internal dep `version` together, add a dated
`CHANGELOG.md` section, then repeat *Tag* → *Publish*.
