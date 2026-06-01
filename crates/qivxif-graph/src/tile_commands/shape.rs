use super::SplitDirection;
use crate::{GraphError, GraphResult, SplitAxis, TileTab, TileTree};
use qivxif_core::NodeId;

pub(super) fn reject_existing(root: &TileTree, pane_id: &NodeId) -> GraphResult<()> {
    if contains_pane(root, pane_id) {
        Err(GraphError::PaneExists)
    } else {
        Ok(())
    }
}

pub(super) fn ensure_contains(root: &TileTree, pane_id: &NodeId) -> GraphResult<()> {
    contains_pane(root, pane_id)
        .then_some(())
        .ok_or(GraphError::PaneMissing)
}

pub(super) fn activate(tile: &mut TileTree, pane_id: &NodeId) -> bool {
    match tile {
        TileTree::Split { first, second, .. } => {
            activate(first, pane_id) || activate(second, pane_id)
        }
        TileTree::Stack { active, tabs } => {
            match tabs.iter().position(|tab| &tab.pane_node_id == pane_id) {
                Some(index) => {
                    *active = index;
                    true
                }
                None => false,
            }
        }
    }
}

pub(super) fn append_to_stack(tile: &mut TileTree, target: &NodeId, tab: TileTab) -> bool {
    match tile {
        TileTree::Split { first, second, .. } => {
            append_to_stack(first, target, tab.clone()) || append_to_stack(second, target, tab)
        }
        TileTree::Stack { active, tabs }
            if tabs.iter().any(|item| &item.pane_node_id == target) =>
        {
            *active = tabs.len();
            tabs.push(tab);
            true
        }
        TileTree::Stack { .. } => false,
    }
}

pub(super) fn remove_tab(tile: TileTree, pane_id: &NodeId) -> (Option<TileTree>, Option<TileTab>) {
    match tile {
        TileTree::Stack {
            mut active,
            mut tabs,
        } => remove_from_stack(&mut active, &mut tabs, pane_id),
        TileTree::Split {
            axis,
            ratio_percent,
            first,
            second,
        } => {
            let (first_tile, removed) = remove_tab(*first, pane_id);
            if removed.is_some() {
                return (
                    join(axis, ratio_percent, first_tile, Some(*second)),
                    removed,
                );
            }
            let (second_tile, removed) = remove_tab(*second, pane_id);
            (join(axis, ratio_percent, first_tile, second_tile), removed)
        }
    }
}

pub(super) fn split_stack(
    tile: TileTree,
    target: &NodeId,
    tab: TileTab,
    direction: SplitDirection,
) -> (TileTree, bool) {
    match tile {
        TileTree::Split {
            axis,
            ratio_percent,
            first,
            second,
        } => {
            let (first_tile, changed) = split_stack(*first, target, tab.clone(), direction);
            if changed {
                return (split(axis, ratio_percent, first_tile, *second), true);
            }
            let (second_tile, changed) = split_stack(*second, target, tab, direction);
            (split(axis, ratio_percent, first_tile, second_tile), changed)
        }
        TileTree::Stack { active, tabs }
            if tabs.iter().any(|item| &item.pane_node_id == target) =>
        {
            (
                split_for_direction(stack(active, tabs), stack(0, vec![tab]), direction),
                true,
            )
        }
        other => (other, false),
    }
}

pub(super) fn clear_maximized(maximized: Option<NodeId>, pane_id: &NodeId) -> Option<NodeId> {
    maximized.filter(|id| id != pane_id)
}

fn remove_from_stack(
    active: &mut usize,
    tabs: &mut Vec<TileTab>,
    pane_id: &NodeId,
) -> (Option<TileTree>, Option<TileTab>) {
    let Some(index) = tabs.iter().position(|tab| &tab.pane_node_id == pane_id) else {
        return (Some(stack(*active, std::mem::take(tabs))), None);
    };
    let tab = tabs.remove(index);
    if tabs.is_empty() {
        return (None, Some(tab));
    }
    *active = next_active(*active, index, tabs.len());
    (Some(stack(*active, std::mem::take(tabs))), Some(tab))
}

fn contains_pane(tile: &TileTree, pane_id: &NodeId) -> bool {
    match tile {
        TileTree::Split { first, second, .. } => {
            contains_pane(first, pane_id) || contains_pane(second, pane_id)
        }
        TileTree::Stack { tabs, .. } => tabs.iter().any(|tab| &tab.pane_node_id == pane_id),
    }
}

fn next_active(active: usize, removed_index: usize, len: usize) -> usize {
    if removed_index < active {
        active - 1
    } else {
        active.min(len - 1)
    }
}

fn join(
    axis: SplitAxis,
    ratio_percent: u8,
    first: Option<TileTree>,
    second: Option<TileTree>,
) -> Option<TileTree> {
    match (first, second) {
        (Some(first), Some(second)) => Some(split(axis, ratio_percent, first, second)),
        (Some(tile), None) | (None, Some(tile)) => Some(tile),
        (None, None) => None,
    }
}

fn split_for_direction(
    existing: TileTree,
    created: TileTree,
    direction: SplitDirection,
) -> TileTree {
    let axis = match direction {
        SplitDirection::Left | SplitDirection::Right => SplitAxis::Row,
        SplitDirection::Top | SplitDirection::Bottom => SplitAxis::Column,
    };
    match direction {
        SplitDirection::Left | SplitDirection::Top => split(axis, 50, created, existing),
        SplitDirection::Right | SplitDirection::Bottom => split(axis, 50, existing, created),
    }
}

fn split(axis: SplitAxis, ratio_percent: u8, first: TileTree, second: TileTree) -> TileTree {
    TileTree::Split {
        axis,
        ratio_percent,
        first: Box::new(first),
        second: Box::new(second),
    }
}

fn stack(active: usize, tabs: Vec<TileTab>) -> TileTree {
    TileTree::Stack { active, tabs }
}
