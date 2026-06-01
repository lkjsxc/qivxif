use crate::{EdgeKind, NodeKind};
use qivxif_core::{
    ActorId, CommitGroupId, EdgeId, MetadataMap, NodeId, ServerTime, TextDocId, UserId, Visibility,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AclRef(pub String);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Tombstone {
    pub by: ActorId,
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodeRecord {
    pub id: NodeId,
    pub kind: NodeKind,
    pub owner_user_id: UserId,
    pub created_by: ActorId,
    pub created_at: ServerTime,
    pub updated_at: ServerTime,
    pub visibility: Visibility,
    pub acl_ref: Option<AclRef>,
    pub current_commit_group: Option<CommitGroupId>,
    pub current_text_ref: Option<TextDocId>,
    pub metadata_map: MetadataMap,
    pub tombstone: Option<Tombstone>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EdgeRecord {
    pub id: EdgeId,
    pub from_node: NodeId,
    pub to_node: NodeId,
    pub kind: EdgeKind,
    pub created_by: ActorId,
    pub created_at: ServerTime,
    pub metadata_map: MetadataMap,
    pub tombstone: Option<Tombstone>,
}
