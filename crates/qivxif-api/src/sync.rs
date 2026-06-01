use qivxif_core::{ActorId, CursorId, EventId};
use qivxif_history::EventEnvelope;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PushRequest {
    pub client_id: String,
    pub actor_id: ActorId,
    pub events: Vec<EventEnvelope>,
    pub cursor_summary: Option<CursorId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PushResponse {
    pub accepted: Vec<EventAcceptance>,
    pub rejected: Vec<EventRejection>,
    pub server_cursor: Option<CursorId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventAcceptance {
    pub event_id: EventId,
    pub server_cursor: CursorId,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventRejection {
    pub event_id: EventId,
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PullRequest {
    pub cursor: Option<CursorId>,
    pub limit: usize,
    pub scope: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PullResponse {
    pub events: Vec<EventEnvelope>,
    pub server_cursor: Option<CursorId>,
    pub has_more: bool,
}
