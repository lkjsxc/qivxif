use super::SplitDirection;
use super::insert::insert_pane_by_edge;
use crate::{equal_split_sizes, SplitAxis, TileTab, TileTree};
use qivxif_core::NodeId;

pub(super) fn reject_existing(root: &TileTree, pane_id: &NodeId) -> crate::GraphResult<()> {
    if contains_pane(root, pane_id) {
        Err(crate::GraphError::PaneExists)
    } else {
        Ok(())
    }
}

pub(super) fn ensure_contains(root: &TileTree, pane_id: &NodeId) -> crate::GraphResult<()> {
    contains_pane(root, pane_id)
        .then_some(())
        .ok_or(crate::GraphError::PaneMissing)
}

pub(super) fn activate(tile: &mut TileTree, pane_id: &NodeId) -> bool {
    match tile {
        TileTree::Split { children, .. } => children.iter_mut().any(|child| activate(child, pane_id)),
        TileTree::Stack { active, tabs } => match tabs.iter().position(|tab| &tab.pane_node_id == pane_id) {
            Some(index) => {
                *active = index;
                true
            }
            None => false,
        },
    }
}

pub(super) fn append_to_stack(tile: &mut TileTree, target: &NodeId, tab: TileTab) -> bool {
    match tile {
        TileTree::Split { children, .. } => {
            children.iter_mut().any(|child| append_to_stack(child, target, tab.clone()))
        }
        TileTree::Stack { active, tabs } if tabs.iter().any(|item| &item.pane_node_id == target) => {
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
        TileTree::Split { axis, children, sizes } => {
            for index in 0..children.len() {
                let (maybe_child, maybe_removed) = remove_tab(children[index].clone(), pane_id);
                if maybe_removed.is_some() {
                    let mut out_children = children;
                    let mut out_sizes = sizes;
                    match maybe_child {
                        Some(child) => out_children[index] = child,
                        None => {
                            out_children.remove(index);
                            out_sizes.remove(index);
                        }
                    }
                    return (collapse_split(axis, out_children, out_sizes), maybe_removed);
                }
            }
            (
                Some(TileTree::Split {
                    axis,
                    children,
                    sizes,
                }),
                None,
            )
        }
    }
}

pub(super) fn split_stack(
    tile: TileTree,
    target: &NodeId,
    tab: TileTab,
    direction: SplitDirection,
) -> (TileTree, bool) {
    if let Some(next) = insert_pane_by_edge(&tile, target, direction, stack(0, vec![tab])) {
        return (next, true);
    }
    (tile, false)
}

pub(super) fn resize_split(
    tile: &mut TileTree,
    pane_id: &NodeId,
    sizes: Vec<u16>,
) -> bool {
    match tile {
        TileTree::Split { children, sizes: current, .. } => {
            if children.iter().any(|child| direct_child_contains(child, pane_id))
                && sizes.len() == children.len()
            {
                *current = sizes;
                return true;
            }
            children.iter_mut().any(|child| resize_split(child, pane_id, sizes.clone()))
        }
        TileTree::Stack { .. } => false,
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
        TileTree::Split { children, .. } => children.iter().any(|child| contains_pane(child, pane_id)),
        TileTree::Stack { tabs, .. } => tabs.iter().any(|tab| &tab.pane_node_id == pane_id),
    }
}

fn direct_child_contains(child: &TileTree, pane_id: &NodeId) -> bool {
    match child {
        TileTree::Stack { tabs, .. } => tabs.iter().any(|tab| &tab.pane_node_id == pane_id),
        TileTree::Split { .. } => false,
    }
}

fn collapse_split(axis: SplitAxis, children: Vec<TileTree>, sizes: Vec<u16>) -> Option<TileTree> {
    let count = children.len();
    match count {
        0 => None,
        1 => Some(children.into_iter().next().expect("one child")),
        _ => Some(TileTree::Split {
            axis,
            children,
            sizes: normalize_sizes(sizes, count),
        }),
    }
}

fn normalize_sizes(sizes: Vec<u16>, count: usize) -> Vec<u16> {
    if sizes.len() == count {
        sizes
    } else {
        equal_split_sizes(count)
    }
}

fn next_active(active: usize, removed_index: usize, len: usize) -> usize {
    if removed_index < active {
        active - 1
    } else {
        active.min(len.saturating_sub(1))
    }
}

fn stack(active: usize, tabs: Vec<TileTab>) -> TileTree {
    TileTree::Stack { active, tabs }
}
