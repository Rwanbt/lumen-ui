# AI_CONTEXT — lumen-ui-icons

## Purpose
A small set of crisp vector icons drawn with the egui painter (no font asset to ship). Each
[`Icon`] is an egui `Widget`. Enabled via the façade `icons` feature. Depends on `egui` +
`lumen-ui-core` (for the default theme color).

## Constraints
- An `Icon` sizes to `interact_size.y` by default and uses the theme's text color (requires a
  theme installed via `lumen_ui::install`). Override with `.size(..)` / `.color(..)`.
- Glyphs are line segments / a circle in a normalized `[0,1]²` box scaled to the allocated rect;
  stroke width scales with size. Keep new glyphs in the same normalized style.

## Forbidden
- `#![forbid(unsafe_code)]`. No font/asset loading — painter primitives only.

## Common patterns
```ignore
ui.add(lumen_ui_icons::Icon::search());
ui.add(lumen_ui_icons::Icon::close().size(20.0).color(egui::Color32::RED));
```

## Modules
- `lib.rs` — `IconKind` (Check/Close/ChevronDown/ChevronRight/Plus/Minus/Search/Menu) + `Icon`
  widget (constructors `check()`, `close()`, … ; `size`/`color`).
