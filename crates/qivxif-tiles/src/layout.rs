use crate::PaneId;
use crate::ops::{remove_pane, split_node, tab_node};
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
    pub maximized: Option<PaneId>,
}

impl TileLayout {
    pub fn single(pane: PaneId) -> Self {
        Self {
            root: TileNode::Leaf {
                panes: vec![pane],
                active: 0,
            },
            focused: pane,
            maximized: None,
        }
    }

    pub fn split_focused(&mut self, pane: PaneId, axis: SplitAxis, ratio: f32) -> bool {
        let changed = split_node(
            &mut self.root,
            self.focused,
            pane,
            axis,
            ratio.clamp(0.1, 0.9),
        );
        if changed {
            self.focused = pane;
            self.maximized = None;
        }
        changed
    }

    pub fn tab_focused(&mut self, pane: PaneId) -> bool {
        let changed = tab_node(&mut self.root, self.focused, pane);
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
        let before = self.panes();
        let (node, removed) = remove_pane(self.root.clone(), pane);
        if !removed {
            return false;
        }
        if let Some(node) = node {
            self.root = node;
            if self.focused == pane {
                self.focused = next_focus(&before, &self.panes(), pane).unwrap_or(pane);
            }
        }
        if self.maximized == Some(pane) {
            self.maximized = None;
        }
        true
    }

    pub fn toggle_maximize(&mut self) {
        self.maximized = (self.maximized != Some(self.focused)).then_some(self.focused);
    }

    pub fn panes(&self) -> Vec<PaneId> {
        let mut panes = Vec::new();
        self.root.collect_panes(&mut panes);
        panes
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

    pub fn active_pane(&self) -> Option<PaneId> {
        match self {
            TileNode::Leaf { panes, active } => panes.get(*active).copied(),
            TileNode::Split { first, .. } => first.active_pane(),
        }
    }

    fn collect_panes(&self, out: &mut Vec<PaneId>) {
        match self {
            TileNode::Leaf { panes, .. } => out.extend(panes.iter().copied()),
            TileNode::Split { first, second, .. } => {
                first.collect_panes(out);
                second.collect_panes(out);
            }
        }
    }
}

fn next_focus(before: &[PaneId], after: &[PaneId], closed: PaneId) -> Option<PaneId> {
    let index = before.iter().position(|pane| *pane == closed)?;
    before[..index]
        .iter()
        .rev()
        .chain(before[index + 1..].iter())
        .copied()
        .find(|pane| after.contains(pane))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_tab_close_and_maximize_keep_focus_valid() {
        let first = PaneId::from_raw(1);
        let second = PaneId::from_raw(2);
        let third = PaneId::from_raw(3);
        let mut layout = TileLayout::single(first);
        assert!(layout.split_focused(second, SplitAxis::Vertical, 0.5));
        assert!(layout.tab_focused(third));
        layout.toggle_maximize();
        assert_eq!(layout.maximized, Some(third));
        assert!(layout.close(third));
        assert_eq!(layout.focused, second);
        assert_eq!(layout.maximized, None);
    }
}
