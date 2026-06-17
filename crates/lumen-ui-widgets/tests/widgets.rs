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
use lumen_ui_core::{install, DarkTheme, Date, LightTheme, Theme, UiContext};
use lumen_ui_themes::{audio_dark, high_contrast};
use lumen_ui_widgets::{
    close_modal, hover_card, open_modal, show_toasts, toast_success, Accordion, Alert, Avatar,
    Breadcrumb, Button, Calendar, Carousel, Checkbox, Chip, CircularProgress, Code, ColorPicker,
    Combobox, DescriptionList, Divider, DropdownMenu, EmptyState, FormField, IconButton, Kbd,
    Label, Link, Modal, MultiSelect, NumberInput, Pagination, Progress, RadioGroup, RangeSlider,
    Rating, SegmentedControl, Select, Skeleton, Slider, Spinner, Stat, Stepper, Switch, Table,
    Tabs, TextField, Textarea, Timeline, TreeNode, TreeView,
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
        let mut stars = 3u32;
        let mut notes = String::from("multi\nline");
        let mut tree_selected: Option<usize> = Some(1);
        let mut amount = 5.0_f64;
        let mut span = (0.2_f32, 0.8_f32);
        let mut color = egui::Color32::from_rgb(120, 80, 200);
        let mut lang = 1usize;
        let mut tags: Vec<usize> = vec![1];
        let mut slide = 0usize;
        let mut date = Date::new(2026, 6, 17);

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
            ui.add(EmptyState::new("No results").message("Try another filter"));
            ui.add(Link::new("Learn more"));
            ui.add(CircularProgress::new(value));
            Rating::new(&mut stars).show(ui);
            ui.add(Code::new("cargo build"));
            Stepper::new(1)
                .step("Account")
                .step("Profile")
                .step("Done")
                .show(ui);
            Table::new("demo-table")
                .column("Name")
                .column("Role")
                .row(["Ada", "Engineer"])
                .row(["Linus", "Maintainer"])
                .show(ui);
            ui.add(IconButton::new(Label::new("+")));
            let menu_trigger = ui.add(Button::secondary("Menu"));
            DropdownMenu::new()
                .item("New")
                .item("Open")
                .show(&menu_trigger);
            let info = ui.add(Label::new("info"));
            hover_card(info, |ui| {
                ui.add(Label::new("details"));
            });
            ui.add(Textarea::new(&mut notes).hint("Notes"));
            ui.add(NumberInput::new(&mut amount, 0.0..=10.0));
            ui.add(RangeSlider::new(&mut span.0, &mut span.1, 0.0..=1.0));
            ui.add(ColorPicker::new(&mut color));
            Combobox::new("lang", &mut lang)
                .option(0usize, "Rust")
                .option(1usize, "Go")
                .show(ui);
            MultiSelect::new("tags", &mut tags)
                .option(0usize, "bug")
                .option(1usize, "feat")
                .show(ui);
            DescriptionList::new("dl")
                .item("Status", "Active")
                .item("Owner", "Ada")
                .show(ui);
            Timeline::new()
                .event("Created")
                .event_detailed("Shipped", "v1.0")
                .show(ui);
            Carousel::new("car", &mut slide, 3).show(ui, |ui, index| {
                ui.add(Label::new(format!("Slide {index}")));
            });
            Calendar::new("cal", &mut date).show(ui);
            FormField::new("Email")
                .hint("We'll never share it")
                .show(ui, |ui| {
                    ui.add(Label::new("control"));
                });
            TreeView::new(&mut tree_selected)
                .node(
                    TreeNode::branch(
                        0,
                        "src",
                        vec![TreeNode::leaf(1, "lib.rs"), TreeNode::leaf(2, "main.rs")],
                    )
                    .default_open(true),
                )
                .show(ui);
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

#[test]
fn number_input_increments_on_stepper_click() {
    let mut harness = Harness::new_ui_state(
        |ui, value: &mut f64| {
            theme_ctx(ui.ctx(), &dark());
            ui.add(NumberInput::new(value, 0.0..=10.0).step(1.0));
        },
        5.0_f64,
    );

    harness.run();
    harness.get_by_label("increment").click();
    harness.run();

    assert_eq!(
        *harness.state(),
        6.0,
        "the + stepper adds one step to the bound value"
    );
}

#[test]
fn range_slider_keeps_handles_ordered() {
    #[derive(Default)]
    struct State {
        low: f32,
        high: f32,
    }

    // Start with the handles inverted; the widget must restore low ≤ high on render.
    let mut harness = Harness::new_ui_state(
        |ui, s: &mut State| {
            theme_ctx(ui.ctx(), &dark());
            ui.add(RangeSlider::new(&mut s.low, &mut s.high, 0.0..=1.0));
        },
        State {
            low: 0.8,
            high: 0.2,
        },
    );

    harness.run();
    let s = harness.state();
    assert!(
        s.low <= s.high,
        "RangeSlider keeps low ≤ high (got {}..{})",
        s.low,
        s.high
    );
}

#[test]
fn color_picker_opens_picker_on_click() {
    let mut harness = Harness::new_ui_state(
        |ui, color: &mut egui::Color32| {
            theme_ctx(ui.ctx(), &dark());
            ui.add(ColorPicker::new(color));
        },
        egui::Color32::from_rgb(200, 60, 60),
    );

    harness.run();
    // The RGB DragValues (Role::SpinButton) only exist once the popup is open. There are
    // several (R/G/B), so use `query_all_*` — `query_by_role` panics on multiple matches.
    assert!(
        harness.query_all_by_role(Role::SpinButton).next().is_none(),
        "picker popup is closed initially"
    );
    harness.get_by_label("color picker").click();
    harness.run();
    assert!(
        harness.query_all_by_role(Role::SpinButton).next().is_some(),
        "clicking the swatch opens egui's RGB picker"
    );
}

#[test]
fn combobox_filters_then_selects() {
    let mut harness = Harness::new_ui_state(
        |ui, selected: &mut usize| {
            theme_ctx(ui.ctx(), &dark());
            Combobox::new("lang", selected)
                .option(0usize, "Rust")
                .option(1usize, "Ruby")
                .option(2usize, "Go")
                .show(ui);
        },
        0usize,
    );

    harness.run();
    // Open by role (the trigger exposes the selection as its value, like Select).
    harness.get_by_role(Role::ComboBox).click();
    harness.run();
    // Type into the search field to filter down to "Go", then pick it.
    let field = harness.get_by_role(Role::TextInput);
    field.focus();
    field.type_text("Go");
    harness.run();
    harness.get_by_label("Go").click();
    harness.run();

    assert_eq!(
        *harness.state(),
        2,
        "picking the filtered option rebinds the value"
    );
    // The popup is closed again after a pick (CloseOnClickOutside + manual close).
    assert!(
        harness.query_by_role(Role::TextInput).is_none(),
        "combobox closes after selecting a value"
    );
}

#[test]
fn multi_select_toggles_membership() {
    let mut harness = Harness::new_ui_state(
        |ui, selected: &mut Vec<usize>| {
            theme_ctx(ui.ctx(), &dark());
            MultiSelect::new("tags", selected)
                .option(0usize, "bug")
                .option(1usize, "feat")
                .show(ui);
        },
        Vec::new(),
    );

    harness.run();
    harness.get_by_role(Role::ComboBox).click();
    harness.run();
    // The popup stays open across toggles (CloseOnClickOutside): add both, then remove one.
    harness.get_by_label("bug").click();
    harness.run();
    harness.get_by_label("feat").click();
    harness.run();
    assert_eq!(
        *harness.state(),
        vec![0, 1],
        "toggling adds both values, order preserved"
    );
    harness.get_by_label("bug").click();
    harness.run();
    assert_eq!(
        *harness.state(),
        vec![1],
        "toggling an active option removes it"
    );
}

#[test]
fn carousel_navigates_with_wraparound() {
    let mut harness = Harness::new_ui_state(
        |ui, index: &mut usize| {
            theme_ctx(ui.ctx(), &dark());
            Carousel::new("car", index, 3).show(ui, |ui, i| {
                ui.add(Label::new(format!("slide {i}")));
            });
        },
        0usize,
    );

    harness.run();
    // "previous" from the first slide wraps to the last.
    harness.get_by_label("previous").click();
    harness.run();
    assert_eq!(*harness.state(), 2, "previous wraps 0 -> last");
    // "next" from the last slide wraps back to the first.
    harness.get_by_label("next").click();
    harness.run();
    assert_eq!(*harness.state(), 0, "next wraps last -> 0");
}

#[test]
fn calendar_selects_clicked_day() {
    let mut harness = Harness::new_ui_state(
        |ui, date: &mut Date| {
            theme_ctx(ui.ctx(), &dark());
            Calendar::new("cal", date).show(ui);
        },
        Date::new(2026, 6, 17),
    );

    harness.run();
    // Day cells are labelled by their number; click the 10th of the displayed month (June 2026).
    harness.get_by_label("10").click();
    harness.run();
    assert_eq!(*harness.state(), Date::new(2026, 6, 10));
}

#[test]
fn calendar_pages_month_without_changing_selection() {
    let mut harness = Harness::new_ui_state(
        |ui, date: &mut Date| {
            theme_ctx(ui.ctx(), &dark());
            Calendar::new("cal", date).show(ui);
        },
        Date::new(2026, 6, 17),
    );

    harness.run();
    harness.get_by_label("next month").click();
    harness.run();
    // Paging the view does not move the selection on its own.
    assert_eq!(*harness.state(), Date::new(2026, 6, 17));
    // June has 30 days; "31" exists only now that July (31 days) is displayed.
    harness.get_by_label("31").click();
    harness.run();
    assert_eq!(*harness.state(), Date::new(2026, 7, 31));
}

#[test]
fn description_list_exposes_terms_and_definitions() {
    let mut harness = Harness::new_ui(|ui| {
        theme_ctx(ui.ctx(), &dark());
        DescriptionList::new("info")
            .item("Status", "Active")
            .item("Owner", "Ada")
            .show(ui);
    });

    harness.run();
    // Both terms and their definitions reach the a11y tree as labelled text.
    for label in ["Status", "Active", "Owner", "Ada"] {
        assert!(
            harness.query_by_label(label).is_some(),
            "description list should expose {label:?}"
        );
    }
}

#[test]
fn timeline_exposes_event_titles_and_details() {
    let mut harness = Harness::new_ui(|ui| {
        theme_ctx(ui.ctx(), &dark());
        Timeline::new()
            .event("Created")
            .event_detailed("Shipped", "v1.0")
            .show(ui);
    });

    harness.run();
    for label in ["Created", "Shipped", "v1.0"] {
        assert!(
            harness.query_by_label(label).is_some(),
            "timeline should expose {label:?}"
        );
    }
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

// ---------------------------------------------------------------------------
// Behavioral coverage for the v2 interactive widgets — assert the return value
// (clicked index / mutated state), not just that they render (added by /review).
// ---------------------------------------------------------------------------

#[test]
fn segmented_control_selects_clicked_segment() {
    let mut harness = Harness::new_ui_state(
        |ui, selected: &mut usize| {
            theme_ctx(ui.ctx(), &dark());
            SegmentedControl::new(selected)
                .segment("Day")
                .segment("Week")
                .show(ui);
        },
        0usize,
    );

    harness.run();
    assert_eq!(*harness.state(), 0, "first segment selected by default");
    harness.get_by_label("Week").click();
    harness.run();
    assert_eq!(*harness.state(), 1, "clicking a segment selects it");
}

#[test]
fn breadcrumb_returns_clicked_ancestor() {
    let mut harness = Harness::new_ui_state(
        |ui, clicked: &mut Option<usize>| {
            theme_ctx(ui.ctx(), &dark());
            if let Some(index) = Breadcrumb::new()
                .item("Home")
                .item("Docs")
                .item("API")
                .show(ui)
            {
                *clicked = Some(index);
            }
        },
        None::<usize>,
    );

    harness.run();
    harness.get_by_label("Home").click();
    harness.run();
    assert_eq!(
        *harness.state(),
        Some(0),
        "clicking an ancestor returns its index"
    );
}

#[test]
fn pagination_requests_clicked_page() {
    let mut harness = Harness::new_ui_state(
        |ui, requested: &mut Option<usize>| {
            theme_ctx(ui.ctx(), &dark());
            // current = 1 (0-based) → page label "1" is the clickable page 0.
            if let Some(page) = Pagination::new(1, 3).show(ui) {
                *requested = Some(page);
            }
        },
        None::<usize>,
    );

    harness.run();
    harness.get_by_label("1").click();
    harness.run();
    assert_eq!(
        *harness.state(),
        Some(0),
        "clicking page '1' requests page index 0"
    );
}

#[test]
fn chip_reports_removed_on_x_click() {
    let mut harness = Harness::new_ui_state(
        |ui, removed: &mut bool| {
            theme_ctx(ui.ctx(), &dark());
            if Chip::new("tag").removable().show(ui).removed {
                *removed = true;
            }
        },
        false,
    );

    harness.run();
    harness.get_by_label("\u{00d7}").click(); // the × remove affordance
    harness.run();
    assert!(*harness.state(), "clicking × reports removed");
}

#[test]
fn rating_sets_value_to_clicked_star() {
    let mut harness = Harness::new_ui_state(
        |ui, stars: &mut u32| {
            theme_ctx(ui.ctx(), &dark());
            Rating::new(stars).show(ui);
        },
        3u32,
    );

    harness.run();
    // value=3 → stars 1..=3 are filled "★"; click the first filled star → value 1.
    harness
        .get_all_by_label("\u{2605}")
        .next()
        .expect("a filled star")
        .click();
    harness.run();
    assert_eq!(
        *harness.state(),
        1,
        "clicking the n-th star sets value to n"
    );
}

#[test]
fn dropdown_menu_returns_clicked_item() {
    let mut harness = Harness::new_ui_state(
        |ui, chosen: &mut Option<usize>| {
            theme_ctx(ui.ctx(), &dark());
            let trigger = ui.add(Button::secondary("Menu"));
            if let Some(index) = DropdownMenu::new().item("New").item("Open").show(&trigger) {
                *chosen = Some(index);
            }
        },
        None::<usize>,
    );

    harness.run();
    harness.get_by_label("Menu").click(); // open the popup
    harness.run();
    harness.get_by_label("New").click(); // select first item
    harness.run();
    assert_eq!(
        *harness.state(),
        Some(0),
        "selecting a menu item returns its index"
    );
}

#[cfg(feature = "datagrid")]
#[test]
fn data_grid_renders_under_all_themes() {
    use lumen_ui_widgets::DataGrid;
    let themes: [Arc<dyn Theme>; 4] = [
        Arc::new(DarkTheme::new()),
        Arc::new(LightTheme::new()),
        Arc::new(audio_dark()),
        Arc::new(high_contrast()),
    ];
    for theme in themes {
        let mut harness = Harness::new_ui(move |ui| {
            theme_ctx(ui.ctx(), &theme);
            DataGrid::new("grid")
                .sortable_column("Name")
                .column("Role")
                .row(["Ada", "Engineer"])
                .row(["Linus", "Maintainer"])
                .show(ui);
        });
        harness.run(); // a panic here fails the test
    }
}

#[cfg(feature = "datagrid")]
#[test]
fn data_grid_sorts_on_header_click() {
    use lumen_ui_widgets::{DataGrid, SortDirection, SortState};
    let mut harness = Harness::new_ui_state(
        |ui, sort: &mut Option<SortState>| {
            theme_ctx(ui.ctx(), &dark());
            DataGrid::new("users")
                .sortable_column("Name")
                .column("Role")
                .row(["Ada", "Engineer"])
                .row(["Linus", "Maintainer"])
                .sort(sort)
                .show(ui);
        },
        None::<SortState>,
    );

    harness.run();
    assert!(
        harness.query_all_by_label("Name").next().is_some(),
        "the header renders its columns"
    );
    assert!(
        harness.query_all_by_label("Ada").next().is_some(),
        "a virtualized body cell renders"
    );
    assert_eq!(*harness.state(), None, "unsorted by default");

    harness.get_by_label_contains("Name").click();
    harness.run();
    assert_eq!(
        *harness.state(),
        Some(SortState {
            column: 0,
            direction: SortDirection::Ascending
        }),
        "clicking a sortable header sorts it ascending"
    );

    harness.get_by_label_contains("Name").click();
    harness.run();
    assert_eq!(
        harness.state().map(|s| s.direction),
        Some(SortDirection::Descending),
        "clicking the active sort column toggles direction"
    );
}

#[test]
fn tree_view_selects_clicked_node() {
    let mut harness = Harness::new_ui_state(
        |ui, selected: &mut Option<usize>| {
            theme_ctx(ui.ctx(), &dark());
            TreeView::new(selected)
                .node(
                    TreeNode::branch(
                        0,
                        "src",
                        vec![TreeNode::leaf(1, "lib.rs"), TreeNode::leaf(2, "main.rs")],
                    )
                    .default_open(true),
                )
                .show(ui);
        },
        None::<usize>,
    );

    harness.run();
    assert_eq!(*harness.state(), None, "nothing selected by default");
    assert!(
        harness.query_by_label("main.rs").is_some(),
        "an open branch reveals its leaves"
    );
    harness.get_by_label("main.rs").click();
    harness.run();
    assert_eq!(
        *harness.state(),
        Some(2),
        "clicking a leaf selects its node id"
    );
}

#[test]
fn drawer_renders_under_all_themes() {
    use lumen_ui_widgets::{open_drawer, Drawer, DrawerSide};
    let themes: [Arc<dyn Theme>; 4] = [
        Arc::new(DarkTheme::new()),
        Arc::new(LightTheme::new()),
        Arc::new(audio_dark()),
        Arc::new(high_contrast()),
    ];
    for theme in themes {
        let mut harness = Harness::new_ui(move |ui| {
            theme_ctx(ui.ctx(), &theme);
            let ctx = ui.ctx().clone();
            open_drawer(&ctx, "nav");
            Drawer::new("nav").side(DrawerSide::Right).show(&ctx, |ui| {
                ui.add(Label::new("DrawerContent"));
            });
        });
        harness.run(); // a panic here fails the test
        assert!(harness.query_by_label("DrawerContent").is_some());
    }
}

#[test]
fn drawer_shows_when_open_and_hides_when_closed() {
    use lumen_ui_widgets::{close_drawer, open_drawer, Drawer};
    // 0 = request open, 1 = request close, other = idle.
    let phase = Arc::new(AtomicU32::new(0));
    let phase_c = phase.clone();

    let mut harness = Harness::new_ui(move |ui| {
        theme_ctx(ui.ctx(), &dark());
        let ctx = ui.ctx().clone();
        match phase_c.load(Ordering::Relaxed) {
            0 => open_drawer(&ctx, "nav"),
            1 => close_drawer(&ctx, "nav"),
            _ => {}
        }
        Drawer::new("nav").show(&ctx, |ui| {
            ui.add(Label::new("Drawer nav"));
        });
    });

    harness.run();
    assert!(
        harness.query_by_label("Drawer nav").is_some(),
        "an opened drawer renders its content"
    );

    phase.store(1, Ordering::Relaxed);
    harness.run();
    assert!(
        harness.query_by_label("Drawer nav").is_none(),
        "close_drawer hides the content"
    );
}
