use super::{
    session::{auth_context, csrf_matches, load_session_user},
    support::{auth_missing, capabilities, csrf_missing, fail, graph_not_found, ok},
};
use crate::{markdown, state::AppState};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use qivxif_api::{
    ApiErrorCode, OperationAcceptance, PublishPayload, PublishRequest, UnpublishRequest,
};
use qivxif_core::NodeId;
use qivxif_store_redb::{OperationReceipt, PublishPostInput, StoreError, UnpublishPostInput};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/publish/{node_id}", post(publish_post))
        .route("/api/unpublish/{node_id}", post(unpublish_post))
        .route("/@{author}/{slug}", get(public_post))
}

async fn publish_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(node_id): Path<String>,
    Json(request): Json<PublishRequest>,
) -> Response {
    let caps = publish_capabilities();
    let Ok(post_node_id) = node_id.parse::<NodeId>() else {
        return invalid_publish_id(caps).into_response();
    };
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing::<PublishPayload>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return csrf_missing::<PublishPayload>(caps).into_response();
    }
    let result = state.store.publish_post(
        &auth_context(&session_user),
        PublishPostInput {
            op_id: request.op_id,
            actor_seq: request.actor_seq,
            actor_id: session_user.user.actor_id,
            post_node_id,
            author_name: session_user.user.name,
            slug: request.slug,
            summary: request.summary,
        },
    );
    publish_response(result, caps).into_response()
}

async fn unpublish_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(node_id): Path<String>,
    Json(request): Json<UnpublishRequest>,
) -> Response {
    let caps = publish_capabilities();
    let Ok(post_node_id) = node_id.parse::<NodeId>() else {
        return invalid_publish_id(caps).into_response();
    };
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing::<PublishPayload>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return csrf_missing::<PublishPayload>(caps).into_response();
    }
    let result = state.store.unpublish_post(
        &auth_context(&session_user),
        UnpublishPostInput {
            op_id: request.op_id,
            actor_seq: request.actor_seq,
            actor_id: session_user.user.actor_id,
            post_node_id,
            reason: request.reason,
        },
    );
    publish_response(result, caps).into_response()
}

async fn public_post(
    State(state): State<AppState>,
    Path((author, slug)): Path<(String, String)>,
) -> Response {
    let author = author.trim_start_matches('@');
    match state.store.public_blog_post(author, &slug) {
        Ok(Some(post)) => {
            let title = post.title.clone();
            let body = markdown::render_markdown(&post.body);
            Html(markdown::html_page(&title, &body)).into_response()
        }
        Ok(None) | Err(StoreError::NodeMissing) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

fn publish_response(
    result: qivxif_store_redb::StoreResult<qivxif_store_redb::PublishPostResult>,
    caps: Vec<qivxif_core::Capability>,
) -> super::support::ApiResponse<PublishPayload> {
    match result {
        Ok(result) => ok(
            PublishPayload {
                post: result.post,
                operation: acceptance(result.receipt),
            },
            caps,
        ),
        Err(error) => publish_error(error, caps),
    }
}

fn acceptance(receipt: OperationReceipt) -> OperationAcceptance {
    OperationAcceptance {
        op_id: receipt.op_id,
        server_cursor: receipt.server_cursor,
    }
}

fn publish_error(
    error: StoreError,
    caps: Vec<qivxif_core::Capability>,
) -> super::support::ApiResponse<PublishPayload> {
    match error {
        StoreError::SlugConflict => fail(
            StatusCode::CONFLICT,
            ApiErrorCode::PublishSlugConflict,
            "slug already belongs to a published post",
            caps,
        ),
        StoreError::Forbidden => fail(
            StatusCode::FORBIDDEN,
            ApiErrorCode::AuthForbidden,
            "actor cannot publish this node",
            caps,
        ),
        StoreError::NodeMissing => graph_not_found(caps),
        StoreError::InvalidOperation => fail(
            StatusCode::BAD_REQUEST,
            ApiErrorCode::StoreConflict,
            "publish request does not match post shape",
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

fn invalid_publish_id(
    caps: Vec<qivxif_core::Capability>,
) -> super::support::ApiResponse<PublishPayload> {
    fail(
        StatusCode::BAD_REQUEST,
        ApiErrorCode::SchemaInvalidId,
        "id is malformed",
        caps,
    )
}

fn publish_capabilities() -> Vec<qivxif_core::Capability> {
    capabilities(&["publish.blog"])
}
