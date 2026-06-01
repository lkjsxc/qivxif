use qivxif_core::{ActorId, SessionId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthRole {
    Owner,
    Admin,
    Member,
    Guest,
    Public,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Viewer {
    Public,
    Session {
        user_id: UserId,
        actor_id: ActorId,
        session_id: SessionId,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthContext {
    pub viewer: Viewer,
    pub roles: Vec<AuthRole>,
}

impl AuthContext {
    pub fn public() -> Self {
        Self {
            viewer: Viewer::Public,
            roles: vec![AuthRole::Public],
        }
    }

    pub fn user_id(&self) -> Option<&UserId> {
        match &self.viewer {
            Viewer::Public => None,
            Viewer::Session { user_id, .. } => Some(user_id),
        }
    }

    pub fn is_admin(&self) -> bool {
        self.roles.contains(&AuthRole::Admin)
    }
}
