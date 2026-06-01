use qivxif_core::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TileLayout {
    pub root: TileTree,
    pub maximized_pane_id: Option<NodeId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TileTree {
    Split {
        axis: SplitAxis,
        ratio_percent: u8,
        first: Box<TileTree>,
        second: Box<TileTree>,
    },
    Stack {
        active: usize,
        tabs: Vec<TileTab>,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SplitAxis {
    Column,
    Row,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TileTab {
    pub pane_node_id: NodeId,
    pub pane_kind: String,
    pub target_node_id: Option<NodeId>,
    pub title: String,
}

impl TileLayout {
    pub fn pane_count(&self) -> usize {
        tile_pane_count(&self.root)
    }
}

fn tile_pane_count(tile: &TileTree) -> usize {
    match tile {
        TileTree::Split { first, second, .. } => tile_pane_count(first) + tile_pane_count(second),
        TileTree::Stack { tabs, .. } => tabs.len(),
    }
}
