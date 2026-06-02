use super::SplitDirection;
use crate::{equal_split_sizes, SplitAxis, TileTree};
use qivxif_core::NodeId;

pub(super) fn insert_pane_by_edge(
    layout: &TileTree,
    target_pane_id: &NodeId,
    direction: SplitDirection,
    new_pane: TileTree,
) -> Option<TileTree> {
    match layout {
        TileTree::Stack { tabs, .. } if tabs.iter().any(|tab| &tab.pane_node_id == target_pane_id) => {
            Some(split_for_direction(layout.clone(), new_pane, direction))
        }
        TileTree::Split { axis, children, sizes } => {
            insert_into_split(*axis, children, sizes, target_pane_id, direction, new_pane)
        }
        TileTree::Stack { .. } => None,
    }
}

fn insert_into_split(
    axis: SplitAxis,
    children: &[TileTree],
    sizes: &[u16],
    target_pane_id: &NodeId,
    direction: SplitDirection,
    new_pane: TileTree,
) -> Option<TileTree> {
    for (index, child) in children.iter().enumerate() {
        if direct_stack_matches(child, target_pane_id) {
            return Some(insert_direct_child(
                axis,
                children,
                sizes,
                index,
                direction,
                new_pane,
            ));
        }
        if contains_target(child, target_pane_id) {
            let mut next_children = children.to_vec();
            next_children[index] = insert_pane_by_edge(child, target_pane_id, direction, new_pane)?;
            return Some(TileTree::Split {
                axis,
                children: next_children,
                sizes: sizes.to_vec(),
            });
        }
    }
    None
}

fn insert_direct_child(
    axis: SplitAxis,
    children: &[TileTree],
    sizes: &[u16],
    index: usize,
    direction: SplitDirection,
    new_pane: TileTree,
) -> TileTree {
    let mut next_children = children.to_vec();
    let split_axis = edge_axis(direction);
    if axis == split_axis {
        let insert_at = if before_edge(direction) { index } else { index + 1 };
        next_children.insert(insert_at, new_pane);
        let count = next_children.len();
        return TileTree::Split {
            axis,
            children: next_children,
            sizes: equal_split_sizes(count),
        };
    }
    next_children[index] = split_for_direction(next_children[index].clone(), new_pane, direction);
    TileTree::Split {
        axis,
        children: next_children,
        sizes: sizes.to_vec(),
    }
}

fn split_for_direction(existing: TileTree, created: TileTree, direction: SplitDirection) -> TileTree {
    let axis = edge_axis(direction);
    let children = if before_edge(direction) {
        vec![created, existing]
    } else {
        vec![existing, created]
    };
    TileTree::Split {
        axis,
        children,
        sizes: equal_split_sizes(2),
    }
}

fn edge_axis(direction: SplitDirection) -> SplitAxis {
    match direction {
        SplitDirection::Left | SplitDirection::Right => SplitAxis::Row,
        SplitDirection::Top | SplitDirection::Bottom => SplitAxis::Column,
    }
}

fn before_edge(direction: SplitDirection) -> bool {
    matches!(direction, SplitDirection::Left | SplitDirection::Top)
}

fn direct_stack_matches(tile: &TileTree, target_pane_id: &NodeId) -> bool {
    matches!(
        tile,
        TileTree::Stack { tabs, .. } if tabs.iter().any(|tab| &tab.pane_node_id == target_pane_id)
    )
}

fn contains_target(tile: &TileTree, target_pane_id: &NodeId) -> bool {
    match tile {
        TileTree::Stack { tabs, .. } => tabs.iter().any(|tab| &tab.pane_node_id == target_pane_id),
        TileTree::Split { children, .. } => {
            children.iter().any(|child| contains_target(child, target_pane_id))
        }
    }
}
