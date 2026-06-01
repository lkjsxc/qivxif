use crate::{EventAcceptance, ServerCapabilities};
use qivxif_core::{ActorId, EdgeId, EventId, MetadataMap, NodeId, ServerTime, UserId, Visibility};
use qivxif_graph::TileLayout;
use qivxif_graph::{EdgeKind, EdgeRecord, GraphProjection, NodeKind, NodeProjection, NodeRecord};
use qivxif_history::{
    EventEnvelope, EventKind, EventScope, PayloadHash,
    text::{TextDocState, TextEvent},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HealthPayload {
    pub status: String,
    pub store_ok: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ServerInfoPayload {
    pub name: String,
    pub capabilities: ServerCapabilities,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UserSummary {
    pub user_id: UserId,
    pub actor_id: ActorId,
    pub name: String,
    pub roles: Vec<String>,
    pub profile_node_id: Option<NodeId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LoginPayload {
    pub user: UserSummary,
    pub csrf_token: String,
    pub next_actor_seq: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LogoutPayload {
    pub logged_out: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MePayload {
    pub user: UserSummary,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodeCreateRequest {
    pub event_id: EventId,
    pub actor_seq: u64,
    pub node_id: NodeId,
    pub kind: NodeKind,
    pub visibility: Visibility,
    pub metadata_map: MetadataMap,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodeCreatePayload {
    pub node: NodeRecord,
    pub event: EventAcceptance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodePayload {
    pub projection: NodeProjection,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodeHistoryPayload {
    pub node_id: NodeId,
    pub events: Vec<EventSummary>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventSummary {
    pub event_id: EventId,
    pub actor_id: ActorId,
    pub actor_seq: u64,
    pub scope: EventScope,
    pub kind: EventKind,
    pub target_node_ids: Vec<NodeId>,
    pub target_edge_ids: Vec<EdgeId>,
    pub target_event_ids: Vec<EventId>,
    pub payload_hash: PayloadHash,
    pub received_at_server: Option<ServerTime>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EdgeCreateRequest {
    pub event_id: EventId,
    pub actor_seq: u64,
    pub edge_id: EdgeId,
    pub from_node: NodeId,
    pub to_node: NodeId,
    pub kind: EdgeKind,
    pub metadata_map: MetadataMap,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EdgeCreatePayload {
    pub edge: EdgeRecord,
    pub event: EventAcceptance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EdgeListPayload {
    pub outgoing: Vec<EdgeRecord>,
    pub incoming: Vec<EdgeRecord>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NeighborhoodPayload {
    pub projection: GraphProjection,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextEventRequest {
    pub actor_seq: u64,
    pub event: TextEvent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextPayload {
    pub state: TextDocState,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextEventPayload {
    pub state: TextDocState,
    pub event: EventAcceptance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TileLayoutSetRequest {
    pub event_id: EventId,
    pub actor_seq: u64,
    pub layout_node_id: NodeId,
    pub layout: TileLayout,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TileLayoutPayload {
    pub layout_node: NodeRecord,
    pub event: EventAcceptance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublishRequest {
    pub event_id: EventId,
    pub actor_seq: u64,
    pub slug: String,
    pub summary: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublishPayload {
    pub post: NodeRecord,
    pub event: EventAcceptance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnpublishRequest {
    pub event_id: EventId,
    pub actor_seq: u64,
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicBlogPostPayload {
    pub title: String,
    pub author_name: String,
    pub slug: String,
    pub summary: String,
    pub html: String,
}

impl EventSummary {
    pub fn from_envelope(event: EventEnvelope) -> Self {
        Self {
            event_id: event.event_id,
            actor_id: event.actor_id,
            actor_seq: event.actor_seq,
            scope: event.scope,
            kind: event.kind,
            target_node_ids: event.target_node_ids,
            target_edge_ids: event.target_edge_ids,
            target_event_ids: event.target_event_ids,
            payload_hash: event.payload_hash,
            received_at_server: event.received_at_server,
        }
    }
}
