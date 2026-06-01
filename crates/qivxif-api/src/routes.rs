use crate::OperationAcceptance;
use crate::ServerCapabilities;
use qivxif_core::{
    ActorId, EdgeId, MetadataMap, NodeId, OperationId, ServerTime, UserId, Visibility,
};
use qivxif_graph::{EdgeKind, EdgeRecord, GraphProjection, NodeKind, NodeProjection, NodeRecord};
use qivxif_history::{
    OperationEnvelope, OperationKind, OperationScope, PayloadHash,
    text::{TextDocState, TextOperation},
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
    pub op_id: OperationId,
    pub actor_seq: u64,
    pub node_id: NodeId,
    pub kind: NodeKind,
    pub visibility: Visibility,
    pub metadata_map: MetadataMap,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodeCreatePayload {
    pub node: NodeRecord,
    pub operation: OperationAcceptance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodePayload {
    pub projection: NodeProjection,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodeHistoryPayload {
    pub node_id: NodeId,
    pub operations: Vec<OperationSummary>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperationSummary {
    pub op_id: OperationId,
    pub actor_id: ActorId,
    pub actor_seq: u64,
    pub scope: OperationScope,
    pub kind: OperationKind,
    pub target_node_ids: Vec<NodeId>,
    pub payload_hash: PayloadHash,
    pub received_at_server: Option<ServerTime>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EdgeCreateRequest {
    pub op_id: OperationId,
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
    pub operation: OperationAcceptance,
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
pub struct TextOperationRequest {
    pub actor_seq: u64,
    pub operation: TextOperation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextPayload {
    pub state: TextDocState,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextOperationPayload {
    pub state: TextDocState,
    pub operation: OperationAcceptance,
}

impl OperationSummary {
    pub fn from_envelope(op: OperationEnvelope) -> Self {
        Self {
            op_id: op.op_id,
            actor_id: op.actor_id,
            actor_seq: op.actor_seq,
            scope: op.scope,
            kind: op.kind,
            target_node_ids: op.target_node_ids,
            payload_hash: op.payload_hash,
            received_at_server: op.received_at_server,
        }
    }
}
