# AI_SUMMARY — src

> **Auto-generated 2026-06-15 06:38** — do not edit manually.
> Source: `tools/ai_docs/generate_ai_summary.py`
> For purpose, thread model and constraints, read `AI_CONTEXT.md`.

## Purpose
Ready-made application patterns composing `lumen-widgets` + egui panels into common app
structures, so wiring a new app is a few lines. Enabled via the façade `patterns` feature.
Depends on `egui` + `lumen-core` + `lumen-widgets`.

## Files & LOC
| File | LOC | |
|------|-----|--|
| `bars.rs` | 31 | |
| `command_palette.rs` | 70 | |
| `dashboard.rs` | 65 | |
| `lib.rs` | 17 | |
| `logpanel.rs` | 64 | |
| `rows.rs` | 49 | |
| `sidebar.rs` | 36 | |
| **Total** | **332** | |

## Rust API
- `CommandPalette` (struct)
- `DashboardLayout` (struct)
- `InspectorPanel` (struct)
- `LogEntry` (struct)
- `LogPanel` (struct)
- `SettingsPage` (struct)
- `Sidebar` (struct)
- `StatusBar` (struct)
- `Toolbar` (struct)
- `LogLevel` (enum)

## Rust Functions
- `open_command_palette()`
- `property_row()`
