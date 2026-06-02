use super::{TileLayout, TileTab, TileTree};
use crate::{SplitAxis, equal_split_sizes};
use qivxif_core::NodeId;

pub fn single_layout(tab: TileTab) -> TileLayout {
    TileLayout {
        root: stack_tile(0, vec![tab]),
        maximized_pane_id: None,
    }
}

pub fn single_layout_with_tabs(tabs: Vec<TileTab>) -> TileLayout {
    TileLayout {
        root: stack_tile(0, tabs),
        maximized_pane_id: None,
    }
}

pub fn split_layout(first: TileTree, second: TileTree) -> TileLayout {
    TileLayout {
        root: split_tile(SplitAxis::Row, vec![first, second]),
        maximized_pane_id: None,
    }
}

pub fn stack_tile(active: usize, tabs: Vec<TileTab>) -> TileTree {
    TileTree::Stack { active, tabs }
}

pub fn split_tile(axis: SplitAxis, children: Vec<TileTree>) -> TileTree {
    let count = children.len();
    TileTree::Split {
        axis,
        children,
        sizes: equal_split_sizes(count),
    }
}

pub fn tab(title: &str, target_node_id: Option<NodeId>) -> TileTab {
    TileTab {
        pane_node_id: NodeId::generate(),
        pane_kind: "text_editor".to_owned(),
        target_node_id,
        title: title.to_owned(),
    }
}

pub fn expect_split(tile: &TileTree) -> (SplitAxis, &[TileTree], &[u16]) {
    match tile {
        TileTree::Split {
            axis,
            children,
            sizes,
        } => (*axis, children.as_slice(), sizes.as_slice()),
        TileTree::Stack { .. } => panic!("expected split"),
    }
}

pub fn stack_items(tile: &TileTree) -> &[TileTab] {
    match tile {
        TileTree::Stack { tabs, .. } => tabs,
        TileTree::Split { .. } => panic!("expected stack"),
    }
}

pub fn stack_tabs(tile: &TileTree) -> Vec<NodeId> {
    stack_items(tile)
        .iter()
        .map(|tab| tab.pane_node_id.clone())
        .collect()
}

pub fn stack_active(tile: &TileTree) -> usize {
    match tile {
        TileTree::Stack { active, .. } => *active,
        TileTree::Split { .. } => panic!("expected stack"),
    }
}
