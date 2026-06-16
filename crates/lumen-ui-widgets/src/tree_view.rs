//! [`TreeView`] — a hierarchical, collapsible node list with single selection.

use egui::{CollapsingHeader, Response, RichText, Ui};
use lumen_ui_core::{TreeViewRecipe, UiThemeExt};

/// One node in a [`TreeView`]. A node with children renders as a collapsible
/// branch; a node without children is a selectable leaf. `id` must be unique
/// within the tree (it keys both selection and the collapse state).
#[derive(Clone, Debug)]
pub struct TreeNode {
    id: usize,
    label: String,
    children: Vec<TreeNode>,
    default_open: bool,
}

impl TreeNode {
    /// A selectable leaf node.
    #[must_use]
    pub fn leaf(id: usize, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            children: Vec::new(),
            default_open: false,
        }
    }

    /// A branch node holding `children` (also selectable via its header).
    #[must_use]
    pub fn branch(id: usize, label: impl Into<String>, children: Vec<TreeNode>) -> Self {
        Self {
            id,
            label: label.into(),
            children,
            default_open: false,
        }
    }

    /// Expand this branch by default (no effect on a leaf).
    #[must_use]
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
}

/// A hierarchical tree with single selection bound to the selected node `id`.
/// Clicking a row selects it; a branch header also toggles its children.
///
/// Selection highlight uses egui's themed selection visuals; node text and the
/// per-level indent come from [`TreeViewRecipe`].
///
/// ```ignore
/// TreeView::new(&mut selected)
///     .node(TreeNode::branch(0, "src", vec![
///         TreeNode::leaf(1, "lib.rs"),
///         TreeNode::leaf(2, "main.rs"),
///     ]).default_open(true))
///     .show(ui);
/// ```
#[derive(Debug)]
pub struct TreeView<'a> {
    roots: Vec<TreeNode>,
    selected: &'a mut Option<usize>,
}

impl<'a> TreeView<'a> {
    #[must_use]
    pub fn new(selected: &'a mut Option<usize>) -> Self {
        Self {
            roots: Vec::new(),
            selected,
        }
    }

    /// Append a root node.
    #[must_use]
    pub fn node(mut self, node: TreeNode) -> Self {
        self.roots.push(node);
        self
    }

    /// Draw the tree. Returns the response of the enclosing vertical layout.
    pub fn show(self, ui: &mut Ui) -> Response {
        let recipe = TreeViewRecipe::resolve(ui.theme().tokens(), &ui.ui_ctx());
        let TreeView { roots, selected } = self;
        ui.spacing_mut().indent = recipe.indent;
        ui.vertical(|ui| {
            for node in &roots {
                render_node(ui, node, selected, &recipe);
            }
        })
        .response
    }
}

/// Render one node and its subtree. `selected` is auto-reborrowed at each call.
fn render_node(
    ui: &mut Ui,
    node: &TreeNode,
    selected: &mut Option<usize>,
    recipe: &TreeViewRecipe,
) {
    let is_selected = *selected == Some(node.id);
    let color = if is_selected {
        recipe.selected_color
    } else {
        recipe.text_color
    };
    let text = RichText::new(&node.label)
        .color(color)
        .size(recipe.text_size);

    if node.children.is_empty() {
        if ui.selectable_label(is_selected, text).clicked() {
            *selected = Some(node.id);
        }
        return;
    }

    let response = CollapsingHeader::new(text)
        .id_salt(node.id)
        .default_open(node.default_open)
        .show(ui, |ui| {
            for child in &node.children {
                render_node(ui, child, selected, recipe);
            }
        });
    if response.header_response.clicked() {
        *selected = Some(node.id);
    }
}
