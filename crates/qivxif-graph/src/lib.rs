mod error;
mod kind;
mod projection;
mod record;
mod reducer;
mod tile_commands;
mod tile_layout;
mod tree;

pub use error::{GraphError, GraphResult};
pub use kind::{EdgeKind, NodeKind};
pub use projection::{GraphProjection, NodeProjection, project_node};
pub use record::{AclRef, EdgeRecord, NodeRecord, Tombstone};
pub use reducer::{GraphEvent, GraphState, apply_graph_event};
pub use tile_commands::{
    SplitDirection, close_tab, focus_tab, maximize_pane, move_tab_to_edge, move_tab_to_stack,
    open_tab, resize_split_layout, restore_maximized, split_tab,
};
pub use tile_layout::{
    SplitAxis, TileLayout, TileTab, TileTree, equal_split_sizes, DEFAULT_SPLIT_WEIGHT,
    MIN_PANE_HEIGHT, MIN_PANE_WIDTH,
};
pub use tree::{TreeNode, TreeProjection, project_tree, project_tree_with_kinds};
