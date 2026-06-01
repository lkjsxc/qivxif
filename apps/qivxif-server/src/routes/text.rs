use super::{
    session::{auth_context, csrf_matches, load_session_user},
    support::{ApiResponse, auth_missing, capabilities, csrf_missing, fail, graph_not_found, ok},
};
use crate::state::AppState;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use qivxif_api::{
    ApiErrorCode, OperationAcceptance, TextOperationPayload, TextOperationRequest, TextPayload,
};
use qivxif_auth::AuthContext;
use qivxif_core::NodeId;
use qivxif_store_redb::{OperationReceipt, StoreError, TextApplyInput};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/text/{node_id}", get(get_text))
        .route("/api/text/{node_id}/ops", post(apply_text))
}

async fn get_text(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(node_id): Path<String>,
) -> ApiResponse<TextPayload> {
    let caps = text_capabilities();
    let Ok(node_id) = node_id.parse::<NodeId>() else {
        return invalid_id(caps);
    };
    let auth = viewer_context(&state, &headers);
    match state.store.get_text_state(&auth, &node_id) {
        Ok(Some(state)) => ok(TextPayload { state }, caps),
        Ok(None) => graph_not_found(caps),
        Err(error) => text_error(error, caps),
    }
}

async fn apply_text(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(node_id): Path<String>,
    Json(request): Json<TextOperationRequest>,
) -> Response {
    let caps = text_capabilities();
    let Ok(node_id) = node_id.parse::<NodeId>() else {
        return invalid_id::<TextOperationPayload>(caps).into_response();
    };
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing::<TextOperationPayload>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return csrf_missing::<TextOperationPayload>(caps).into_response();
    }
    let auth = auth_context(&session_user);
    let result = state.store.apply_text_operation(
        &auth,
        TextApplyInput {
            actor_id: session_user.user.actor_id,
            actor_seq: request.actor_seq,
            node_id,
            operation: request.operation,
        },
    );
    match result {
        Ok(result) => ok(
            TextOperationPayload {
                state: result.state,
                operation: acceptance(result.receipt),
            },
            caps,
        )
        .into_response(),
        Err(error) => text_error::<TextOperationPayload>(error, caps).into_response(),
    }
}

fn viewer_context(state: &AppState, headers: &HeaderMap) -> AuthContext {
    load_session_user(state, headers)
        .map(|session_user| auth_context(&session_user))
        .unwrap_or_else(AuthContext::public)
}

fn acceptance(receipt: OperationReceipt) -> OperationAcceptance {
    OperationAcceptance {
        op_id: receipt.op_id,
        server_cursor: receipt.server_cursor,
    }
}

fn invalid_id<T>(caps: Vec<qivxif_core::Capability>) -> ApiResponse<T> {
    fail(
        StatusCode::BAD_REQUEST,
        ApiErrorCode::SchemaInvalidId,
        "id is malformed",
        caps,
    )
}

fn text_error<T>(error: StoreError, caps: Vec<qivxif_core::Capability>) -> ApiResponse<T> {
    match error {
        StoreError::Forbidden => fail(
            StatusCode::FORBIDDEN,
            ApiErrorCode::AuthForbidden,
            "actor cannot access text node",
            caps,
        ),
        StoreError::NodeMissing => graph_not_found(caps),
        StoreError::InvalidOperation => fail(
            StatusCode::BAD_REQUEST,
            ApiErrorCode::TextInvalidRange,
            "text operation is invalid",
            caps,
        ),
        _ => fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorCode::StoreUnavailable,
            "store could not complete the request",
            caps,
        ),
    }
}

fn text_capabilities() -> Vec<qivxif_core::Capability> {
    capabilities(&["text.edit"])
}
