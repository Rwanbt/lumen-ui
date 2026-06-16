//! Behavioral tests for the app-shell patterns via [`egui_kittest`] — headless,
//! driven through the AccessKit tree. These assert the patterns actually render
//! their regions and route interaction to bound state (not just that they compile).

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use lumen_ui_core::{install, DarkTheme, Theme, UiContext};
use lumen_ui_patterns::{
    open_command_palette, AuthCard, CommandPalette, DashboardLayout, Form, InspectorPanel,
    LogEntry, LogPanel, MasterDetail, SettingsPage, Sidebar, StatusBar, Toolbar, Wizard,
};
use lumen_ui_widgets::{Button, Label};

/// Install a theme on the harness context (called every frame — idempotent).
fn theme_ctx(ctx: &egui::Context, theme: &Arc<dyn Theme>) {
    install(ctx, theme.clone(), UiContext::default());
}

/// A dark-theme handle — the theme is incidental to these pattern behaviors.
fn dark() -> Arc<dyn Theme> {
    Arc::new(DarkTheme::new())
}

#[test]
fn sidebar_selects_item_on_click() {
    let mut harness = Harness::new_ui_state(
        |ui, selected: &mut usize| {
            theme_ctx(ui.ctx(), &dark());
            Sidebar::new(selected)
                .item("Home")
                .item("Settings")
                .show(ui);
        },
        0usize,
    );

    harness.run();
    assert_eq!(*harness.state(), 0, "first entry is selected by default");
    harness.get_by_label("Settings").click();
    harness.run();
    assert_eq!(*harness.state(), 1, "clicking a sidebar entry selects it");
}

#[test]
fn command_palette_returns_chosen_command() {
    // 0 = open once, anything else = idle (stop re-opening so the choice can close it).
    let phase = Arc::new(AtomicU32::new(0));
    let phase_c = phase.clone();

    let mut harness = Harness::new_ui_state(
        move |ui, chosen: &mut Option<usize>| {
            theme_ctx(ui.ctx(), &dark());
            let ctx = ui.ctx().clone();
            if phase_c.load(Ordering::Relaxed) == 0 {
                open_command_palette(&ctx, "cmdk");
            }
            if let Some(index) = CommandPalette::new("cmdk")
                .command("Open File")
                .command("Save")
                .show(&ctx)
            {
                *chosen = Some(index);
            }
        },
        None,
    );

    harness.run();
    assert!(
        harness.query_by_label("Save").is_some(),
        "an open palette lists its commands"
    );

    phase.store(1, Ordering::Relaxed); // stop re-opening
    harness.get_by_label("Save").click();
    harness.run();
    assert_eq!(
        *harness.state(),
        Some(1),
        "choosing a command returns its index and closes the palette"
    );
}

#[test]
fn log_panel_renders_entries_with_badges() {
    let mut harness = Harness::new_ui(|ui| {
        theme_ctx(ui.ctx(), &dark());
        let entries = [
            LogEntry::info("service started"),
            LogEntry::error("connection lost"),
        ];
        LogPanel::new().show(ui, &entries);
    });

    harness.run();
    assert!(harness.query_by_label("service started").is_some());
    assert!(harness.query_by_label("connection lost").is_some());
    assert!(
        harness.query_by_label("ERROR").is_some(),
        "error lines carry an ERROR severity badge"
    );
}

#[test]
fn dashboard_renders_all_regions() {
    let mut harness = Harness::new_ui(|ui| {
        theme_ctx(ui.ctx(), &dark());
        DashboardLayout::new()
            .toolbar(|ui| {
                ui.add(Label::new("ToolbarRegion"));
            })
            .sidebar(|ui| {
                ui.add(Label::new("SidebarRegion"));
            })
            .inspector(|ui| {
                ui.add(Label::new("InspectorRegion"));
            })
            .status_bar(|ui| {
                ui.add(Label::new("StatusRegion"));
            })
            .show(ui, |ui| {
                ui.add(Label::new("MainRegion"));
            });
    });

    harness.run();
    for region in [
        "ToolbarRegion",
        "SidebarRegion",
        "InspectorRegion",
        "StatusRegion",
        "MainRegion",
    ] {
        assert!(
            harness.query_by_label(region).is_some(),
            "the dashboard renders the {region} region"
        );
    }
}

#[test]
fn settings_inspector_and_property_rows_render() {
    let mut harness = Harness::new_ui(|ui| {
        theme_ctx(ui.ctx(), &dark());
        SettingsPage::new("Preferences").show(ui, |ui| {
            lumen_ui_patterns::property_row(ui, "Volume", |ui| {
                ui.add(Label::new("VolumeControl"));
            });
        });
        InspectorPanel::new("Inspector").show(ui, |ui| {
            lumen_ui_patterns::property_row(ui, "Opacity", |ui| {
                ui.add(Label::new("OpacityControl"));
            });
        });
    });

    harness.run();
    for label in [
        "Preferences",
        "Volume",
        "VolumeControl",
        "Inspector",
        "Opacity",
        "OpacityControl",
    ] {
        assert!(
            harness.query_by_label(label).is_some(),
            "expected `{label}` in the rendered settings/inspector rows"
        );
    }
}

#[test]
fn toolbar_and_status_bar_render_items() {
    let mut harness = Harness::new_ui(|ui| {
        theme_ctx(ui.ctx(), &dark());
        Toolbar::new().show(ui, |ui| {
            ui.add(Button::ghost("File"));
        });
        StatusBar::new().show(ui, |ui| {
            ui.add(Label::new("Ready"));
        });
    });

    harness.run();
    assert!(harness.query_by_label("File").is_some());
    assert!(harness.query_by_label("Ready").is_some());
}

#[test]
fn form_routes_action_click() {
    let mut harness = Harness::new_ui_state(
        |ui, submitted: &mut bool| {
            theme_ctx(ui.ctx(), &dark());
            Form::new()
                .field(|ui| {
                    ui.add(Label::new("EmailField"));
                })
                .actions(|ui| {
                    if ui.add(Button::primary("Submit")).clicked() {
                        *submitted = true;
                    }
                })
                .show(ui);
        },
        false,
    );

    harness.run();
    assert!(
        harness.query_by_label("EmailField").is_some(),
        "the form renders its field rows"
    );
    assert!(!*harness.state(), "no action fired before a click");
    harness.get_by_label("Submit").click();
    harness.run();
    assert!(
        *harness.state(),
        "clicking a form action routes to bound state"
    );
}

#[derive(Default)]
struct AuthState {
    email: String,
    password: String,
    remember: bool,
    submitted: bool,
}

#[test]
fn auth_card_renders_fields_and_submits() {
    let mut harness = Harness::new_ui_state(
        |ui, state: &mut AuthState| {
            theme_ctx(ui.ctx(), &dark());
            let response = AuthCard::new("Welcome back", &mut state.email, &mut state.password)
                .remember(&mut state.remember)
                .secondary_link("Forgot password?")
                .show(ui);
            if response.submitted {
                state.submitted = true;
            }
        },
        AuthState::default(),
    );

    harness.run();
    for label in [
        "Welcome back",
        "Email",
        "Password",
        "Remember me",
        "Sign in",
        "Forgot password?",
    ] {
        assert!(
            // `query_all`: a Checkbox exposes both a role node and a text run for its label.
            harness.query_all_by_label(label).next().is_some(),
            "the auth card renders `{label}`"
        );
    }
    assert!(!harness.state().submitted, "no submit before a click");
    harness.get_by_label("Sign in").click();
    harness.run();
    assert!(
        harness.state().submitted,
        "clicking submit reports it on the response"
    );
}

#[test]
fn master_detail_selects_and_updates_detail() {
    let mut harness = Harness::new_ui_state(
        |ui, selected: &mut usize| {
            theme_ctx(ui.ctx(), &dark());
            MasterDetail::new(selected)
                .item("Inbox")
                .item("Sent")
                .show(ui, |ui, index| {
                    ui.add(Label::new(format!("Detail-{index}")));
                });
        },
        0usize,
    );

    harness.run();
    assert_eq!(*harness.state(), 0, "first entry is selected by default");
    assert!(
        harness.query_by_label("Detail-0").is_some(),
        "the detail pane reflects the default selection"
    );
    harness.get_by_label("Sent").click();
    harness.run();
    assert_eq!(*harness.state(), 1, "clicking a list entry selects it");
    assert!(
        harness.query_by_label("Detail-1").is_some(),
        "the detail pane follows the selection"
    );
}

#[test]
fn wizard_advances_and_finishes() {
    // 0 = first step. Wizard advances on Next and reports `finished` on the last step.
    let mut harness = Harness::new_ui_state(
        |ui, state: &mut (usize, bool)| {
            theme_ctx(ui.ctx(), &dark());
            let response =
                Wizard::new(&mut state.0)
                    .step("Account")
                    .step("Done")
                    .show(ui, |ui, index| {
                        ui.add(Label::new(format!("Body-{index}")));
                    });
            if response.finished {
                state.1 = true;
            }
        },
        (0usize, false),
    );

    harness.run();
    assert_eq!(harness.state().0, 0, "starts on the first step");
    assert!(
        harness.query_by_label("Body-0").is_some(),
        "the body reflects the active step"
    );
    harness.get_by_label("Next").click();
    harness.run();
    assert_eq!(harness.state().0, 1, "Next advances to the following step");
    assert!(
        !harness.state().1,
        "the wizard is not finished before the last step's Finish"
    );

    // On the last step, the primary action is Finish, not Next.
    harness.get_by_label("Finish").click();
    harness.run();
    assert!(
        harness.state().1,
        "clicking Finish on the last step reports finished"
    );
}
