use crate::PaneId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitAxis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TileNode {
    Leaf {
        panes: Vec<PaneId>,
        active: usize,
    },
    Split {
        axis: SplitAxis,
        ratio: f32,
        first: Box<TileNode>,
        second: Box<TileNode>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TileLayout {
    pub root: TileNode,
    pub focused: PaneId,
}

impl TileLayout {
    pub fn single(pane: PaneId) -> Self {
        Self {
            root: TileNode::Leaf {
                panes: vec![pane],
                active: 0,
            },
            focused: pane,
        }
    }

    pub fn split_focused(&mut self, pane: PaneId, axis: SplitAxis, ratio: f32) -> bool {
        let ratio = ratio.clamp(0.1, 0.9);
        let changed = split_node(&mut self.root, self.focused, pane, axis, ratio);
        if changed {
            self.focused = pane;
        }
        changed
    }

    pub fn focus(&mut self, pane: PaneId) -> bool {
        if self.root.contains(pane) {
            self.focused = pane;
            true
        } else {
            false
        }
    }

    pub fn close(&mut self, pane: PaneId) -> bool {
        let (node, removed) = remove_pane(self.root.clone(), pane);
        if removed && let Some(node) = node {
            self.root = node;
            if self.focused == pane {
                self.focused = self.root.first_pane().unwrap_or(pane);
            }
            return true;
        }
        false
    }
}

impl TileNode {
    pub fn contains(&self, pane: PaneId) -> bool {
        match self {
            TileNode::Leaf { panes, .. } => panes.contains(&pane),
            TileNode::Split { first, second, .. } => first.contains(pane) || second.contains(pane),
        }
    }

    pub fn first_pane(&self) -> Option<PaneId> {
        match self {
            TileNode::Leaf { panes, .. } => panes.first().copied(),
            TileNode::Split { first, .. } => first.first_pane(),
        }
    }
}

fn split_node(
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
            let new = TileNode::Leaf {
                panes: vec![pane],
                active: 0,
            };
            *node = TileNode::Split {
                axis,
                ratio,
                first: Box::new(old),
                second: Box::new(new),
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

fn remove_pane(node: TileNode, pane: PaneId) -> (Option<TileNode>, bool) {
    match node {
        TileNode::Leaf { mut panes, active } => {
            let before = panes.len();
            panes.retain(|candidate| *candidate != pane);
            let removed = before != panes.len();
            let active = active.min(panes.len().saturating_sub(1));
            if panes.is_empty() {
                (None, removed)
            } else {
                (Some(TileNode::Leaf { panes, active }), removed)
            }
        }
        TileNode::Split {
            axis,
            ratio,
            first,
            second,
        } => {
            let (first, left_removed) = remove_pane(*first, pane);
            let (second, right_removed) = remove_pane(*second, pane);
            match (first, second) {
                (Some(first), Some(second)) => (
                    Some(TileNode::Split {
                        axis,
                        ratio,
                        first: Box::new(first),
                        second: Box::new(second),
                    }),
                    left_removed || right_removed,
                ),
                (Some(node), None) | (None, Some(node)) => (Some(node), true),
                (None, None) => (None, true),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_focuses_new_pane_and_close_collapses() {
        let first = PaneId::from_raw(1);
        let second = PaneId::from_raw(2);
        let mut layout = TileLayout::single(first);
        assert!(layout.split_focused(second, SplitAxis::Vertical, 0.5));
        assert_eq!(layout.focused, second);
        assert!(layout.close(second));
        assert_eq!(layout.focused, first);
        assert!(matches!(layout.root, TileNode::Leaf { .. }));
    }
}
