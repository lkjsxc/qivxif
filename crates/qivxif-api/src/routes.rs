use crate::OperationAcceptance;
use crate::ServerCapabilities;
use qivxif_core::{ActorId, EdgeId, MetadataMap, NodeId, OperationId, UserId, Visibility};
use qivxif_graph::{EdgeKind, EdgeRecord, NodeKind, NodeProjection, NodeRecord};
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
