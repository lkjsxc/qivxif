use super::{
    session::{auth_context, load_session_user},
    support::{ApiResponse, capabilities, fail, graph_not_found, invalid_id, ok},
};
use crate::state::AppState;
use axum::{
    Router,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    routing::get,
};
use qivxif_api::{ApiErrorCode, NeighborhoodPayload};
use qivxif_auth::AuthContext;
use qivxif_core::NodeId;
use qivxif_store_redb::StoreError;
use serde::Deserialize;

const DEFAULT_DEPTH: usize = 1;
const MAX_DEPTH: usize = 3;
const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 100;

#[derive(Deserialize)]
struct NeighborhoodQuery {
    node_id: String,
    depth: Option<usize>,
    limit: Option<usize>,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/graph/neighborhood", get(neighborhood))
}

async fn neighborhood(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<NeighborhoodQuery>,
) -> ApiResponse<NeighborhoodPayload> {
    let caps = neighborhood_capabilities();
    let Ok(node_id) = query.node_id.parse::<NodeId>() else {
        return invalid_id(caps);
    };
    let auth = viewer_context(&state, &headers);
    let depth = query.depth.unwrap_or(DEFAULT_DEPTH).min(MAX_DEPTH);
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);
    match state.store.get_neighborhood(&auth, &node_id, depth, limit) {
        Ok(projection) => ok(NeighborhoodPayload { projection }, caps),
        Err(StoreError::NodeMissing) => graph_not_found(caps),
        Err(StoreError::Forbidden) => fail(
            StatusCode::FORBIDDEN,
            ApiErrorCode::AuthForbidden,
            "actor cannot read graph neighborhood",
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

fn viewer_context(state: &AppState, headers: &HeaderMap) -> AuthContext {
    load_session_user(state, headers)
        .map(|session_user| auth_context(&session_user))
        .unwrap_or_else(AuthContext::public)
}

fn neighborhood_capabilities() -> Vec<qivxif_core::Capability> {
    capabilities(&["graph.neighborhood"])
}
