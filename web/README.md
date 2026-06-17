# lumen-ui — WASM gallery

A small showcase of lumen-ui that runs in the browser (and natively), used for the GitHub Pages
demo. It's a **standalone crate** (its own `[workspace]`), so it never weighs on the library CI; it
path-depends on the workspace crates.

## Run natively

```bash
cd web
cargo run
```

## Run / build for the web (Trunk)

```bash
cargo install --locked trunk          # once
cd web
trunk serve                            # live dev server at http://localhost:8080
trunk build --release                  # static site in web/dist/
```

## Deploy

The `Pages (mdBook + WASM gallery)` workflow (`.github/workflows/pages.yml`, **manual dispatch**)
builds the book + this gallery and deploys both to GitHub Pages — the book at the site root, the
gallery under `/gallery/`. Requires Pages enabled (public repo or paid plan) with
*Settings → Pages → Source: GitHub Actions*. The CI `wasm` job compile-checks this crate on every PR.
