use super::{
    session::{auth_context, load_session_user},
    support::{ApiResponse, capabilities, fail, graph_not_found, invalid_id, ok},
};
use crate::state::AppState;
use axum::{
    Router,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    routing::get,
};
use qivxif_api::{ApiErrorCode, EventSummary, NodeHistoryPayload};
use qivxif_auth::AuthContext;
use qivxif_core::NodeId;
use qivxif_store_redb::StoreError;
use serde::Deserialize;

const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 200;

#[derive(Deserialize)]
struct HistoryQuery {
    limit: Option<usize>,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/nodes/{node_id}/history", get(node_history))
}

async fn node_history(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(node_id): Path<String>,
    Query(query): Query<HistoryQuery>,
) -> ApiResponse<NodeHistoryPayload> {
    let caps = history_capabilities();
    let Ok(node_id) = node_id.parse::<NodeId>() else {
        return invalid_id(caps);
    };
    let auth = viewer_context(&state, &headers);
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);
    match state.store.list_events_for_node(&auth, &node_id, limit) {
        Ok(events) => ok(
            NodeHistoryPayload {
                node_id,
                events: events
                    .into_iter()
                    .map(EventSummary::from_envelope)
                    .collect(),
            },
            caps,
        ),
        Err(StoreError::NodeMissing) => graph_not_found(caps),
        Err(StoreError::Forbidden) => fail(
            StatusCode::FORBIDDEN,
            ApiErrorCode::AuthForbidden,
            "actor cannot read node history",
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

fn history_capabilities() -> Vec<qivxif_core::Capability> {
    capabilities(&["history.inspect"])
}
