//! [`AuthCard`] — a centered login / sign-up card.
//!
//! Composes [`Card`] + [`FormField`] + [`TextField`] + [`Button`] (and an optional
//! "remember me" [`Checkbox`] and secondary [`Link`]) into the canonical auth surface.
//! Width is constrained and the card is horizontally centered; spacing comes from theme
//! tokens. Returns which actions fired this frame so the caller drives the auth flow.

use egui::Ui;
use lumen_ui_core::UiThemeExt;
use lumen_ui_widgets::{Button, Card, Checkbox, FormField, Heading, Link, TextField};

const CARD_WIDTH: f32 = 360.0;

/// What the user did in an [`AuthCard`] this frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AuthCardResponse {
    /// The submit button was clicked.
    pub submitted: bool,
    /// The secondary link was clicked (e.g. "Forgot password?").
    pub secondary_clicked: bool,
}

/// A login / sign-up card bound to caller-owned `email` and `password` buffers.
///
/// ```ignore
/// let r = AuthCard::new("Sign in", &mut email, &mut password)
///     .remember(&mut remember_me)
///     .secondary_link("Forgot password?")
///     .show(ui);
/// if r.submitted { /* authenticate */ }
/// ```
pub struct AuthCard<'a> {
    title: String,
    email: &'a mut String,
    password: &'a mut String,
    remember: Option<&'a mut bool>,
    submit_label: String,
    secondary: Option<String>,
}

impl<'a> AuthCard<'a> {
    #[must_use]
    pub fn new(title: impl Into<String>, email: &'a mut String, password: &'a mut String) -> Self {
        Self {
            title: title.into(),
            email,
            password,
            remember: None,
            submit_label: "Sign in".to_owned(),
            secondary: None,
        }
    }

    /// Add a "remember me" checkbox bound to `remember`.
    #[must_use]
    pub fn remember(mut self, remember: &'a mut bool) -> Self {
        self.remember = Some(remember);
        self
    }

    /// Override the submit button label (default: "Sign in").
    #[must_use]
    pub fn submit_label(mut self, label: impl Into<String>) -> Self {
        self.submit_label = label.into();
        self
    }

    /// Add a secondary link below the submit button (e.g. "Forgot password?").
    #[must_use]
    pub fn secondary_link(mut self, text: impl Into<String>) -> Self {
        self.secondary = Some(text.into());
        self
    }

    /// Draw the card, centered and width-constrained. Returns the actions that fired.
    pub fn show(self, ui: &mut Ui) -> AuthCardResponse {
        let AuthCard {
            title,
            email,
            password,
            remember,
            submit_label,
            secondary,
        } = self;
        let spacing = ui.theme().tokens().spacing;
        let mut out = AuthCardResponse::default();

        ui.vertical_centered(|ui| {
            ui.set_max_width(CARD_WIDTH);
            Card::new().show(ui, |ui| {
                ui.add(Heading::new(title));
                ui.add_space(spacing.md);

                FormField::new("Email").show(ui, |ui| {
                    ui.add(TextField::new(email));
                });
                ui.add_space(spacing.md);
                FormField::new("Password").show(ui, |ui| {
                    ui.add(TextField::new(password).password(true));
                });

                if let Some(remember) = remember {
                    ui.add_space(spacing.md);
                    ui.add(Checkbox::new(remember, "Remember me"));
                }

                ui.add_space(spacing.md);
                if ui.add(Button::primary(submit_label)).clicked() {
                    out.submitted = true;
                }

                if let Some(secondary) = secondary {
                    ui.add_space(spacing.sm);
                    if ui.add(Link::new(secondary)).clicked() {
                        out.secondary_clicked = true;
                    }
                }
            });
        });

        out
    }
}
