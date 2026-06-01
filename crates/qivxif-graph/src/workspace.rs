use qivxif_core::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorkspaceLayout {
    pub root: WorkspaceTile,
    pub maximized_pane_id: Option<NodeId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WorkspaceTile {
    Split {
        axis: SplitAxis,
        ratio_percent: u8,
        first: Box<WorkspaceTile>,
        second: Box<WorkspaceTile>,
    },
    Stack {
        active: usize,
        tabs: Vec<WorkspaceTab>,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SplitAxis {
    Column,
    Row,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorkspaceTab {
    pub pane_node_id: NodeId,
    pub pane_kind: String,
    pub target_node_id: Option<NodeId>,
    pub title: String,
}

impl WorkspaceLayout {
    pub fn pane_count(&self) -> usize {
        tile_pane_count(&self.root)
    }
}

fn tile_pane_count(tile: &WorkspaceTile) -> usize {
    match tile {
        WorkspaceTile::Split { first, second, .. } => {
            tile_pane_count(first) + tile_pane_count(second)
        }
        WorkspaceTile::Stack { tabs, .. } => tabs.len(),
    }
}
