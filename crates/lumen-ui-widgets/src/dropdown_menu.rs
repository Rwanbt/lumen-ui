//! [`DropdownMenu`] — a click-toggled menu of actions anchored to a trigger.

use egui::{Button, Popup, Response, RichText, Ui};
use lumen_ui_core::{MenuRecipe, UiThemeExt};

/// A menu of action items shown in a popover anchored to a trigger response (e.g.
/// a button). [`DropdownMenu::show`] returns the index of a clicked item, if any,
/// and closes the menu on selection.
///
/// Distinct from [`crate::Select`]: a dropdown menu triggers actions rather than
/// binding a chosen value.
#[derive(Clone, Debug, Default)]
pub struct DropdownMenu {
    items: Vec<String>,
}

impl DropdownMenu {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn item(mut self, label: impl Into<String>) -> Self {
        self.items.push(label.into());
        self
    }

    /// Anchor to `trigger` and show when toggled. Returns the clicked item index.
    pub fn show(self, trigger: &Response) -> Option<usize> {
        let mut chosen = None;
        Popup::from_toggle_button_response(trigger).show(|ui: &mut Ui| {
            let recipe = MenuRecipe::resolve(ui.theme().tokens());
            for (index, item) in self.items.iter().enumerate() {
                let clicked = ui
                    .add(
                        Button::new(
                            RichText::new(item)
                                .color(recipe.text_color)
                                .size(recipe.text_size),
                        )
                        .frame(false),
                    )
                    .clicked();
                if clicked {
                    chosen = Some(index);
                    ui.close();
                }
            }
        });
        chosen
    }
}
