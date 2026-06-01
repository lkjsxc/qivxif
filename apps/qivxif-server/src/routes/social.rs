use super::{
    session::{auth_context, csrf_matches, load_session_user},
    support::{ApiResponse, auth_missing, capabilities, csrf_missing, fail, graph_store_error, ok},
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
    EventAcceptance, FeedHomePayload, FeedItemPayload, ShortPostPayload, ShortPostRequest,
};
use qivxif_store_redb::{EventReceipt, FeedItem, ShortPostInput, StoreError};
use serde::Deserialize;

#[derive(Deserialize)]
struct FeedQuery {
    limit: Option<usize>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/social/short-posts", post(create_short_post))
        .route("/api/feed/home", get(home_feed))
}

async fn create_short_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<ShortPostRequest>,
) -> Response {
    let caps = social_capabilities();
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing::<ShortPostPayload>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return csrf_missing::<ShortPostPayload>(caps).into_response();
    }
    let result = state.store.create_short_post(
        &auth_context(&session_user),
        ShortPostInput {
            event_id: request.event_id,
            actor_seq: request.actor_seq,
            node_id: request.node_id,
            actor_id: session_user.user.actor_id,
            author_user_id: session_user.user.id,
            author_name: session_user.user.name,
            body: request.body,
            visibility: request.visibility,
            reply_to: request.reply_to,
        },
    );
    short_post_response(result, caps).into_response()
}

async fn home_feed(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<FeedQuery>,
) -> ApiResponse<FeedHomePayload> {
    let caps = social_capabilities();
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing(caps);
    };
    match state
        .store
        .home_feed(&auth_context(&session_user), query.limit.unwrap_or(50))
    {
        Ok((items, cursor, has_more)) => ok(
            FeedHomePayload {
                items: items.into_iter().map(feed_payload).collect(),
                cursor,
                has_more,
            },
            caps,
        ),
        Err(error) => graph_store_error(error, caps),
    }
}

fn short_post_response(
    result: qivxif_store_redb::StoreResult<qivxif_store_redb::ShortPostResult>,
    caps: Vec<qivxif_core::Capability>,
) -> ApiResponse<ShortPostPayload> {
    match result {
        Ok(result) => ok(
            ShortPostPayload {
                post: result.post,
                feed_item: feed_payload(result.feed_item),
                event: acceptance(result.receipt),
            },
            caps,
        ),
        Err(StoreError::InvalidEvent) => fail(
            StatusCode::BAD_REQUEST,
            qivxif_api::ApiErrorCode::StoreConflict,
            "short post body or reply target is invalid",
            caps,
        ),
        Err(error) => graph_store_error(error, caps),
    }
}

fn feed_payload(item: FeedItem) -> FeedItemPayload {
    FeedItemPayload {
        event_id: item.event_id,
        post_node_id: item.post_node_id,
        author_user_id: item.author_user_id,
        author_name: item.author_name,
        body: item.body,
        visibility: item.visibility,
        created_at: item.created_at,
        reply_to: item.reply_to,
    }
}

fn acceptance(receipt: EventReceipt) -> EventAcceptance {
    EventAcceptance {
        event_id: receipt.event_id,
        server_cursor: receipt.server_cursor,
    }
}

fn social_capabilities() -> Vec<qivxif_core::Capability> {
    capabilities(&["social.short_post", "feed.home"])
}
