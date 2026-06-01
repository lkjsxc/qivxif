use crate::EventAcceptance;
use qivxif_core::{EdgeId, EventId, NodeId, ServerTime, UserId, Visibility};
use qivxif_graph::{EdgeRecord, NodeRecord};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ShortPostRequest {
    pub event_id: EventId,
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
    pub event: EventAcceptance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FollowRequest {
    pub event_id: EventId,
    pub actor_seq: u64,
    pub edge_id: EdgeId,
    pub target_profile_node_id: NodeId,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnfollowRequest {
    pub event_id: EventId,
    pub actor_seq: u64,
    pub edge_id: EdgeId,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FollowPayload {
    pub edge: EdgeRecord,
    pub event: EventAcceptance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModerationRequest {
    pub event_id: EventId,
    pub actor_seq: u64,
    pub edge_id: EdgeId,
    pub target_profile_node_id: NodeId,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModerationClearRequest {
    pub event_id: EventId,
    pub actor_seq: u64,
    pub edge_id: EdgeId,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModerationPayload {
    pub edge: EdgeRecord,
    pub event: EventAcceptance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FeedHomePayload {
    pub items: Vec<FeedItemPayload>,
    pub cursor: Option<EventId>,
    pub has_more: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FeedItemPayload {
    pub event_id: EventId,
    pub post_node_id: NodeId,
    pub author_user_id: UserId,
    pub author_name: String,
    pub body: String,
    pub visibility: Visibility,
    pub created_at: ServerTime,
    pub reply_to: Option<NodeId>,
}
