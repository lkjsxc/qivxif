use super::{
    session::{auth_context, csrf_matches, load_session_user},
    support::{ApiResponse, auth_missing, capabilities, csrf_missing, fail, ok},
};
use crate::state::AppState;
use axum::{
    Json, Router,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use qivxif_api::{
    ApiErrorCode, OperationAcceptance, OperationRejection, PullRequest, PullResponse, PushRequest,
    PushResponse,
};
use qivxif_store_redb::{OperationReceipt, StoreError};
use qivxif_sync::{SyncLimits, validate_pull};

const MAX_SYNC_BATCH: usize = 128;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/sync/push", post(push))
        .route("/api/sync/pull", get(pull))
}

async fn push(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<PushRequest>,
) -> Response {
    let caps = sync_capabilities();
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing::<PushResponse>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return csrf_missing::<PushResponse>(caps).into_response();
    }
    if request.actor_id != session_user.user.actor_id {
        return auth_missing::<PushResponse>(caps).into_response();
    }
    if request.operations.len() > MAX_SYNC_BATCH {
        return batch_too_large::<PushResponse>(caps).into_response();
    }
    let auth = auth_context(&session_user);
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();
    for op in request.operations {
        match state.store.accept_operation(&auth, op.clone()) {
            Ok(receipt) => accepted.push(acceptance(receipt)),
            Err(error) => rejected.push(rejection(op.op_id, error)),
        }
    }
    let server_cursor = accepted.last().map(|item| item.server_cursor.clone());
    ok(
        PushResponse {
            accepted,
            rejected,
            server_cursor,
        },
        caps,
    )
    .into_response()
}

async fn pull(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(request): Query<PullRequest>,
) -> ApiResponse<PullResponse> {
    let caps = sync_capabilities();
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing(caps);
    };
    if validate_pull(request.clone(), limits()).is_err() {
        return batch_too_large(caps);
    }
    let auth = auth_context(&session_user);
    match state
        .store
        .list_operations_after_cursor(&auth, request.cursor.as_ref(), request.limit)
    {
        Ok((operations, server_cursor, has_more)) => ok(
            PullResponse {
                operations,
                server_cursor,
                has_more,
            },
            caps,
        ),
        Err(_) => fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorCode::StoreUnavailable,
            "store could not complete the request",
            caps,
        ),
    }
}

fn acceptance(receipt: OperationReceipt) -> OperationAcceptance {
    OperationAcceptance {
        op_id: receipt.op_id,
        server_cursor: receipt.server_cursor,
    }
}

fn rejection(op_id: qivxif_core::OperationId, error: StoreError) -> OperationRejection {
    let (code, message) = match error {
        StoreError::UnknownOperationKind => (
            "schema.unknown_operation_kind",
            "operation kind is not accepted",
        ),
        StoreError::InvalidOperation => ("operation.payload_hash_mismatch", "operation is invalid"),
        StoreError::Forbidden => ("auth.forbidden", "actor cannot apply operation"),
        StoreError::DuplicateActorSeq => (
            "operation.duplicate_actor_seq",
            "actor sequence belongs to another operation",
        ),
        StoreError::NodeMissing => ("graph.not_found", "target graph record is absent"),
        _ => ("store.conflict", "operation conflicts with durable state"),
    };
    OperationRejection {
        op_id,
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

fn batch_too_large<T>(caps: Vec<qivxif_core::Capability>) -> ApiResponse<T> {
    fail(
        StatusCode::BAD_REQUEST,
        ApiErrorCode::SyncBatchTooLarge,
        "sync batch exceeds server limit",
        caps,
    )
}

fn sync_capabilities() -> Vec<qivxif_core::Capability> {
    capabilities(&["sync.http_push_pull"])
}

fn limits() -> SyncLimits {
    SyncLimits {
        max_push_ops: MAX_SYNC_BATCH,
        max_pull_ops: MAX_SYNC_BATCH,
    }
}
