use crate::{HistoryError, HistoryResult};
use qivxif_core::{ActorId, ClientTime, EdgeId, EventId, NodeId, ServerTime};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PayloadHash(String);

impl PayloadHash {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventPayload {
    pub bytes: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventScope {
    Auth,
    Graph,
    Text,
    Tile,
    Sync,
    Publish,
    Social,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum EventKind {
    #[serde(rename = "user.bootstrap_owner")]
    UserBootstrapOwner,
    #[serde(rename = "auth.session_created")]
    AuthSessionCreated,
    #[serde(rename = "node.create")]
    NodeCreate,
    #[serde(rename = "node.update_metadata")]
    NodeUpdateMetadata,
    #[serde(rename = "node.tombstone")]
    NodeTombstone,
    #[serde(rename = "edge.create")]
    EdgeCreate,
    #[serde(rename = "edge.tombstone")]
    EdgeTombstone,
    #[serde(rename = "edge.relate")]
    EdgeRelate,
    #[serde(rename = "text.create_doc")]
    TextCreateDoc,
    #[serde(rename = "text.insert")]
    TextInsert,
    #[serde(rename = "text.delete")]
    TextDelete,
    #[serde(rename = "text.restore")]
    TextRestore,
    #[serde(rename = "tile.layout_set")]
    TileLayoutSet,
    #[serde(rename = "graph_map.item_place")]
    GraphMapItemPlace,
    #[serde(rename = "sync.cursor_advance")]
    SyncCursorAdvance,
    #[serde(rename = "publish.post")]
    PublishPost,
    #[serde(rename = "publish.unpublish")]
    PublishUnpublish,
    #[serde(rename = "social.short_post_create")]
    SocialShortPostCreate,
    #[serde(rename = "social.follow")]
    SocialFollow,
    #[serde(rename = "social.unfollow")]
    SocialUnfollow,
    #[serde(rename = "social.mute")]
    SocialMute,
    #[serde(rename = "social.unmute")]
    SocialUnmute,
    #[serde(rename = "social.block")]
    SocialBlock,
    #[serde(rename = "social.unblock")]
    SocialUnblock,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventEnvelope {
    pub event_id: EventId,
    pub actor_id: ActorId,
    pub actor_seq: u64,
    pub parents: Vec<EventId>,
    pub scope: EventScope,
    pub kind: EventKind,
    pub target_node_ids: Vec<NodeId>,
    pub target_edge_ids: Vec<EdgeId>,
    pub target_event_ids: Vec<EventId>,
    pub payload: EventPayload,
    pub payload_hash: PayloadHash,
    pub created_at_client: Option<ClientTime>,
    pub received_at_server: Option<ServerTime>,
    pub auth_context: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedEvent(pub EventEnvelope);

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TargetSet {
    pub nodes: BTreeSet<NodeId>,
    pub edges: BTreeSet<EdgeId>,
    pub events: BTreeSet<EventId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayCursor {
    pub after_event: Option<EventId>,
}

pub fn hash_payload(bytes: &[u8]) -> PayloadHash {
    PayloadHash(blake3::hash(bytes).to_hex().to_string())
}

pub fn validate_event_envelope(event: EventEnvelope) -> HistoryResult<ValidatedEvent> {
    if event.actor_seq == 0 {
        return Err(HistoryError::InvalidActorSeq);
    }
    if event.payload_hash.as_str().is_empty() {
        return Err(HistoryError::MissingPayloadHash);
    }
    if event.payload_hash != hash_payload(&event.payload.bytes) {
        return Err(HistoryError::PayloadHashMismatch);
    }
    Ok(ValidatedEvent(event))
}

pub fn event_targets(event: &EventEnvelope) -> TargetSet {
    TargetSet {
        nodes: event.target_node_ids.iter().cloned().collect(),
        edges: event.target_edge_ids.iter().cloned().collect(),
        events: event.target_event_ids.iter().cloned().collect(),
    }
}

impl ReplayCursor {
    pub fn from_event(event_id: EventId) -> Self {
        Self {
            after_event: Some(event_id),
        }
    }
}

#[cfg(test)]
mod tests;
