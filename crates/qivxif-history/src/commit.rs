use qivxif_core::{ActorId, CommitGroupId, NodeId, OperationId, ServerTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitGroupKind {
    BootstrapAdmin,
    CreateNode,
    EditText,
    LinkNodes,
    PublishPost,
    WorkspaceChange,
    SocialAction,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommitGroup {
    pub id: CommitGroupId,
    pub actor_id: ActorId,
    pub kind: CommitGroupKind,
    pub parent_groups: Vec<CommitGroupId>,
    pub operations: Vec<OperationId>,
    pub target_node_ids: Vec<NodeId>,
    pub created_at: ServerTime,
}
