//! Functional widget tests via [`egui_kittest`] — headless, driven through the
//! AccessKit tree (the same surface a screen reader sees). No GPU/renderer needed.
//!
//! These verify *behavior* the type system can't: that widgets render without
//! panicking under every built-in theme, that clicks/typing/keys reach the bound
//! state, and that focus is exposed.

use std::sync::Arc;

use egui::accesskit::Role;
use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use lumen_core::{install, DarkTheme, LightTheme, Theme, UiContext};
use lumen_themes::{audio_dark, high_contrast};
use lumen_widgets::{Button, Checkbox, Slider, Switch, TextField};

/// Install a theme on the harness context (called every frame — idempotent).
fn theme_ctx(ctx: &egui::Context, theme: &Arc<dyn Theme>) {
    install(ctx, theme.clone(), UiContext::default());
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

        let mut harness = Harness::new_ui(move |ui| {
            theme_ctx(ui.ctx(), &theme);
            ui.add(Button::primary("Save"));
            ui.add(Button::danger("Delete").enabled(false));
            ui.add(TextField::new(&mut text));
            ui.add(Switch::new(&mut on));
            ui.add(Checkbox::new(&mut checked, "Accept"));
            ui.add(Slider::new(&mut value, 0.0..=1.0));
        });

        // A panic inside `run` fails the test and names the offending theme.
        harness.run();
        assert!(!name.is_empty());
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
