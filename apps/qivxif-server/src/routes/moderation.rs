use super::{
    session::{auth_context, csrf_matches, load_session_user},
    support::{ApiResponse, auth_missing, capabilities, csrf_missing, fail, graph_store_error, ok},
};
use crate::state::AppState;
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
};
use qivxif_api::{
    ApiErrorCode, EventAcceptance, ModerationClearRequest, ModerationPayload, ModerationRequest,
};
use qivxif_store_redb::{
    EventReceipt, ModerationAction, ModerationClearInput, ModerationInput, ModerationResult,
    StoreError,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/social/mute", post(mute))
        .route("/api/social/unmute", post(unmute))
        .route("/api/social/block", post(block))
        .route("/api/social/unblock", post(unblock))
}

async fn mute(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<ModerationRequest>,
) -> Response {
    create_response(state, headers, request, ModerationAction::Mute).await
}

async fn block(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<ModerationRequest>,
) -> Response {
    create_response(state, headers, request, ModerationAction::Block).await
}

async fn unmute(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<ModerationClearRequest>,
) -> Response {
    clear_response(state, headers, request, ModerationAction::Mute).await
}

async fn unblock(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<ModerationClearRequest>,
) -> Response {
    clear_response(state, headers, request, ModerationAction::Block).await
}

async fn create_response(
    state: AppState,
    headers: HeaderMap,
    request: ModerationRequest,
    action: ModerationAction,
) -> Response {
    let caps = moderation_capabilities();
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing::<ModerationPayload>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return csrf_missing::<ModerationPayload>(caps).into_response();
    }
    let result = state.store.create_moderation_edge(
        &auth_context(&session_user),
        ModerationInput {
            event_id: request.event_id,
            actor_seq: request.actor_seq,
            edge_id: request.edge_id,
            actor_id: session_user.user.actor_id,
            actor_user_id: session_user.user.id,
            actor_profile_node_id: session_user.user.profile_node_id,
            target_profile_node_id: request.target_profile_node_id,
            action,
        },
    );
    moderation_response(result, caps).into_response()
}

async fn clear_response(
    state: AppState,
    headers: HeaderMap,
    request: ModerationClearRequest,
    action: ModerationAction,
) -> Response {
    let caps = moderation_capabilities();
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing::<ModerationPayload>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return csrf_missing::<ModerationPayload>(caps).into_response();
    }
    let result = state.store.clear_moderation_edge(
        &auth_context(&session_user),
        ModerationClearInput {
            event_id: request.event_id,
            actor_seq: request.actor_seq,
            edge_id: request.edge_id,
            actor_id: session_user.user.actor_id,
            actor_user_id: session_user.user.id,
            actor_profile_node_id: session_user.user.profile_node_id,
            action,
        },
    );
    moderation_response(result, caps).into_response()
}

fn moderation_response(
    result: qivxif_store_redb::StoreResult<ModerationResult>,
    caps: Vec<qivxif_core::Capability>,
) -> ApiResponse<ModerationPayload> {
    match result {
        Ok(result) => ok(
            ModerationPayload {
                edge: result.edge,
                event: acceptance(result.receipt),
            },
            caps,
        ),
        Err(StoreError::InvalidEvent) => fail(
            StatusCode::BAD_REQUEST,
            ApiErrorCode::StoreConflict,
            "moderation target is invalid",
            caps,
        ),
        Err(error) => graph_store_error(error, caps),
    }
}

fn acceptance(receipt: EventReceipt) -> EventAcceptance {
    EventAcceptance {
        event_id: receipt.event_id,
        server_cursor: receipt.server_cursor,
    }
}

fn moderation_capabilities() -> Vec<qivxif_core::Capability> {
    capabilities(&["social.moderation", "feed.home"])
}
