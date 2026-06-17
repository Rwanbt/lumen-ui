# AI_CONTEXT — lumen-ui-widgets

## Purpose
Themed egui widgets that consume `lumen-ui-core` recipes. v0.1 ships `Button`; v0.2 adds the
foundational set (Input, Card, Badge, Switch, Checkbox, RadioGroup, Slider, Label, Heading).
Widgets are the only place that calls egui's drawing API besides `lumen-ui-core::theme`.

## Constraints
- A widget MUST resolve its style from `ui.theme().<widget>_recipe(variant, state, &ui.ui_ctx())`.
  Never hard-code a color, padding, or radius.
- Interaction state is read from the **previous frame** via `ctx.read_response(id)` — egui only
  knows hover/active after allocation. `install()` (in core) sets `max_passes = 2` for this.
- egui 0.34 reality: `egui::Button` has no `.padding()`/`.shadow()`. Padding + shadow come from
  wrapping the button in an `egui::Frame` (`inner_margin`, `shadow`, `corner_radius`, `fill`).
- Widgets are **stateless**: no app-owned booleans. From v0.3, composed components keep their
  open/visible state in `ctx.data` keyed by `Id`.

## Forbidden
- No `unwrap()`/`expect()` outside tests. `#![forbid(unsafe_code)]`.
- Do not store an `egui::Button` (it borrows its content atoms `Button<'a>`); build and `add`
  it inline at draw time.

## Common patterns
```rust
use lumen_ui_widgets::Button;

if ui.add(Button::primary("Save")).clicked() { /* ... */ }
ui.add(Button::ghost("Cancel").enabled(false));
```

## Modules
- `button.rs` — the `Button` widget (Primary/Secondary/Ghost/Danger), Frame+Button technique.
  v0.2: fill interpolates via `lumen_ui_core::anim::lerp_color` (minimal motion).
- `text.rs` — `Label` (+ `muted`) and `Heading` (+ `display`); resolve color/size from
  `Theme::text_recipe(role, ctx)`.
- `card.rs` — `Card` themed surface container; exposes `show(ui, |ui| …)` (not a `Widget`).
- `badge.rs` — `Badge` status label (Neutral/Primary/Success/Warning/Danger).
- `switch.rs` — `Switch` (`&mut bool`), animated knob; custom painter (rect+circle).
- `checkbox.rs` — `Checkbox` (`&mut bool` + label); custom box + check mark.
  Both use `Theme::toggle_recipe`.
- `slider.rs` — `Slider` (`&mut f32`, inclusive range); custom track+fill+knob, drag handling;
  uses `Theme::slider_recipe`.
- `radio.rs` — `RadioGroup<T>` generic single-selection (builder `.option`); custom ring+dot.
- `text_field.rs` — `TextField` (`&mut String`, hint, password) wrapping `TextEdit` in a themed
  `Frame` (`TextEdit::frame(Frame::NONE)`); focus border via frame N-1; `Theme::text_field_recipe`.
- `tabs.rs` — `Tabs` headless tab bar; selection persisted in `ctx.data`; composes `Button`.
- `accordion.rs` — `Accordion` themed collapsible (wraps `egui::CollapsingHeader`).
- `toast.rs` — free fns `toast`/`toast_success|warning|error` + `show_toasts(ctx)`; queue in
  `ctx.data`, auto-dismiss via egui frame time, rendered in a foreground `Area`.
- `modal.rs` — `Modal` (+ `open_modal`/`close_modal`); open state in `ctx.data`, wraps
  `egui::Modal` (backdrop + Esc close). `show` returns `None` while closed.
- `drawer.rs` — `Drawer` (+ `open_drawer`/`close_drawer`, `DrawerSide`): off-canvas side panel over
  a scrim. Reuses `egui::Modal` (backdrop/input-block/Esc) with its area anchored to a screen edge
  and the panel drawn full-height (`content_rect().height()`). Open state in `ctx.data`; `show`
  returns `None` while closed. `DrawerRecipe` (fill/width/inner_margin). Motion instant (slide → v1.6).
- `select.rs` — `Select<T>` dropdown (`&mut T`) over `egui::ComboBox` + `selectable_value`.
- `overlay.rs` — `tooltip`/`popover`/`context_menu` free fns (over `Response` + `egui::Popup`).
- `focus.rs` — `focus_ring(ui, &response, corner_radius, color)` (a11y, v0.8): a 2 px ring drawn
  outside a focused widget. Used by `Button`/`Switch`/`Checkbox`/`Slider`.
- `data_grid.rs` — `DataGrid` (+ `SortState`/`SortDirection`), **feature `datagrid`** (optional
  `egui_extras` dep, ADR-0010). Virtualized body (`TableBuilder::body().rows`), resizable columns,
  clickable sortable headers that emit a caller-owned `SortState` (caller sorts its data). String
  cells. `DataGridRecipe` carries fixed header/row heights (needed for virtualization). Contrast
  with `table.rs` (always-available `Grid`, static/small data, no virtualization).
- `tree_view.rs` — `TreeView` + `TreeNode` (hierarchical, collapsible; single selection bound to
  `&mut Option<usize>` by node id). Branches wrap `egui::CollapsingHeader` (keyed by `id_salt`);
  leaves are `selectable_label`. `TreeViewRecipe` carries text style + indent; selection highlight
  uses egui's themed selection visuals.
- `number_input.rs` — `NumberInput` (`&mut f64`, range, `.step`): an `egui::DragValue` flanked by
  −/+ stepper buttons that move the value by `step` (clamped). Steppers are custom painter rects
  with `WidgetInfo::labeled(Button, "increment"/"decrement")` for a11y + testability.
  `NumberInputRecipe` styles the stepper buttons + gap (the DragValue uses egui visuals).
- `range_slider.rs` — `RangeSlider` (two `&mut f32` low/high + range): two-handle track. The handle
  nearest the pointer is grabbed and stored in `ctx.data` keyed by the response id so handles can't
  swap mid-drag; `low ≤ high` invariant enforced each frame. **Reuses `SliderRecipe`** (ADR-0009).
- `color_picker.rs` — `ColorPicker` (`&mut Color32`, `.with_alpha()`): a themed swatch (drawn from
  `ColorPickerRecipe`) that opens egui's own HSV picker (`color_picker::color_picker_color32`) in a
  `Popup::menu`. The deep picker module is reused as-is (not reimplemented); swatch carries
  `WidgetInfo::labeled(Button, "color picker")`.

## Accessibility (v0.8)
- Hit targets follow `UiContext::min_interactive_size()` (44 px in Touch — WCAG 2.5.5). Custom
  widgets allocate that height; `Button` passes it as `egui::Button::min_size`.
- Keyboard nav: `Sense::click` widgets focus + activate on Space/Enter for free; `Slider` also
  handles arrow keys when focused. Focus is made visible by `focus_ring` (+ the theme's global
  `widgets.active.bg_stroke` for stock egui widgets). Built-in palettes are WCAG-AA audited
  (`lumen_ui_core::a11y::audit_colors`).

v0.3 composed components reuse atomic widgets/recipes + read `theme.tokens()` for incidental
chrome rather than growing the frozen `Theme` trait per component.
