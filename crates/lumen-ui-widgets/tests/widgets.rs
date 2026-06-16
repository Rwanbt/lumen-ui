//! Functional widget tests via [`egui_kittest`] — headless, driven through the
//! AccessKit tree (the same surface a screen reader sees). No GPU/renderer needed.
//!
//! These verify *behavior* the type system can't: that widgets render without
//! panicking under every built-in theme, that clicks/typing/keys reach the bound
//! state, and that focus is exposed.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

use egui::accesskit::Role;
use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use lumen_ui_core::{install, DarkTheme, LightTheme, Theme, UiContext};
use lumen_ui_themes::{audio_dark, high_contrast};
use lumen_ui_widgets::{
    close_modal, open_modal, show_toasts, toast_success, Accordion, Alert, Avatar, Breadcrumb,
    Button, Checkbox, Chip, Divider, Kbd, Label, Modal, Pagination, Progress, RadioGroup,
    SegmentedControl, Select, Skeleton, Slider, Spinner, Stat, Switch, Tabs, TextField,
};

/// Install a theme on the harness context (called every frame — idempotent).
fn theme_ctx(ctx: &egui::Context, theme: &Arc<dyn Theme>) {
    install(ctx, theme.clone(), UiContext::default());
}

/// A dark-theme handle, the default for interaction tests where the theme is incidental.
fn dark() -> Arc<dyn Theme> {
    Arc::new(DarkTheme::new())
}

#[test]
fn every_widget_renders_under_all_built_in_themes() {
    let themes: [(&str, Arc<dyn Theme>); 4] = [
        ("dark", Arc::new(DarkTheme::new())),
        ("light", Arc::new(LightTheme::new())),
        ("audio_dark", Arc::new(audio_dark())),
        ("high_contrast", Arc::new(high_contrast())),
    ];

    for (name, theme) in themes {
        // Each frame mutates these; we only care that nothing panics.
        let mut text = String::from("hello");
        let mut on = true;
        let mut checked = false;
        let mut value = 0.5_f32;
        let mut segment = 0usize;

        let mut harness = Harness::new_ui(move |ui| {
            theme_ctx(ui.ctx(), &theme);
            ui.add(Button::primary("Save"));
            ui.add(Button::danger("Delete").enabled(false));
            ui.add(TextField::new(&mut text));
            ui.add(Switch::new(&mut on));
            ui.add(Checkbox::new(&mut checked, "Accept"));
            ui.add(Slider::new(&mut value, 0.0..=1.0));
            // Spinner is animated (requests repaint every frame) so it is covered by a
            // separate `step()` test below; `run()` would never converge with it present.
            ui.add(Progress::new(value));
            ui.add(Divider::horizontal());
            ui.add(Alert::warning("Heads up").title("Notice"));
            ui.add(Skeleton::new(120.0, 16.0));
            ui.add(Avatar::new("Ada Lovelace"));
            ui.add(Kbd::new("Ctrl"));
            Chip::new("tag").removable().show(ui);
            ui.add(Stat::new("Revenue", "$12.4k").delta("+8%", true));
            Breadcrumb::new()
                .item("Home")
                .item("Docs")
                .item("API")
                .show(ui);
            SegmentedControl::new(&mut segment)
                .segment("Day")
                .segment("Week")
                .show(ui);
            Pagination::new(1, 3).show(ui);
        });

        // A panic inside `run` fails the test and names the offending theme.
        harness.run();
        assert!(!name.is_empty());
    }
}

#[test]
fn spinner_renders_under_all_themes() {
    // The spinner animates (requests repaint each frame), so drive single frames
    // with `step()` rather than `run()`, which would never converge.
    let themes: [Arc<dyn Theme>; 4] = [
        Arc::new(DarkTheme::new()),
        Arc::new(LightTheme::new()),
        Arc::new(audio_dark()),
        Arc::new(high_contrast()),
    ];
    for theme in themes {
        let mut harness = Harness::new_ui(move |ui| {
            theme_ctx(ui.ctx(), &theme);
            ui.add(Spinner::new());
        });
        harness.step(); // a panic here fails the test
    }
}

#[test]
fn button_click_reaches_handler() {
    #[derive(Default)]
    struct State {
        clicks: u32,
    }

    let mut harness = Harness::new_ui_state(
        |ui, state: &mut State| {
            theme_ctx(ui.ctx(), &(Arc::new(DarkTheme::new()) as Arc<dyn Theme>));
            if ui.add(Button::primary("Save")).clicked() {
                state.clicks += 1;
            }
        },
        State::default(),
    );

    harness.run();
    harness.get_by_label("Save").click();
    harness.run();

    assert_eq!(harness.state().clicks, 1, "one click should register once");
}

#[test]
fn disabled_button_does_not_click() {
    #[derive(Default)]
    struct State {
        clicks: u32,
    }

    let mut harness = Harness::new_ui_state(
        |ui, state: &mut State| {
            theme_ctx(ui.ctx(), &(Arc::new(DarkTheme::new()) as Arc<dyn Theme>));
            if ui.add(Button::primary("Locked").enabled(false)).clicked() {
                state.clicks += 1;
            }
        },
        State::default(),
    );

    harness.run();
    // A disabled button is non-interactive; querying + clicking must not fire it.
    if let Some(node) = harness.query_by_label("Locked") {
        node.click();
    }
    harness.run();

    assert_eq!(harness.state().clicks, 0);
}

#[test]
fn text_field_accepts_typed_input() {
    let mut harness = Harness::new_ui_state(
        |ui, text: &mut String| {
            theme_ctx(ui.ctx(), &(Arc::new(DarkTheme::new()) as Arc<dyn Theme>));
            ui.add(TextField::new(text));
        },
        String::new(),
    );

    harness.run();
    let field = harness.get_by_role(Role::TextInput);
    field.focus();
    field.type_text("lumen");
    harness.run();

    assert_eq!(harness.state().as_str(), "lumen");
}

#[test]
fn switch_toggles_on_click() {
    let mut harness = Harness::new_ui_state(
        |ui, on: &mut bool| {
            theme_ctx(ui.ctx(), &(Arc::new(DarkTheme::new()) as Arc<dyn Theme>));
            ui.add(Switch::new(on));
        },
        false,
    );

    harness.run();
    assert!(!*harness.state());
    harness.get_by_role(Role::CheckBox).click();
    harness.run();
    assert!(*harness.state(), "clicking the switch flips the bound bool");
}

#[test]
fn checkbox_toggles_by_label() {
    let mut harness = Harness::new_ui_state(
        |ui, checked: &mut bool| {
            theme_ctx(ui.ctx(), &(Arc::new(DarkTheme::new()) as Arc<dyn Theme>));
            ui.add(Checkbox::new(checked, "Accept terms"));
        },
        false,
    );

    harness.run();
    // The label is carried by both the checkbox node and the trailing Label widget, so
    // disambiguate by role: this also asserts the checkbox itself exposes its label to a11y.
    harness
        .get_by_role_and_label(Role::CheckBox, "Accept terms")
        .click();
    harness.run();
    assert!(*harness.state());
}

#[test]
fn slider_responds_to_arrow_keys_when_focused() {
    let mut harness = Harness::new_ui_state(
        |ui, value: &mut f32| {
            theme_ctx(ui.ctx(), &(Arc::new(DarkTheme::new()) as Arc<dyn Theme>));
            ui.add(Slider::new(value, 0.0..=1.0));
        },
        0.5_f32,
    );

    harness.run();
    harness.get_by_role(Role::Slider).focus();
    harness.run();
    harness.key_press(egui::Key::ArrowRight);
    harness.run();

    assert!(
        *harness.state() > 0.5,
        "ArrowRight should increase a focused slider (got {})",
        harness.state()
    );
}

// ---------------------------------------------------------------------------
// Composed components — their open/visible/selection state lives in `ctx.data`,
// driven here through the a11y tree + egui memory exactly as an app would.
// (v1.0 behavioral coverage pass.)
// ---------------------------------------------------------------------------

#[test]
fn tabs_switch_active_index_on_click() {
    let mut harness = Harness::new_ui_state(
        |ui, active: &mut usize| {
            theme_ctx(ui.ctx(), &dark());
            *active = Tabs::new("main-tabs").tab("Files").tab("Search").show(ui);
        },
        0usize,
    );

    harness.run();
    assert_eq!(*harness.state(), 0, "first tab is active by default");
    harness.get_by_label("Search").click();
    harness.run();
    assert_eq!(*harness.state(), 1, "clicking a tab selects it");
}

#[test]
fn accordion_reveals_body_when_expanded() {
    let mut harness = Harness::new_ui(|ui| {
        theme_ctx(ui.ctx(), &dark());
        Accordion::new("Advanced").show(ui, |ui| {
            ui.add(Label::new("Hidden body"));
        });
    });

    harness.run();
    assert!(
        harness.query_by_label("Hidden body").is_none(),
        "a collapsed accordion hides its body"
    );
    harness.get_by_label("Advanced").click();
    harness.run();
    assert!(
        harness.query_by_label("Hidden body").is_some(),
        "an expanded accordion reveals its body"
    );
}

#[test]
fn radio_group_selects_clicked_option() {
    #[derive(PartialEq, Clone)]
    enum Mode {
        Alpha,
        Beta,
    }

    let mut harness = Harness::new_ui_state(
        |ui, selected: &mut Mode| {
            theme_ctx(ui.ctx(), &dark());
            ui.add(
                RadioGroup::new(selected)
                    .option(Mode::Alpha, "Alpha")
                    .option(Mode::Beta, "Beta"),
            );
        },
        Mode::Alpha,
    );

    harness.run();
    assert!(*harness.state() == Mode::Alpha);
    // The label is on both the radio node and a trailing Label, so disambiguate by role.
    harness
        .get_by_role_and_label(Role::RadioButton, "Beta")
        .click();
    harness.run();
    assert!(
        *harness.state() == Mode::Beta,
        "clicking a radio row selects that option"
    );
}

#[test]
fn select_rebinds_value_on_pick() {
    #[derive(PartialEq, Clone)]
    enum Mode {
        Alpha,
        Beta,
    }

    let mut harness = Harness::new_ui_state(
        |ui, selected: &mut Mode| {
            theme_ctx(ui.ctx(), &dark());
            Select::new("mode-select", selected)
                .option(Mode::Alpha, "Alpha")
                .option(Mode::Beta, "Beta")
                .show(ui);
        },
        Mode::Alpha,
    );

    harness.run();
    // The combo exposes its selection as the ComboBox node's *value* ("Alpha"), not a
    // label — open it by role. "Beta" then exists only in the popup, unambiguous to click.
    harness.get_by_role(Role::ComboBox).click();
    harness.run();
    harness.get_by_label("Beta").click();
    harness.run();
    assert!(
        *harness.state() == Mode::Beta,
        "picking a dropdown option rebinds the value"
    );
}

#[test]
fn modal_shows_when_open_and_hides_when_closed() {
    // 0 = request open, 1 = request close, other = idle.
    let phase = Arc::new(AtomicU32::new(0));
    let phase_c = phase.clone();

    let mut harness = Harness::new_ui(move |ui| {
        theme_ctx(ui.ctx(), &dark());
        let ctx = ui.ctx().clone();
        match phase_c.load(Ordering::Relaxed) {
            0 => open_modal(&ctx, "confirm"),
            1 => close_modal(&ctx, "confirm"),
            _ => {}
        }
        Modal::new("confirm").title("Confirm").show(&ctx, |ui| {
            ui.add(Label::new("Modal body"));
        });
    });

    harness.run();
    assert!(
        harness.query_by_label("Modal body").is_some(),
        "an opened modal renders its body"
    );

    phase.store(1, Ordering::Relaxed);
    harness.run();
    assert!(
        harness.query_by_label("Modal body").is_none(),
        "close_modal hides the body"
    );
}

#[test]
fn modal_closes_on_escape() {
    // 0 = open once, anything else = idle (stop re-opening so Esc can stick).
    let phase = Arc::new(AtomicU32::new(0));
    let phase_c = phase.clone();

    let mut harness = Harness::new_ui(move |ui| {
        theme_ctx(ui.ctx(), &dark());
        let ctx = ui.ctx().clone();
        if phase_c.load(Ordering::Relaxed) == 0 {
            open_modal(&ctx, "confirm");
        }
        Modal::new("confirm").show(&ctx, |ui| {
            ui.add(Label::new("Esc body"));
        });
    });

    harness.run();
    assert!(harness.query_by_label("Esc body").is_some());

    phase.store(2, Ordering::Relaxed); // stop re-opening
    harness.key_press(egui::Key::Escape);
    harness.run(); // this frame still renders, then egui::Modal sets should_close
    harness.run(); // next frame: open == false → body gone
    assert!(
        harness.query_by_label("Esc body").is_none(),
        "Esc closes the modal (egui::Modal integration)"
    );
}

#[test]
fn toast_renders_pushed_message() {
    let pushed = Arc::new(AtomicBool::new(false));
    let pushed_c = pushed.clone();

    let mut harness = Harness::new_ui(move |ui| {
        theme_ctx(ui.ctx(), &dark());
        let ctx = ui.ctx().clone();
        if !pushed_c.swap(true, Ordering::Relaxed) {
            toast_success(&ctx, "Saved");
        }
        show_toasts(&ctx);
    });

    // `show_toasts` keeps requesting repaints (to expire toasts on time), so the
    // convergence loop in `run()` never settles — drive a single frame with `step()`.
    harness.step();
    assert!(
        harness.query_by_label("Saved").is_some(),
        "a pushed toast renders its message"
    );
}
