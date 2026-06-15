# AI_CONTEXT — lumen-ui-motion

## Purpose
Animation engine for lumen-ui: spring physics + easing curves (+ transitions in slice 2).
The richer counterpart to `lumen-ui-core::anim` (the dependency-free minimal motion baked into
widgets). Enabled via the façade `motion` feature. Depends only on `egui`.

## Constraints
- Animation state lives in `ctx.data` keyed by a stable `Id` (spring value+velocity, tween
  from/to/start_time). The `Id` MUST be stable across frames per animated quantity.
- Frame-rate independent: integrates with `ctx.input(|i| i.stable_dt)` (clamped to 1/30 s).
- Requests a repaint while still moving; stops when settled (value+velocity under epsilon).
- `lumen-ui-core::anim` stays the default for widgets (no heavy dep); `lumen-ui-motion` is opt-in for
  app/pattern code and transitions (ADR-0003: same call-site shape, spring/easing-backed).

## Forbidden
- `#![forbid(unsafe_code)]`. No panics.
- Don't allocate per-frame in the animation path beyond the small `ctx.data` entries.

## Common patterns
```rust
use lumen_ui_motion::{Spring, ease, Easing};

let w = Spring::SMOOTH.animate(ctx, egui::Id::new("panel"), if open { 240.0 } else { 0.0 });
let a = ease(ctx, egui::Id::new("fade"), if open { 1.0 } else { 0.0 }, 0.2, Easing::EaseOut);
let col = Spring::WOBBLY.animate_color(ctx, egui::Id::new("c"), target_color);
```

## Modules
- `spring.rs` — `Spring` (stiffness/damping/mass + SMOOTH/GENTLE/WOBBLY/STIFF presets);
  `animate(ctx,id,target)`, `animate_color(...)`.
- `easing.rs` — `Easing` (Linear/EaseIn/EaseOut/EaseInOut/CubicBezier) with a CSS bézier solver.
- `lib.rs` — `ease(ctx, id, target, duration, easing)` tween helper.
- `transitions.rs` — `fade(ui, id, visible, |ui| …)` opacity transition (returns `None` when fully
  hidden). Slide/scale need sub-tree transforms egui lacks ergonomically → app-level for now.
