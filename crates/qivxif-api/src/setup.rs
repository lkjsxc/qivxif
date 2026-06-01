use crate::UserSummary;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetupStatusPayload {
    pub required: bool,
    pub owner_creation_open: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetupOwnerRequest {
    pub name: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetupOwnerPayload {
    pub user: UserSummary,
    pub csrf_token: String,
    pub next_actor_seq: u64,
}
