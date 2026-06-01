use super::{
    session::{auth_context, csrf_matches, load_session_user},
    support::{
        ApiResponse, auth_missing, capabilities, csrf_missing, graph_not_found, graph_store_error,
        invalid_id, ok,
    },
};
use crate::state::AppState;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use qivxif_api::{
    EdgeCreatePayload, EdgeCreateRequest, EdgeListPayload, EventAcceptance, NodeCreatePayload,
    NodeCreateRequest, NodePayload,
};
use qivxif_auth::AuthContext;
use qivxif_core::NodeId;
use qivxif_store_redb::{EdgeCreateInput, EventReceipt, NodeCreateInput};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/nodes", post(create_node))
        .route("/api/nodes/{node_id}", get(get_node))
        .route("/api/nodes/{node_id}/edges", get(list_edges))
        .route("/api/edges", post(create_edge))
}

async fn create_node(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<NodeCreateRequest>,
) -> Response {
    let caps = graph_capabilities();
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing::<NodeCreatePayload>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return csrf_missing::<NodeCreatePayload>(caps).into_response();
    }
    let result = state.store.create_node(NodeCreateInput {
        event_id: request.event_id,
        actor_seq: request.actor_seq,
        node_id: request.node_id,
        owner_user_id: session_user.user.id,
        actor_id: session_user.user.actor_id,
        kind: request.kind,
        visibility: request.visibility,
        metadata_map: request.metadata_map,
    });
    match result {
        Ok(result) => ok(
            NodeCreatePayload {
                node: result.node,
                event: acceptance(result.receipt),
            },
            caps,
        )
        .into_response(),
        Err(error) => graph_store_error::<NodeCreatePayload>(error, caps).into_response(),
    }
}

async fn create_edge(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<EdgeCreateRequest>,
) -> Response {
    let caps = graph_capabilities();
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing::<EdgeCreatePayload>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return csrf_missing::<EdgeCreatePayload>(caps).into_response();
    }
    let auth = auth_context(&session_user);
    let result = state.store.create_edge(
        &auth,
        EdgeCreateInput {
            event_id: request.event_id,
            actor_seq: request.actor_seq,
            edge_id: request.edge_id,
            from_node: request.from_node,
            to_node: request.to_node,
            actor_id: session_user.user.actor_id,
            kind: request.kind,
            metadata_map: request.metadata_map,
        },
    );
    match result {
        Ok(result) => ok(
            EdgeCreatePayload {
                edge: result.edge,
                event: acceptance(result.receipt),
            },
            caps,
        )
        .into_response(),
        Err(error) => graph_store_error::<EdgeCreatePayload>(error, caps).into_response(),
    }
}

async fn get_node(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(node_id): Path<String>,
) -> ApiResponse<NodePayload> {
    let caps = graph_capabilities();
    let Ok(node_id) = node_id.parse::<NodeId>() else {
        return invalid_id(caps);
    };
    let auth = viewer_context(&state, &headers);
    match state.store.get_node_projection(&auth, &node_id) {
        Ok(Some(projection)) => ok(NodePayload { projection }, caps),
        Ok(None) => graph_not_found(caps),
        Err(error) => graph_store_error(error, caps),
    }
}

async fn list_edges(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(node_id): Path<String>,
) -> ApiResponse<EdgeListPayload> {
    let caps = graph_capabilities();
    let Ok(node_id) = node_id.parse::<NodeId>() else {
        return invalid_id(caps);
    };
    let auth = viewer_context(&state, &headers);
    match state.store.get_node_projection(&auth, &node_id) {
        Ok(Some(projection)) => ok(
            EdgeListPayload {
                outgoing: projection.outgoing,
                incoming: projection.incoming,
            },
            caps,
        ),
        Ok(None) => graph_not_found(caps),
        Err(error) => graph_store_error(error, caps),
    }
}

fn viewer_context(state: &AppState, headers: &HeaderMap) -> AuthContext {
    load_session_user(state, headers)
        .map(|session_user| auth_context(&session_user))
        .unwrap_or_else(AuthContext::public)
}

fn acceptance(receipt: EventReceipt) -> EventAcceptance {
    EventAcceptance {
        event_id: receipt.event_id,
        server_cursor: receipt.server_cursor,
    }
}

fn graph_capabilities() -> Vec<qivxif_core::Capability> {
    capabilities(&["graph.node_create", "graph.edge_create"])
}
