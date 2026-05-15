use crate::{PaneId, SplitAxis, TileNode};

pub fn split_node(
    node: &mut TileNode,
    target: PaneId,
    pane: PaneId,
    axis: SplitAxis,
    ratio: f32,
) -> bool {
    match node {
        TileNode::Leaf { panes, active } if panes.contains(&target) => {
            let old = TileNode::Leaf {
                panes: panes.clone(),
                active: *active,
            };
            *node = TileNode::Split {
                axis,
                ratio,
                first: Box::new(old),
                second: Box::new(TileNode::Leaf {
                    panes: vec![pane],
                    active: 0,
                }),
            };
            true
        }
        TileNode::Leaf { .. } => false,
        TileNode::Split { first, second, .. } => {
            split_node(first, target, pane, axis, ratio)
                || split_node(second, target, pane, axis, ratio)
        }
    }
}

pub fn tab_node(node: &mut TileNode, target: PaneId, pane: PaneId) -> bool {
    match node {
        TileNode::Leaf { panes, active } if panes.contains(&target) => {
            panes.push(pane);
            *active = panes.len() - 1;
            true
        }
        TileNode::Leaf { .. } => false,
        TileNode::Split { first, second, .. } => {
            tab_node(first, target, pane) || tab_node(second, target, pane)
        }
    }
}

pub fn remove_pane(node: TileNode, pane: PaneId) -> (Option<TileNode>, bool) {
    match node {
        TileNode::Leaf { mut panes, active } => {
            let removed = panes.iter().any(|candidate| *candidate == pane);
            panes.retain(|candidate| *candidate != pane);
            let active = active.min(panes.len().saturating_sub(1));
            match panes.is_empty() {
                true => (None, removed),
                false => (Some(TileNode::Leaf { panes, active }), removed),
            }
        }
        TileNode::Split {
            axis,
            ratio,
            first,
            second,
        } => merge_split(
            axis,
            ratio,
            remove_pane(*first, pane),
            remove_pane(*second, pane),
        ),
    }
}

fn merge_split(
    axis: SplitAxis,
    ratio: f32,
    left: (Option<TileNode>, bool),
    right: (Option<TileNode>, bool),
) -> (Option<TileNode>, bool) {
    match (left, right) {
        ((Some(first), left), (Some(second), right)) => (
            Some(TileNode::Split {
                axis,
                ratio,
                first: Box::new(first),
                second: Box::new(second),
            }),
            left || right,
        ),
        ((Some(node), left), (None, right)) | ((None, left), (Some(node), right)) => {
            (Some(node), left || right)
        }
        ((None, left), (None, right)) => (None, left || right),
    }
}
