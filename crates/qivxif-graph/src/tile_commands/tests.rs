use super::*;
use crate::{equal_split_sizes, SplitAxis, TileTree};

#[test]
fn focusing_is_local_to_one_stack() {
    let left_a = tab("left-a", None);
    let left_b = tab("left-b", None);
    let right = tab("right", None);
    let layout = split_layout(
        stack_tile(0, vec![left_a.clone(), left_b.clone()]),
        stack_tile(0, vec![right.clone()]),
    );

    let next = focus_tab(layout, &left_b.pane_node_id).unwrap();

    let split = expect_split(&next.root);
    assert_eq!(stack_active(&split.1[0]), 1);
    assert_eq!(stack_active(&split.1[1]), 0);
}

#[test]
fn opening_tab_targets_only_anchor_stack() {
    let left = tab("left", None);
    let right = tab("right", None);
    let created = tab("created", None);
    let layout = split_layout(
        stack_tile(0, vec![left.clone()]),
        stack_tile(0, vec![right.clone()]),
    );

    let next = open_tab(layout, &right.pane_node_id, created.clone()).unwrap();

    let split = expect_split(&next.root);
    assert_eq!(stack_tabs(&split.1[0]), vec![left.pane_node_id]);
    assert_eq!(
        stack_tabs(&split.1[1]),
        vec![right.pane_node_id, created.pane_node_id]
    );
    assert_eq!(stack_active(&split.1[1]), 1);
}

#[test]
fn same_resource_tabs_keep_distinct_pane_ids() {
    let target = NodeId::generate();
    let first = tab("first", Some(target.clone()));
    let second = tab("second", Some(target.clone()));
    let layout = single_layout(first.clone());

    let next = open_tab(layout, &first.pane_node_id, second.clone()).unwrap();

    assert_eq!(next.pane_count(), 2);
    let tabs = stack_items(&next.root);
    assert_ne!(tabs[0].pane_node_id, tabs[1].pane_node_id);
    assert_eq!(tabs[0].target_node_id, tabs[1].target_node_id);
}

#[test]
fn closing_last_tab_collapses_source_stack() {
    let left = tab("left", None);
    let right = tab("right", None);
    let layout = split_layout(
        stack_tile(0, vec![left.clone()]),
        stack_tile(0, vec![right.clone()]),
    );

    let next = close_tab(layout, &right.pane_node_id).unwrap();

    assert_eq!(next.pane_count(), 1);
    assert_eq!(stack_tabs(&next.root), vec![left.pane_node_id]);
}

#[test]
fn closing_only_root_tab_is_rejected() {
    let only = tab("only", None);
    let layout = single_layout(only.clone());

    let error = close_tab(layout, &only.pane_node_id).unwrap_err();

    assert_eq!(error, GraphError::TileLayoutEmpty);
}

#[test]
fn moving_last_tab_collapses_source_and_activates_target_stack() {
    let left = tab("left", None);
    let right = tab("right", None);
    let layout = split_layout(
        stack_tile(0, vec![left.clone()]),
        stack_tile(0, vec![right.clone()]),
    );

    let next = move_tab_to_stack(layout, &right.pane_node_id, &left.pane_node_id).unwrap();

    assert_eq!(next.pane_count(), 2);
    assert_eq!(
        stack_tabs(&next.root),
        vec![left.pane_node_id, right.pane_node_id]
    );
    assert_eq!(stack_active(&next.root), 1);
}

#[test]
fn moving_tab_to_edge_creates_sibling_stack() {
    let left = tab("left", None);
    let right = tab("right", None);
    let layout = single_layout_with_tabs(vec![left.clone(), right.clone()]);

    let next = move_tab_to_edge(
        layout,
        &right.pane_node_id,
        &left.pane_node_id,
        SplitDirection::Right,
    )
    .unwrap();

    let split = expect_split(&next.root);
    assert_eq!(stack_tabs(&split.1[0]), vec![left.pane_node_id]);
    assert_eq!(stack_tabs(&split.1[1]), vec![right.pane_node_id]);
}

#[test]
fn resize_split_updates_direct_parent_sizes() {
    let left = tab("left", None);
    let right = tab("right", None);
    let layout = split_layout(
        stack_tile(0, vec![left.clone()]),
        stack_tile(0, vec![right.clone()]),
    );

    let next = resize_split_layout(layout, &left.pane_node_id, vec![700, 300]).unwrap();

    let split = expect_split(&next.root);
    assert_eq!(split.2, vec![700, 300]);
}

#[test]
fn maximizing_missing_pane_rejects() {
    let layout = single_layout(tab("left", None));

    let error = maximize_pane(layout, &NodeId::generate()).unwrap_err();

    assert_eq!(error, GraphError::PaneMissing);
}

fn single_layout(tab: TileTab) -> TileLayout {
    single_layout_with_tabs(vec![tab])
}

fn single_layout_with_tabs(tabs: Vec<TileTab>) -> TileLayout {
    TileLayout {
        root: stack_tile(0, tabs),
        maximized_pane_id: None,
    }
}

fn split_layout(first: TileTree, second: TileTree) -> TileLayout {
    TileLayout {
        root: split_tile(SplitAxis::Row, vec![first, second]),
        maximized_pane_id: None,
    }
}

fn stack_tile(active: usize, tabs: Vec<TileTab>) -> TileTree {
    TileTree::Stack { active, tabs }
}

fn split_tile(axis: SplitAxis, children: Vec<TileTree>) -> TileTree {
    let count = children.len();
    TileTree::Split {
        axis,
        children,
        sizes: equal_split_sizes(count),
    }
}

fn tab(title: &str, target_node_id: Option<NodeId>) -> TileTab {
    TileTab {
        pane_node_id: NodeId::generate(),
        pane_kind: "text_editor".to_owned(),
        target_node_id,
        title: title.to_owned(),
    }
}

fn expect_split(tile: &TileTree) -> (SplitAxis, &[TileTree], &[u16]) {
    match tile {
        TileTree::Split { axis, children, sizes } => (*axis, children.as_slice(), sizes.as_slice()),
        TileTree::Stack { .. } => panic!("expected split"),
    }
}

fn stack_items(tile: &TileTree) -> &[TileTab] {
    match tile {
        TileTree::Stack { tabs, .. } => tabs,
        TileTree::Split { .. } => panic!("expected stack"),
    }
}

fn stack_tabs(tile: &TileTree) -> Vec<NodeId> {
    stack_items(tile)
        .iter()
        .map(|tab| tab.pane_node_id.clone())
        .collect()
}

fn stack_active(tile: &TileTree) -> usize {
    match tile {
        TileTree::Stack { active, .. } => *active,
        TileTree::Split { .. } => panic!("expected stack"),
    }
}
