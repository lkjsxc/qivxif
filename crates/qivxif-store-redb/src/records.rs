use qivxif_auth::{AuthRole, PasswordHashString};
use qivxif_core::{ActorId, CursorId, EventId, NodeId, SessionId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StoredUser {
    pub id: UserId,
    pub actor_id: ActorId,
    pub profile_node_id: NodeId,
    pub name: String,
    pub password_hash: PasswordHashString,
    pub roles: Vec<AuthRole>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StoredSession {
    pub id: SessionId,
    pub user_id: UserId,
    pub actor_id: ActorId,
    pub csrf_token_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventReceipt {
    pub event_id: EventId,
    pub server_cursor: CursorId,
}
