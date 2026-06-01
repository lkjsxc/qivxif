use qivxif_api::{PullRequest, PushRequest};
use qivxif_history::validate_event_envelope;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SyncLimits {
    pub max_push_events: usize,
    pub max_pull_events: usize,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SyncError {
    #[error("batch too large")]
    BatchTooLarge,
    #[error("invalid event")]
    InvalidEvent,
    #[error("cursor invalid")]
    CursorInvalid,
}

pub type SyncResult<T> = Result<T, SyncError>;

pub fn validate_push(request: PushRequest, limits: SyncLimits) -> SyncResult<PushRequest> {
    if request.events.len() > limits.max_push_events {
        return Err(SyncError::BatchTooLarge);
    }
    for event in request.events.iter().cloned() {
        validate_event_envelope(event).map_err(|_| SyncError::InvalidEvent)?;
    }
    Ok(request)
}

pub fn validate_pull(request: PullRequest, limits: SyncLimits) -> SyncResult<PullRequest> {
    if request.limit > limits.max_pull_events {
        return Err(SyncError::BatchTooLarge);
    }
    if request.scope.trim().is_empty() {
        return Err(SyncError::CursorInvalid);
    }
    Ok(request)
}
