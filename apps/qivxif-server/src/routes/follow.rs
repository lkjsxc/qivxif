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
use qivxif_api::{ApiErrorCode, EventAcceptance, FollowPayload, FollowRequest, UnfollowRequest};
use qivxif_store_redb::{EventReceipt, FollowInput, FollowResult, StoreError, UnfollowInput};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/social/follow", post(follow_profile))
        .route("/api/social/unfollow", post(unfollow_profile))
}

async fn follow_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<FollowRequest>,
) -> Response {
    let caps = follow_capabilities();
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing::<FollowPayload>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return csrf_missing::<FollowPayload>(caps).into_response();
    }
    let result = state.store.follow_profile(
        &auth_context(&session_user),
        FollowInput {
            event_id: request.event_id,
            actor_seq: request.actor_seq,
            edge_id: request.edge_id,
            actor_id: session_user.user.actor_id,
            follower_user_id: session_user.user.id,
            follower_profile_node_id: session_user.user.profile_node_id,
            target_profile_node_id: request.target_profile_node_id,
        },
    );
    follow_response(result, caps).into_response()
}

async fn unfollow_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<UnfollowRequest>,
) -> Response {
    let caps = follow_capabilities();
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing::<FollowPayload>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return csrf_missing::<FollowPayload>(caps).into_response();
    }
    let result = state.store.unfollow_profile(
        &auth_context(&session_user),
        UnfollowInput {
            event_id: request.event_id,
            actor_seq: request.actor_seq,
            edge_id: request.edge_id,
            actor_id: session_user.user.actor_id,
            follower_user_id: session_user.user.id,
            follower_profile_node_id: session_user.user.profile_node_id,
        },
    );
    follow_response(result, caps).into_response()
}

fn follow_response(
    result: qivxif_store_redb::StoreResult<FollowResult>,
    caps: Vec<qivxif_core::Capability>,
) -> ApiResponse<FollowPayload> {
    match result {
        Ok(result) => ok(
            FollowPayload {
                edge: result.edge,
                event: acceptance(result.receipt),
            },
            caps,
        ),
        Err(StoreError::InvalidEvent) => fail(
            StatusCode::BAD_REQUEST,
            ApiErrorCode::StoreConflict,
            "follow target is invalid",
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

fn follow_capabilities() -> Vec<qivxif_core::Capability> {
    capabilities(&["social.follow", "feed.home"])
}
