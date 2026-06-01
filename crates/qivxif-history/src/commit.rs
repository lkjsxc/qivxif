use qivxif_core::{ActorId, CommitGroupId, EventId, NodeId, ServerTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitGroupKind {
    BootstrapOwner,
    CreateNode,
    EditText,
    LinkNodes,
    PublishPost,
    TileChange,
    SocialAction,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommitGroup {
    pub id: CommitGroupId,
    pub actor_id: ActorId,
    pub kind: CommitGroupKind,
    pub parent_groups: Vec<CommitGroupId>,
    pub events: Vec<EventId>,
    pub target_node_ids: Vec<NodeId>,
    pub created_at: ServerTime,
}
