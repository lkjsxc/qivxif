use crate::ServerCapabilities;
use qivxif_core::{ActorId, NodeId, UserId};
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
