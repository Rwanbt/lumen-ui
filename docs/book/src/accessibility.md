# Accessibility

Accessibility is built in, not bolted on. Every built-in theme is **WCAG 2.1 AA audited in CI**,
and the interactive widgets are keyboard-navigable with visible focus and finger-sized hit targets.

## Contrast audit

`lumen-core::a11y` is pure WCAG contrast math:

```rust,ignore
use lumen_ui::a11y::{self, ContrastLevel};
use lumen_ui::ContrastLevel as _;

let ratio = a11y::contrast_ratio(fg, bg);          // 1.0 – 21.0
let ok = a11y::meets_aa(fg, bg);                   // 4.5:1 body text
let ok = a11y::meets(fg, bg, ContrastLevel::Aaa);  // 7:1
```

Audit a whole palette — every text-bearing semantic pair at once:

```rust,ignore
let report = a11y::audit_colors(&my_theme.tokens().colors);
assert!(report.all_pass(), "{:?}", report.failures().collect::<Vec<_>>());
```

`ContrastLevel` is `Aa` (4.5) / `AaLarge` (3.0) / `Aaa` (7.0) / `AaaLarge` (4.5). The audit holds
text — including `text_muted` — to AA at the resting state (see
[ADR-0006](https://github.com/Rwanbt/lumen-ui/blob/main/docs/adr/0006-wcag-aa-audit.md) for the
scope rationale). Adopt the same one-line test for your custom themes.

## Keyboard navigation

- All interactive widgets use `Sense::click`, so egui gives them **Tab focus** and
  **Space/Enter activation** for free.
- `Slider` additionally responds to **arrow keys** (±1 % of the range per press) when focused.

## Visible focus

Focused widgets draw a 2 px primary **focus ring** (shared `focus_ring` helper across
Button/Switch/Checkbox/Slider). The theme also sets a global `widgets.active.bg_stroke`, so stock
egui widgets indicate focus too.

## Touch targets

`UiContext::min_interactive_size()` returns **44 px in `Touch`** density (WCAG 2.5.5), and never
drops below the 24 px AA floor at any density. Button/Switch/Checkbox/Slider enforce it as their
hit target. Switch to touch density when targeting tablets/handhelds:

```rust,ignore
install(ctx, theme, UiContext { density: Density::Touch });
```

> The `material` adapter (egui-material3) is **not** part of v1.0 — it targets egui 0.33 and pulls
> ~465 transitive crates. See
> [ADR-0005](https://github.com/Rwanbt/lumen-ui/blob/main/docs/adr/0005-defer-material-adapter.md).

Next: [Architecture](architecture.md).
