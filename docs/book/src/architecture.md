# Architecture

A condensed map. The full document is
[`ARCHITECTURE.md`](https://github.com/Rwanbt/lumen-ui/blob/main/ARCHITECTURE.md) at the repo root,
and decisions are recorded as
[ADRs](https://github.com/Rwanbt/lumen-ui/tree/main/docs/adr).

## Workspace & dependency direction

A Cargo workspace from day zero (ADR-0001). Dependencies flow **down toward `lumen-ui-core` only** —
nothing depends "up" toward the façade.

```text
            lumen-ui (façade)
   ┌───────────┬───────────┬───────────┐
 widgets    layout      motion      patterns ──► widgets + layout
   │           │           │
   └───────────┴───────────┴──────────────────► lumen-ui-core
themes ─► lumen-ui-core      icons ─► egui
```

## The frozen core

`lumen-ui-core::theme` is the **frozen core**. The `Theme` trait's recipe methods are parameterized
by `(variant, state, ctx)` from day one (ADR-0002), so new states, variants, or context fields are
**additive, not breaking**. Changing a recipe or trait signature requires an ADR.

## egui adaptation layer

All contact with egui's API is concentrated in `lumen-ui-core` (and the thin widget impls), ADR-0004.
egui is pinned in the workspace (`0.34.3`); a bump is handled in one place. Every egui signature is
verified by compilation before commit — the original multi-AI design code hallucinated methods that
don't exist (`Button::padding`, `Button::shadow`), and the golden rule is to never repeat that.

## State model

Widgets are stateless. Interaction state is read from the **previous frame** (`ctx.read_response`),
and `install()` sets `max_passes = 2` so hover/active are stable. Composed components (Modal, Toast,
Tabs, CommandPalette) keep their open/visible state in `ctx.data` keyed by an `Id` — no app-owned
booleans.

## Key ADRs

| ADR | Decision |
|-----|----------|
| 0001 | Workspace from day zero |
| 0002 | Recipes parameterized by state |
| 0003 | Minimal motion in v0.2, spring engine in v0.5 (no API change) |
| 0004 | MSRV 1.92 + single egui pin/adaptation layer |
| 0005 | `material` adapter deferred (egui-material3 targets egui 0.33) |
| 0006 | WCAG AA contrast audit (scope = resting state) |
