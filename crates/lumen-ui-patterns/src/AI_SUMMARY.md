# AI_SUMMARY — src

> **Auto-generated 2026-06-18 00:09** — do not edit manually.
> Source: `tools/ai_docs/generate_ai_summary.py`
> For purpose, thread model and constraints, read `AI_CONTEXT.md`.

## Purpose
Ready-made application patterns composing `lumen-ui-widgets` + egui panels into common app
structures, so wiring a new app is a few lines. Enabled via the façade `patterns` feature.
Depends on `egui` + `lumen-ui-core` + `lumen-ui-widgets`.

## Files & LOC
| File | LOC | |
|------|-----|--|
| `auth_card.rs` | 103 | |
| `bars.rs` | 31 | |
| `command_palette.rs` | 70 | |
| `dashboard.rs` | 65 | |
| `data_table.rs` | 152 | |
| `form.rs` | 54 | |
| `lib.rs` | 27 | |
| `logpanel.rs` | 64 | |
| `master_detail.rs` | 71 | |
| `rows.rs` | 49 | |
| `sidebar.rs` | 36 | |
| `wizard.rs` | 103 | |
| **Total** | **825** | |

## Rust API
- `AuthCard` (struct)
- `AuthCardResponse` (struct)
- `CommandPalette` (struct)
- `DashboardLayout` (struct)
- `DataTable` (struct)
- `DataTableState` (struct)
- `Form` (struct)
- `InspectorPanel` (struct)
- `LogEntry` (struct)
- `LogPanel` (struct)
- `MasterDetail` (struct)
- `SettingsPage` (struct)
- `Sidebar` (struct)
- `StatusBar` (struct)
- `Toolbar` (struct)
- `Wizard` (struct)
- `WizardResponse` (struct)
- `LogLevel` (enum)

## Rust Functions
- `open_command_palette()`
- `property_row()`
