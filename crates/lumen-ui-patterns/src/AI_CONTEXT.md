# AI_CONTEXT — lumen-ui-patterns

## Purpose
Ready-made application patterns composing `lumen-ui-widgets` + egui panels into common app
structures, so wiring a new app is a few lines. Enabled via the façade `patterns` feature.
Depends on `egui` + `lumen-ui-core` + `lumen-ui-widgets`.

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
    .toolbar(|ui| { ui.add(lumen_ui_widgets::Button::primary("New")); })
    .sidebar(|ui| { /* nav */ })
    .inspector(|ui| { /* properties */ })
    .status_bar(|ui| { ui.add(lumen_ui_widgets::Label::muted("Ready")); })
    .show(ui, |ui| { /* central content */ });
```

## Modules
- `dashboard.rs` — `DashboardLayout` (optional toolbar/status_bar/sidebar/inspector + central).
- `bars.rs` — `Toolbar` / `StatusBar` horizontal bar helpers.
- `sidebar.rs` — `Sidebar` vertical nav (bound to `&mut usize`, full-width entries).
- `rows.rs` — `property_row(ui, label, |ui| control)`, `SettingsPage` (scroll) / `InspectorPanel`.
- `logpanel.rs` — `LogPanel` + `LogEntry`/`LogLevel` (severity badges, stick-to-bottom scroll).
- `command_palette.rs` — `CommandPalette` (+ `open_command_palette`); open state + query in
  `ctx.data`, modal overlay with live filter, returns the chosen command index.
- `form.rs` — `Form` (field stack + optional actions footer; tokenized gaps; pure widget
  composition, no recipe).
- `auth_card.rs` — `AuthCard` (+ `AuthCardResponse`): centered, width-constrained login card
  composing `Card`/`FormField`/`TextField`/`Button` (+ optional remember `Checkbox` and secondary
  `Link`). Returns which actions fired (`submitted` / `secondary_clicked`).
- `master_detail.rs` — `MasterDetail`: resizable left list (themed `Sidebar`, bound to
  `&mut usize`) + central detail pane via `show_inside`. Detail closure receives the selected
  index. Same `&mut Ui` convention as `DashboardLayout`.
