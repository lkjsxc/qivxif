use crate::OperationAcceptance;
use qivxif_core::{NodeId, OperationId, ServerTime, UserId, Visibility};
use qivxif_graph::NodeRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ShortPostRequest {
    pub op_id: OperationId,
    pub actor_seq: u64,
    pub node_id: NodeId,
    pub body: String,
    pub visibility: Visibility,
    pub reply_to: Option<NodeId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ShortPostPayload {
    pub post: NodeRecord,
    pub feed_item: FeedItemPayload,
    pub operation: OperationAcceptance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FeedHomePayload {
    pub items: Vec<FeedItemPayload>,
    pub cursor: Option<OperationId>,
    pub has_more: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FeedItemPayload {
    pub operation_id: OperationId,
    pub post_node_id: NodeId,
    pub author_user_id: UserId,
    pub author_name: String,
    pub body: String,
    pub visibility: Visibility,
    pub created_at: ServerTime,
    pub reply_to: Option<NodeId>,
}
