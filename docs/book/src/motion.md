# Motion

`lumen-ui-motion` (feature `motion`) provides frame-rate-independent springs, easing tweens, and
transitions. State lives in `ctx.data` keyed by an `Id`, and the context repaints while a value is
still moving. Widgets already use the minimal-motion helper in `lumen-ui-core`; reach for this crate
when you want richer, physical motion.

## Springs

```rust,ignore
use lumen_ui::prelude::*;   // Spring, ease, fade, Easing
use eframe::egui;

// Presets: SMOOTH, GENTLE, WOBBLY, STIFF — or build your own.
let x = Spring::SMOOTH.animate(ctx, egui::Id::new("panel-x"), target_x);

// Custom spring (stiffness / damping / mass):
let custom = Spring { stiffness: 200.0, damping: 20.0, mass: 1.0 };
let v = custom.animate(ctx, egui::Id::new("v"), target);

// Animate a color:
let fill = Spring::GENTLE.animate_color(ctx, egui::Id::new("fill"), target_color);
```

## Easing tweens

```rust,ignore
// ease(ctx, id, target, duration_seconds, easing) -> current value
let t = ease(ctx, egui::Id::new("fade"), 1.0, 0.2, Easing::EaseInOut);

// CSS cubic-bézier control points are supported:
let t = ease(ctx, id, 1.0, 0.25, Easing::CubicBezier(0.4, 0.0, 0.2, 1.0));
```

`Easing` is `Linear | EaseIn | EaseOut | EaseInOut | CubicBezier(x1, y1, x2, y2)`.

## Transitions

`fade` animates a panel's opacity in/out; when fully hidden, its content isn't laid out.

```rust,ignore
fade(ui, egui::Id::new("details"), show_details, |ui| {
    ui.add(Label::new("revealed when show_details is true"));
});
```

See `cargo run -p lumen-ui --example motion`. Next: [Patterns](patterns.md).
