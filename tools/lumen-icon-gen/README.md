# lumen-icon-gen (PoC — ADR-0008)

Proof of concept for the SVG → painter icon pipeline decided in
[ADR-0008](../../docs/adr/0008-svg-icon-codegen.md). It converts SVG path data into
`lumen-ui-icons`-style painter code **at build time**, so the published
`lumen-ui-icons` crate keeps its **no-asset / no-runtime-dependency** guarantee:
only this tool depends on `svgtypes`.

## Pipeline

```
SVG path d="…"
  → svgtypes::SimplifyingPathParser   (rel/smooth/H/V/arc → absolute Move/Line/Curve/Quad/Close)
  → flatten Béziers to polylines      (FLATTEN_STEPS subdivisions)
  → normalize by viewBox to 0..1
  → emit `fn paint_<name>(painter, rect, stroke)` using the `at(rect, x, y)` helper
```

The output matches the hand-drawn style of `lumen-ui-icons`, so generated icons are
drop-in: same `at(rect, x, y)` normalized helper, same `Shape::line` stroke calls.

## Usage

```bash
cargo run -p lumen-icon-gen -- --demo                 # print generated code for built-in samples
cargo run -p lumen-icon-gen -- <in_dir> <out_file>    # generate a module from a dir of *.svg
```

## PoC scope vs production

- **PoC**: handles `<path d>` data; `<path>` attributes are extracted by a minimal
  string scan.
- **Production** (per ADR-0008 §6.3): resolve `<circle>/<rect>/<line>/<polyline>`,
  groups and transforms via `usvg`; curate a Lucide subset; preserve Béziers
  (`CubicBezierShape`) instead of flattening where crisper.

Icons derived from [Lucide](https://lucide.dev) are under the ISC License; keep its
notice with any generated output.
