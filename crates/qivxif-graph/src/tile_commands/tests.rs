use super::*;
use crate::TileTree;
use qivxif_core::NodeId;

mod fixtures;

#[test]
fn focusing_is_local_to_one_stack() {
    let left_a = fixtures::tab("left-a", None);
    let left_b = fixtures::tab("left-b", None);
    let right = fixtures::tab("right", None);
    let layout = fixtures::split_layout(
        fixtures::stack_tile(0, vec![left_a.clone(), left_b.clone()]),
        fixtures::stack_tile(0, vec![right.clone()]),
    );
    let next = focus_tab(layout, &left_b.pane_node_id).unwrap();
    let split = fixtures::expect_split(&next.root);
    assert_eq!(fixtures::stack_active(&split.1[0]), 1);
    assert_eq!(fixtures::stack_active(&split.1[1]), 0);
}

#[test]
fn opening_tab_targets_only_anchor_stack() {
    let left = fixtures::tab("left", None);
    let right = fixtures::tab("right", None);
    let created = fixtures::tab("created", None);
    let layout = fixtures::split_layout(
        fixtures::stack_tile(0, vec![left.clone()]),
        fixtures::stack_tile(0, vec![right.clone()]),
    );
    let next = open_tab(layout, &right.pane_node_id, created.clone()).unwrap();
    let split = fixtures::expect_split(&next.root);
    assert_eq!(fixtures::stack_tabs(&split.1[0]), vec![left.pane_node_id]);
    assert_eq!(
        fixtures::stack_tabs(&split.1[1]),
        vec![right.pane_node_id, created.pane_node_id]
    );
}

#[test]
fn same_resource_tabs_keep_distinct_pane_ids() {
    let target = NodeId::generate();
    let first = fixtures::tab("first", Some(target.clone()));
    let second = fixtures::tab("second", Some(target.clone()));
    let layout = fixtures::single_layout(first.clone());
    let next = open_tab(layout, &first.pane_node_id, second.clone()).unwrap();
    assert_eq!(next.pane_count(), 2);
    let tabs = fixtures::stack_items(&next.root);
    assert_ne!(tabs[0].pane_node_id, tabs[1].pane_node_id);
}

#[test]
fn closing_last_tab_collapses_source_stack() {
    let left = fixtures::tab("left", None);
    let right = fixtures::tab("right", None);
    let layout = fixtures::split_layout(
        fixtures::stack_tile(0, vec![left.clone()]),
        fixtures::stack_tile(0, vec![right.clone()]),
    );
    let next = close_tab(layout, &right.pane_node_id).unwrap();
    assert_eq!(next.pane_count(), 1);
}

#[test]
fn closing_only_root_tab_is_rejected() {
    let only = fixtures::tab("only", None);
    let error = close_tab(fixtures::single_layout(only.clone()), &only.pane_node_id).unwrap_err();
    assert_eq!(error, GraphError::TileLayoutEmpty);
}

#[test]
fn moving_last_tab_collapses_source_and_activates_target_stack() {
    let left = fixtures::tab("left", None);
    let right = fixtures::tab("right", None);
    let layout = fixtures::split_layout(
        fixtures::stack_tile(0, vec![left.clone()]),
        fixtures::stack_tile(0, vec![right.clone()]),
    );
    let next = move_tab_to_stack(layout, &right.pane_node_id, &left.pane_node_id).unwrap();
    assert_eq!(next.pane_count(), 2);
}

#[test]
fn moving_tab_to_edge_creates_sibling_stack() {
    let left = fixtures::tab("left", None);
    let right = fixtures::tab("right", None);
    let layout = fixtures::single_layout_with_tabs(vec![left.clone(), right.clone()]);
    let next = move_tab_to_edge(
        layout,
        &right.pane_node_id,
        &left.pane_node_id,
        SplitDirection::Right,
    )
    .unwrap();
    let split = fixtures::expect_split(&next.root);
    assert_eq!(fixtures::stack_tabs(&split.1[0]), vec![left.pane_node_id]);
}

#[test]
fn resize_split_updates_direct_parent_sizes() {
    let left = fixtures::tab("left", None);
    let right = fixtures::tab("right", None);
    let layout = fixtures::split_layout(
        fixtures::stack_tile(0, vec![left.clone()]),
        fixtures::stack_tile(0, vec![right.clone()]),
    );
    let next = resize_split_layout(layout, &left.pane_node_id, vec![700, 300]).unwrap();
    let split = fixtures::expect_split(&next.root);
    assert_eq!(split.2, vec![700, 300]);
}

#[test]
fn maximizing_missing_pane_rejects() {
    let error = maximize_pane(
        fixtures::single_layout(fixtures::tab("left", None)),
        &NodeId::generate(),
    )
    .unwrap_err();
    assert_eq!(error, GraphError::PaneMissing);
}
