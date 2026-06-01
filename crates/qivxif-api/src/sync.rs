use qivxif_core::{ActorId, CursorId, OperationId};
use qivxif_history::OperationEnvelope;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PushRequest {
    pub client_id: String,
    pub actor_id: ActorId,
    pub operations: Vec<OperationEnvelope>,
    pub cursor_summary: Option<CursorId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PushResponse {
    pub accepted: Vec<OperationAcceptance>,
    pub rejected: Vec<OperationRejection>,
    pub server_cursor: Option<CursorId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperationAcceptance {
    pub op_id: OperationId,
    pub server_cursor: CursorId,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperationRejection {
    pub op_id: OperationId,
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
    pub operations: Vec<OperationEnvelope>,
    pub server_cursor: Option<CursorId>,
    pub has_more: bool,
}
