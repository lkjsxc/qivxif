use qivxif_core::NodeId;
use serde::{Deserialize, Serialize};

pub const MIN_PANE_WIDTH: u16 = 260;
pub const MIN_PANE_HEIGHT: u16 = 180;
pub const DEFAULT_SPLIT_WEIGHT: u16 = 500;

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
        children: Vec<TileTree>,
        sizes: Vec<u16>,
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

pub fn equal_split_sizes(count: usize) -> Vec<u16> {
    vec![DEFAULT_SPLIT_WEIGHT; count.max(1)]
}

fn tile_pane_count(tile: &TileTree) -> usize {
    match tile {
        TileTree::Split { children, .. } => children.iter().map(tile_pane_count).sum(),
        TileTree::Stack { tabs, .. } => tabs.len(),
    }
}
