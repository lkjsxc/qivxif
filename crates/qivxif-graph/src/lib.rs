mod error;
mod kind;
mod projection;
mod record;
mod reducer;
mod workspace;

pub use error::{GraphError, GraphResult};
pub use kind::{EdgeKind, NodeKind};
pub use projection::{GraphProjection, NodeProjection, project_node};
pub use record::{AclRef, EdgeRecord, NodeRecord, Tombstone};
pub use reducer::{GraphOp, GraphState, apply_graph_op};
pub use workspace::{SplitAxis, WorkspaceLayout, WorkspaceTab, WorkspaceTile};
