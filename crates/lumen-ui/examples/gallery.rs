//! `gallery` — the full lumen-ui widget catalogue on one scrollable screen, with a **live theme
//! switch** across every built-in theme. Toggling the theme restyles everything, zero widget code.
//!
//! Run with: `cargo run -p lumen-ui --example gallery --features themes`

use std::sync::Arc;

use eframe::egui;
use egui::Color32;
use lumen_ui::prelude::*;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "lumen-ui — gallery",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            install(
                &cc.egui_ctx,
                Arc::new(DarkTheme::new()),
                UiContext::default(),
            );
            Ok(Box::<Gallery>::default())
        }),
    )
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Plan {
    #[default]
    Free,
    Pro,
    Team,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tag {
    Audio,
    Midi,
    Video,
}

/// Every built-in theme, in the order the switcher cycles them.
const THEMES: [&str; 8] = [
    "Dark",
    "Light",
    "AudioDark",
    "HighContrast",
    "Nord",
    "Solarized",
    "Seno Night",
    "Seno Dawn",
];

fn theme_for(index: usize) -> Arc<dyn Theme> {
    match index {
        1 => Arc::new(LightTheme::new()),
        2 => Arc::new(audio_dark()),
        3 => Arc::new(high_contrast()),
        4 => Arc::new(nord()),
        5 => Arc::new(solarized_dark()),
        6 => Arc::new(seno_night()),
        7 => Arc::new(seno_dawn()),
        _ => Arc::new(DarkTheme::new()),
    }
}

struct Gallery {
    theme: usize,
    name: String,
    bio: String,
    agree: bool,
    notify: bool,
    volume: f32,
    plan: Plan,
    priority: Plan,
    tags: Vec<Tag>,
    amount: f64,
    range_lo: f32,
    range_hi: f32,
    rating: u32,
    page: usize,
    date: Date,
    time: Time,
    color: Color32,
}

impl Default for Gallery {
    fn default() -> Self {
        Self {
            theme: 0,
            name: String::new(),
            bio: String::new(),
            agree: false,
            notify: true,
            volume: 0.5,
            plan: Plan::Free,
            priority: Plan::Pro,
            tags: vec![Tag::Audio],
            amount: 12.0,
            range_lo: 20.0,
            range_hi: 80.0,
            rating: 3,
            page: 1,
            date: Date::new(2026, 6, 19),
            time: Time::new(14, 30),
            color: Color32::from_rgb(0xe8, 0x65, 0x3d),
        }
    }
}

impl eframe::App for Gallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(Heading::display("lumen-ui gallery"));
                if ui
                    .add(Button::ghost(format!("Theme: {}", THEMES[self.theme])))
                    .clicked()
                {
                    self.theme = (self.theme + 1) % THEMES.len();
                    set_theme(ui.ctx(), theme_for(self.theme));
                }
            });
            ui.add(Label::muted(
                "Click the theme button to cycle all 8 themes — the whole gallery restyles.",
            ));
            ui.add_space(12.0);

            self.section_actions(ui);
            self.section_inputs(ui);
            self.section_selection(ui);
            self.section_pickers(ui);
            self.section_numeric(ui);
            self.section_navigation(ui);
            self.section_data(ui);
            self.section_feedback(ui);
            self.section_overlays(ui);
        });

        Modal::new("demo_modal")
            .title("Delete project?")
            .show(ui.ctx(), |ui| {
                ui.add(Label::new("This action cannot be undone."));
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.add(Button::danger("Delete")).clicked() {
                        toast_error(ui.ctx(), "Project deleted");
                        close_modal(ui.ctx(), "demo_modal");
                    }
                    if ui.add(Button::ghost("Cancel")).clicked() {
                        close_modal(ui.ctx(), "demo_modal");
                    }
                });
            });
        show_toasts(ui.ctx());
    }
}

impl Gallery {
    fn section_actions(&mut self, ui: &mut egui::Ui) {
        Card::new().show(ui, |ui| {
            ui.add(Heading::new("Buttons & badges"));
            ui.horizontal(|ui| {
                if ui.add(Button::primary("Primary")).clicked() {
                    toast_success(ui.ctx(), "Primary action done");
                }
                ui.add(Button::secondary("Secondary"));
                ui.add(Button::ghost("Ghost"));
                if ui.add(Button::danger("Danger")).clicked() {
                    toast_error(ui.ctx(), "Something went wrong");
                }
                ui.add(Button::primary("Disabled").enabled(false));
            });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add(Badge::new("Neutral"));
                ui.add(Badge::primary("Primary"));
                ui.add(Badge::success("Success"));
                ui.add(Badge::warning("Warning"));
                ui.add(Badge::danger("Danger"));
                ui.add(Avatar::new("Erwan Barat"));
            });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                Chip::new("Audio").show(ui);
                Chip::new("Removable").removable().show(ui);
                ui.add(Kbd::new("Ctrl"));
                ui.add(Kbd::new("S"));
                ui.add(Link::new("Documentation"));
            });
        });
        ui.add_space(8.0);
    }

    fn section_inputs(&mut self, ui: &mut egui::Ui) {
        Card::new().show(ui, |ui| {
            ui.add(Heading::new("Text & toggles"));
            ui.add(TextField::new(&mut self.name).hint("Your name"));
            ui.add_space(6.0);
            ui.add(Textarea::new(&mut self.bio).hint("Short bio"));
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add(Switch::new(&mut self.notify));
                ui.add(Label::new("Notifications"));
            });
            ui.add(Checkbox::new(&mut self.agree, "I accept the terms"));
        });
        ui.add_space(8.0);
    }

    fn section_selection(&mut self, ui: &mut egui::Ui) {
        Card::new().show(ui, |ui| {
            ui.add(Heading::new("Selection"));
            ui.add(
                RadioGroup::new(&mut self.plan)
                    .option(Plan::Free, "Free")
                    .option(Plan::Pro, "Pro")
                    .option(Plan::Team, "Team"),
            );
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add(Label::new("Select:"));
                Select::new("plan_select", &mut self.plan)
                    .option(Plan::Free, "Free")
                    .option(Plan::Pro, "Pro")
                    .option(Plan::Team, "Team")
                    .show(ui);
                ui.add(Label::new("Combobox:"));
                Combobox::new("priority_combo", &mut self.priority)
                    .option(Plan::Free, "Low")
                    .option(Plan::Pro, "Medium")
                    .option(Plan::Team, "High")
                    .show(ui);
            });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add(Label::new("Multi-select:"));
                MultiSelect::new("tags_multi", &mut self.tags)
                    .option(Tag::Audio, "Audio")
                    .option(Tag::Midi, "MIDI")
                    .option(Tag::Video, "Video")
                    .show(ui);
            });
        });
        ui.add_space(8.0);
    }

    fn section_pickers(&mut self, ui: &mut egui::Ui) {
        Card::new().show(ui, |ui| {
            ui.add(Heading::new("Pickers"));
            ui.horizontal(|ui| {
                ui.add(Label::new("Date:"));
                DatePicker::new("date_pick", &mut self.date).show(ui);
                ui.add(Label::new("Time:"));
                TimePicker::new(&mut self.time).show(ui);
            });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add(Label::new("Color:"));
                ui.add(ColorPicker::new(&mut self.color));
            });
        });
        ui.add_space(8.0);
    }

    fn section_numeric(&mut self, ui: &mut egui::Ui) {
        Card::new().show(ui, |ui| {
            ui.add(Heading::new("Numeric & ranges"));
            ui.horizontal(|ui| {
                ui.add(Slider::new(&mut self.volume, 0.0..=1.0));
                ui.add(Label::muted(format!("{:.0}%", self.volume * 100.0)));
            });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add(Label::new("Amount:"));
                ui.add(NumberInput::new(&mut self.amount, 0.0..=100.0));
            });
            ui.add_space(6.0);
            ui.add(Label::new("Range:"));
            ui.add(RangeSlider::new(
                &mut self.range_lo,
                &mut self.range_hi,
                0.0..=100.0,
            ));
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add(Label::new("Rating:"));
                Rating::new(&mut self.rating).show(ui);
            });
        });
        ui.add_space(8.0);
    }

    fn section_navigation(&mut self, ui: &mut egui::Ui) {
        Card::new().show(ui, |ui| {
            ui.add(Heading::new("Navigation"));
            Breadcrumb::new()
                .item("Home")
                .item("Library")
                .item("Audio")
                .show(ui);
            ui.add_space(6.0);
            let tab = Tabs::new("gallery_tabs")
                .tab("Overview")
                .tab("Details")
                .tab("Activity")
                .show(ui);
            ui.add_space(6.0);
            match tab {
                0 => ui.add(Label::new("Overview content.")),
                1 => ui.add(Label::new("Detailed content.")),
                _ => ui.add(Label::new("Recent activity.")),
            };
            ui.add_space(6.0);
            if let Some(p) = Pagination::new(self.page, 10).show(ui) {
                self.page = p;
            }
        });
        ui.add_space(8.0);
        Accordion::new("Advanced settings")
            .default_open(false)
            .show(ui, |ui| {
                ui.add(Label::muted("Collapsible content, state kept by egui."));
            });
        ui.add_space(8.0);
    }

    fn section_data(&mut self, ui: &mut egui::Ui) {
        Card::new().show(ui, |ui| {
            ui.add(Heading::new("Data"));
            Table::new("tracks_table")
                .column("Track")
                .column("Type")
                .column("Bars")
                .row(["Drums", "Audio", "32"])
                .row(["Bass", "MIDI", "16"])
                .row(["Pad", "MIDI", "8"])
                .striped(true)
                .show(ui);
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add(Stat::new("Revenue", "$12.4k").delta("+8%", true));
                ui.add(Stat::new("Churn", "2.1%").delta("-0.4%", false));
            });
            ui.add_space(8.0);
            DescriptionList::new("specs")
                .item("Format", "WAV 24-bit")
                .item("Sample rate", "48 kHz")
                .show(ui);
            ui.add_space(8.0);
            Timeline::new()
                .event_detailed("Imported", "12 audio files")
                .event_detailed("Edited", "Trimmed clip 3")
                .event("Exported")
                .show(ui);
        });
        ui.add_space(8.0);
    }

    fn section_feedback(&mut self, ui: &mut egui::Ui) {
        Card::new().show(ui, |ui| {
            ui.add(Heading::new("Feedback"));
            ui.add(Alert::info("Heads up — this is an informational alert.").title("Info"));
            ui.add_space(6.0);
            ui.add(Alert::success("Saved successfully."));
            ui.add_space(6.0);
            ui.add(Progress::new(self.volume));
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add(CircularProgress::new(self.volume));
                ui.add(Spinner::new());
                ui.add(Skeleton::new(120.0, 16.0));
            });
        });
        ui.add_space(8.0);
    }

    fn section_overlays(&mut self, ui: &mut egui::Ui) {
        Card::new().show(ui, |ui| {
            ui.add(Heading::new("Overlays"));
            ui.horizontal(|ui| {
                tooltip(ui.add(Button::ghost("Hover me")), "A themed tooltip");
                let menu = ui.add(Button::secondary("Popover"));
                popover(&menu, |ui| {
                    ui.add(Label::new("Popover content"));
                    ui.add(Label::muted("closes on outside click"));
                });
                if ui.add(Button::primary("Open dialog")).clicked() {
                    open_modal(ui.ctx(), "demo_modal");
                }
            });
            let target = ui.add(Label::new("Right-click me"));
            context_menu(&target, |ui| {
                if ui.add(Button::ghost("Copy")).clicked() {
                    toast(ui.ctx(), "Copied");
                }
            });
        });
        ui.add_space(8.0);
    }
}
