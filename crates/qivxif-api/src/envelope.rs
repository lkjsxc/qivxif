use qivxif_core::{Capability, RequestId, ServerTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApiEnvelope<T> {
    pub request_id: RequestId,
    pub server_time: ServerTime,
    pub capabilities: Vec<Capability>,
    pub payload: Option<T>,
    pub error: Option<ApiError>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApiError {
    pub code: ApiErrorCode,
    pub message: String,
    pub field_errors: Vec<FieldError>,
    pub retry: Option<RetryInfo>,
    pub conflict: Option<ConflictInfo>,
    pub required_capability: Option<Capability>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ApiErrorCode {
    #[serde(rename = "auth.invalid_credentials")]
    AuthInvalidCredentials,
    #[serde(rename = "auth.session_missing")]
    AuthSessionMissing,
    #[serde(rename = "auth.csrf_missing")]
    AuthCsrfMissing,
    #[serde(rename = "auth.forbidden")]
    AuthForbidden,
    #[serde(rename = "graph.not_found")]
    GraphNotFound,
    #[serde(rename = "schema.invalid_id")]
    SchemaInvalidId,
    #[serde(rename = "schema.invalid_input")]
    SchemaInvalidInput,
    #[serde(rename = "schema.unknown_node_kind")]
    SchemaUnknownNodeKind,
    #[serde(rename = "schema.unknown_edge_kind")]
    SchemaUnknownEdgeKind,
    #[serde(rename = "schema.unknown_operation_kind")]
    SchemaUnknownOperationKind,
    #[serde(rename = "operation.duplicate_actor_seq")]
    OperationDuplicateActorSeq,
    #[serde(rename = "operation.payload_hash_mismatch")]
    OperationPayloadHashMismatch,
    #[serde(rename = "operation.missing_parent")]
    OperationMissingParent,
    #[serde(rename = "store.conflict")]
    StoreConflict,
    #[serde(rename = "store.unavailable")]
    StoreUnavailable,
    #[serde(rename = "sync.batch_too_large")]
    SyncBatchTooLarge,
    #[serde(rename = "sync.cursor_invalid")]
    SyncCursorInvalid,
    #[serde(rename = "publish.slug_conflict")]
    PublishSlugConflict,
    #[serde(rename = "text.invalid_range")]
    TextInvalidRange,
    #[serde(rename = "cache.quota_exceeded")]
    CacheQuotaExceeded,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RetryInfo {
    pub retryable: bool,
    pub after_millis: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ConflictInfo {
    pub target: String,
    pub current: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ServerCapabilities {
    pub capabilities: Vec<Capability>,
    pub max_sync_batch: usize,
}

impl<T> ApiEnvelope<T> {
    pub fn success(request_id: RequestId, payload: T, capabilities: Vec<Capability>) -> Self {
        Self {
            request_id,
            server_time: ServerTime::now(),
            capabilities,
            payload: Some(payload),
            error: None,
        }
    }

    pub fn failure(request_id: RequestId, error: ApiError, capabilities: Vec<Capability>) -> Self {
        Self {
            request_id,
            server_time: ServerTime::now(),
            capabilities,
            payload: None,
            error: Some(error),
        }
    }
}
