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
- `combobox.rs` — `Combobox<T>` searchable `Select`: a search `TextField` + `selectable_value` list
  filtered by case-insensitive substring, over `egui::ComboBox`. Uses `CloseOnClickOutside` (so
  clicking the search field doesn't dismiss it) and closes the popup manually after a pick via
  `Popup::close_id(response.id.with("popup"))` — replicates egui's private
  `ComboBox::widget_to_popup_id` (`= id.with("popup")`, verified vs 0.34.3; re-check on upgrade).
  Open state in egui memory; transient query in `ctx.data` keyed by id. No recipe (composition).
- `multi_select.rs` — `MultiSelect<T>` over `egui::ComboBox`: `selectable_label` per option toggling
  membership in the caller's `&mut Vec<T>` (insertion order kept). `CloseOnClickOutside` keeps the
  popup open across toggles. Trigger shows "N selected". No recipe (composition).
- `description_list.rs` — `DescriptionList` term/definition pairs (HTML `<dl>`) in an aligned
  two-column `egui::Grid`. Pure display; `DescriptionListRecipe` (term/definition colors+sizes, gap).
- `timeline.rs` — `Timeline` vertical chronological events: a dot per event linked by a connector
  line, title (+ optional detail) to the right. Single-pass: each connector is drawn from the
  previous dot to the current one (robust to row heights). `TimelineRecipe` (dot/line/text styles).
- `carousel.rs` — `Carousel` (`&mut usize` current index, `len`): one slide at a time via an
  `add_slide(ui, index)` closure, framed by prev/next arrows that **wrap** around the ends, with a
  dot indicator row below. Arrows are painter glyphs with `WidgetInfo::labeled(Button,
  "previous"/"next")` for a11y + tests. `CarouselRecipe` (arrow/dot styles).
- `calendar.rs` — `Calendar` (`&mut lumen_ui_core::Date`): a month grid (`egui::Grid`, 7 cols) that
  selects a day. The *displayed* month lives in `ctx.data` (paged by header ‹/› arrows) and is
  independent of the selection until a day is clicked. Uses the core `date` math (ADR-0011); day
  cells + arrows are painter-drawn with `WidgetInfo::labeled` (a11y + tests). `CalendarRecipe`.
- `date_picker.rs` — `DatePicker` (`id`, `&mut Date`): a `Button` labelled with the date that opens
  a `Calendar` in a `Popup::menu` (`CloseOnClickOutside`). Closes itself once the day changes
  (`Popup::close_id(id.with("popup"))`) so the calendar's month arrows keep it open. No recipe
  (composes Button + Calendar).
- `time_picker.rs` — `TimePicker` (`&mut Time`): two `egui::DragValue`s (hour `0..=23`, minute
  `0..=59`, zero-padded) separated by a colon. No recipe (egui visuals); returns the union response.
- `file_upload.rs` — `FileUpload` (stateless) → `FileUploadResponse { response, dropped }`: a themed
  drop zone reading egui's **native** `ctx.input().raw.dropped_files`/`hovered_files` (zero dep, no
  `rfd`; a "browse" dialog would need an optional `rfd` feature). Highlights while files hover.
  `FileUploadRecipe`. Works on desktop (path) and web (bytes).
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

## Generic DAW-style controls (reclassified from lumen-ui-audio, v1.1)
Nothing audio-specific about these, so they live here (the signal *displays* — VuMeter/LevelBar/
Waveform — stay in `lumen-ui-audio`). Each resolves a pure recipe in `lumen-ui-core`.
- `knob.rs` — `Knob` (`&mut f32`, range): 270° rotary; vertical drag changes the value; arc fills
  from min, pointer marks value. `KnobRecipe`. a11y `WidgetInfo::slider`.
- `fader.rs` — `Fader` (`&mut f32`, range): vertical fader (max at top), click/drag. Reuses
  `SliderRecipe`. a11y `WidgetInfo::slider`.
- `xy_pad.rs` — `XyPad` (`&mut x`, `&mut y`, ranges): square 2-D control, X→/Y↑, crosshair + point.
  `XyPadRecipe`.
- `transport.rs` — `Transport::new().playing(b).recording(b).show(ui) -> Option<TransportAction>`
  ({PlayPause, Stop, Record}): painter-drawn icons (triangle/bars/square/circle), record lit while
  recording. Buttons carry `WidgetInfo::labeled`. `TransportRecipe`.
