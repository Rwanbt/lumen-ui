# API freeze (v1.0 candidate)

As of v0.9 the public API below is **frozen**: it will not change in a breaking way before v1.0,
and from v1.0 it follows strict [SemVer](https://semver.org/). Additive changes (new widgets, new
recipe fields handled by `..Default`, new `WidgetState`/`Density` variants, new `UiContext` fields)
are explicitly *non-breaking* by design ([ADR-0002](adr/0002-recipes-parameterized-by-state.md)).

You depend on the **`lumen-ui` façade**, never the internal crates directly. The frozen surface is
what the façade re-exports per feature.

## `theme` feature (core surface)

- **Entry points**: `install`, `set_theme`
- **Traits**: `Theme`, `UiThemeExt`
- **Themes**: `DarkTheme`, `LightTheme`, `PaletteTheme` + `ThemeMode`
- **Tokens**: `Tokens`, `Colors`, `Spacing`, `Radius`, `Typography`, `Elevation`, `Motion`
- **Context**: `UiContext`, `Density`
- **Recipes**: `ButtonRecipe`/`ButtonVariant`, `TextRecipe`/`TextRole`, `CardRecipe`,
  `BadgeRecipe`/`BadgeVariant`, `ToggleRecipe`, `SliderRecipe`, `TextFieldRecipe`, `WidgetState`
- **Accessibility** (`a11y` module): `contrast_ratio`, `relative_luminance`, `meets`, `meets_aa`,
  `audit_colors`, `ContrastLevel`, `ContrastCheck`, `AuditReport`
- **Motion helper** (`anim` module): `lerp_color`

## `widgets` feature

`Button`, `Label`, `Heading`, `Card`, `Badge`, `Switch`, `Checkbox`, `RadioGroup`, `Slider`,
`TextField`, `Tabs`, `Accordion`, `Select`, `Modal` (+ `open_modal`/`close_modal`),
`ToastVariant` (+ `toast`/`toast_success`/`toast_warning`/`toast_error`/`show_toasts`),
`tooltip`/`popover`/`context_menu`.

## `layout` feature

`Flex`, `Grid`, `FlexUiExt`, `Justify`, `Align`, `Breakpoint`, `responsive`.

## `motion` feature

`Spring` (+ `SMOOTH`/`GENTLE`/`WOBBLY`/`STIFF`), `Easing`, `ease`, `fade`.

## `patterns` feature

`DashboardLayout`, `Toolbar`, `StatusBar`, `Sidebar`, `SettingsPage`, `InspectorPanel`,
`property_row`, `LogPanel`/`LogEntry`/`LogLevel`, `CommandPalette` (+ `open_command_palette`).

## `themes` feature

`audio_dark`, `high_contrast`.

## `icons` feature

`Icon`, `IconKind`.

## Stability rules

- **Frozen core** (`lumen-ui-core::theme`): any change to the `Theme` trait or a recipe struct's
  existing fields requires an ADR and a major version bump.
- **Recipe structs** are `#[non_exhaustive]`-equivalent in spirit: construct via the theme, read
  fields; new fields may be added (non-breaking) in minor versions.
- **egui version**: pinned to 0.34.x (workspace). A new egui minor is a lumen-ui minor at most,
  handled in the single adaptation layer ([ADR-0004](adr/0004-msrv-egui-pin.md)).
- **MSRV**: Rust 1.92, verified by a dedicated CI job. An MSRV bump is a minor version change.

## Not in the frozen surface

- Internal crates' non-re-exported items (treat anything not reachable through `lumen-ui` as private).
- The `material` feature (deferred — [ADR-0005](adr/0005-defer-material-adapter.md)).
- The `lumen-theme-gen` CLI (a dev tool, `publish = false`, not SemVer-bound).
