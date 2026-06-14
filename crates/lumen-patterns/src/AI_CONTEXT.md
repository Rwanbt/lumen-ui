# AI_CONTEXT — lumen-patterns

## Purpose
Ready-made application patterns composing `lumen-widgets` + egui panels into common app
structures, so wiring a new app is a few lines. Enabled via the façade `patterns` feature.
Depends on `egui` + `lumen-core` + `lumen-widgets`.

## Constraints
- Requires a theme installed via `lumen_ui::install(..)` (panels are themed by the global visuals
  set by `apply_to_ctx`).
- egui 0.34 reworked panels: use `egui::Panel` nested via `show_inside(ui, ...)` (the old
  `SidePanel/TopBottomPanel/CentralPanel::show(ctx, ...)` are deprecated). `DashboardLayout::show`
  therefore takes the `&mut Ui` from `eframe::App::ui` (no raw ctx needed).
- Region order is fixed inside `show` (top, bottom, left, right, central) per egui's requirements.

## Forbidden
- `#![forbid(unsafe_code)]`. No panics.

## Common patterns
```ignore
DashboardLayout::new()
    .toolbar(|ui| { ui.add(lumen_widgets::Button::primary("New")); })
    .sidebar(|ui| { /* nav */ })
    .inspector(|ui| { /* properties */ })
    .status_bar(|ui| { ui.add(lumen_widgets::Label::muted("Ready")); })
    .show(ui, |ui| { /* central content */ });
```

## Modules
- `dashboard.rs` — `DashboardLayout` (optional toolbar/status_bar/sidebar/inspector + central).
- `bars.rs` — `Toolbar` / `StatusBar` horizontal bar helpers.
- (next slices) sidebar nav, settings/inspector rows, log panel, command palette.
