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

1. **✅ crates.io naming — resolved (ADR-0007).** The original `lumen-core` package name was
   taken on crates.io (an unrelated ML crate), so all internal crates were renamed to the
   `lumen-ui-*` namespace, aligned with the façade. Availability verified 2026-06-15 — every
   target name is **free**:

   | crates.io name | Status |
   |----------------|--------|
   | `lumen-ui` (façade) | free ✅ |
   | `lumen-ui-core` · `-widgets` · `-layout` · `-motion` · `-patterns` · `-themes` · `-icons` | all free ✅ |

   Re-verify at publish time (someone could claim a name in the interim), but no further rename
   is planned. `tools/lumen-theme-gen` is a dev utility and is **not** published.
2. **Repo public** — done (2026-06-15). Required for GitHub Pages (the book / WASM gallery);
   independent of crates.io.
3. **Go/no-go** — publishing cannot be undone (a version can be *yanked* but not deleted).

## Tag the release

```bash
git tag -a v1.0.0 -m "lumen-ui 1.0.0"
git push origin v1.0.0
```

## Publish to crates.io (in dependency order)

Path+version deps must already exist on crates.io, so publish **bottom-up**. Wait for each to be
indexed (usually seconds) before the next.

```bash
cargo login                      # once, with your crates.io token

cargo publish -p lumen-ui-core
cargo publish -p lumen-ui-widgets   # depends on lumen-ui-core
cargo publish -p lumen-ui-layout
cargo publish -p lumen-ui-motion
cargo publish -p lumen-ui-themes
cargo publish -p lumen-ui-icons
cargo publish -p lumen-ui-patterns  # depends on widgets + layout
cargo publish -p lumen-ui        # façade — depends on all of the above
# tools/lumen-theme-gen has publish = false (dev tool) — not published.
```

Dry-run a single leaf crate first to sanity-check packaging:

```bash
cargo publish -p lumen-ui-core --dry-run
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
