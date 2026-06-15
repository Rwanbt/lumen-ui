# Performance budgets

lumen-ui resolves a **recipe** for every widget, every frame. That path must be effectively
free relative to egui's own layout + paint cost so theming never becomes the bottleneck.

## Budgets

| Path | Budget | Why |
|------|--------|-----|
| Recipe resolution (per widget) | **< 1 µs** | A dense screen has ~100–300 widgets; staying ≪ 1 µs keeps total recipe cost in the low tens of µs against a 16.6 ms (60 fps) frame. |
| WCAG palette audit | **< 50 µs** | Dev/test-time check (and optional runtime theme validation), not per-frame. |
| Full UI frame | **< 16.6 ms** | 60 fps. Recipe resolution must be a rounding error within this. |

## Measured (criterion, release)

Run locally with `cargo bench -p lumen-ui-core`. Representative numbers on a desktop x86-64:

| Benchmark | Time | Budget | Margin |
|-----------|------|--------|--------|
| `button_recipe` | ~26 ns | < 1 µs | ~38× under |
| `text_recipe` | ~3.5 ns | < 1 µs | ~280× under |
| `audit_colors` (10 pairs) | ~1.3 µs | < 50 µs | ~38× under |

At ~26 ns/recipe, a 300-widget frame spends **< 10 µs** resolving recipes — about 0.05 % of the
frame budget. Recipe resolution is allocation-free and branch-light by design (a `match` over the
variant/state plus a few color mixes), so this scales linearly and predictably.

## Methodology

- Benchmarks live in `crates/lumen-ui-core/benches/hot_paths.rs` (criterion, `harness = false`).
- They are marked `test = false` so the CI `test` job skips them; `clippy --all-targets` still
  lints them. Run them on demand, not in CI, to avoid noisy timing on shared runners.
- To compare against a baseline: `cargo bench -p lumen-ui-core -- --save-baseline main`, then after a
  change `cargo bench -p lumen-ui-core -- --baseline main`.
