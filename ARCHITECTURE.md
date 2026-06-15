# Architecture — lumen-ui

> Read this before adding a crate, a widget, or a theme. It describes the layers, the
> data flow, ownership, and the red zones. The full version plan lives in [ROADMAP.md](ROADMAP.md).

## The one idea

`lumen-ui` separates **what a widget is** from **how it looks**:

```
Tokens  ──(Theme resolves)──▶  Recipe  ──(Widget draws)──▶  egui paint
 raw values                  per (variant,state,density)     Frame + Button + …
```

A widget never reads a token. It asks the installed theme for a **recipe** matching its
`(variant, state, density)`, then paints. Swap the theme → every widget restyles, with zero
changes to app logic.

## Dependency direction (acyclic, enforced by the compiler)

```
lumen-ui (façade)
   └─▶ lumen-ui-widgets ─▶ lumen-ui-core
   └─▶ lumen-ui-motion  ─▶ lumen-ui-core      (v0.5)
   └─▶ lumen-ui-layout  ─▶ lumen-ui-core      (v0.4)
   └─▶ lumen-ui-patterns ─▶ widgets+layout (v0.6)
                          all ─▶ lumen-ui-core
```

**Rule:** dependencies only flow *down* toward `lumen-ui-core`. `lumen-ui-core` depends on
nothing but `egui`. A widget crate that needs to reach "up" toward the façade is a design
error — extract the shared concept down into `lumen-ui-core` instead.

## Layers

| Layer | Crate / module | Responsibility |
|-------|----------------|----------------|
| Tokens | `lumen-ui-core::tokens` | Raw visual constants (colors, spacing, radius, elevation, motion) |
| Context | `lumen-ui-core::context` | Ambient `Density` / `UiContext` |
| Recipe | `lumen-ui-core::recipe` | Resolved per-widget style for one `(variant, state)` |
| Theme | `lumen-ui-core::theme` + `dark` | Owns tokens; resolves recipes; maps onto egui `Style`/`Visuals` |
| Motion | `lumen-ui-core::anim` (v0.2) → `lumen-ui-motion` (v0.5) | Time-based interpolation; same call-site API across both |
| Widgets | `lumen-ui-widgets` | Consume recipes, draw with egui |
| Façade | `lumen-ui` | Re-exports + prelude + feature flags |

## Data flow & state ownership

- The installed theme lives in `egui::Context` persisted data under `Id("lumen_theme")` as
  `Arc<dyn Theme>`. The UI context lives under `Id("lumen_ctx")`. Both are read **lock-free**
  via `UiThemeExt` (`ui.theme()`, `ui.ui_ctx()`).
- **Widgets are stateless** — interaction state belongs to egui (`ctx.read_response(id)`),
  not to the application. From v0.3, composed components (modals, toasts) store their own
  open/visible state in `ctx.data` keyed by `Id`, never in the host `App`.

## Red zones (touch with care)

1. **`lumen_ui_core::theme` (the `Theme` trait).** This is the most fundamental contract. Recipes
   are parameterized by `(variant, state, ctx)` *from v0.1* precisely so the trait does not
   break when new states/variants arrive. Adding a method or changing a recipe signature is a
   breaking change for every theme and widget — gate behind an ADR.
2. **The egui adaptation surface.** egui ships ~3 minor releases/year with breaking changes.
   All egui API contact is concentrated in `lumen-ui-core` (and the thin widget impls). Pin egui
   strictly (`workspace.dependencies`); bump deliberately with a compat-matrix check.
3. **Frame-N-1 interaction state.** Hover/active are only known after allocation, so widgets
   read the *previous* frame's response. `install()` sets `max_passes = 2` to keep this stable.
   Do not "optimize" this away.

## Motion: the transparent swap (v0.2 → v0.5)

In v0.2, widgets interpolate via `lumen_ui_core::anim::lerp_color` (built on
`ctx.animate_value_with_time` — no heavy dependency). In v0.5 the same call sites move to the
`lumen-ui-motion` spring solver **with no change to any widget's public API**. This is why motion
is "minimal but present" from v0.2 rather than bolted on at v0.5.

## Performance budgets (verified in CI from v0.5)

| Path | Budget |
|------|--------|
| Per-widget recipe resolution | allocation-free on the hot path |
| Animation step (v0.5) | < 1 ms / frame |
| 60 fps frame | < 16.6 ms |

## Shutdown / lifecycle

`lumen-ui` holds no OS resources and spawns no threads — the theme `Arc` is dropped when the
`egui::Context` data store is cleared. There is no shutdown sequence to coordinate.
