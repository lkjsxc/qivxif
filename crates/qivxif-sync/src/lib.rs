use qivxif_api::{PullRequest, PushRequest};
use qivxif_history::validate_operation_envelope;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SyncLimits {
    pub max_push_ops: usize,
    pub max_pull_ops: usize,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SyncError {
    #[error("batch too large")]
    BatchTooLarge,
    #[error("invalid operation")]
    InvalidOperation,
    #[error("cursor invalid")]
    CursorInvalid,
}

pub type SyncResult<T> = Result<T, SyncError>;

pub fn validate_push(request: PushRequest, limits: SyncLimits) -> SyncResult<PushRequest> {
    if request.operations.len() > limits.max_push_ops {
        return Err(SyncError::BatchTooLarge);
    }
    for op in request.operations.iter().cloned() {
        validate_operation_envelope(op).map_err(|_| SyncError::InvalidOperation)?;
    }
    Ok(request)
}

pub fn validate_pull(request: PullRequest, limits: SyncLimits) -> SyncResult<PullRequest> {
    if request.limit > limits.max_pull_ops {
        return Err(SyncError::BatchTooLarge);
    }
    if request.scope.trim().is_empty() {
        return Err(SyncError::CursorInvalid);
    }
    Ok(request)
}
