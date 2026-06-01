use super::{
    session::{auth_context, csrf_matches, load_session_user},
    support::{auth_missing, capabilities, csrf_missing, graph_store_error, ok},
};
use crate::state::AppState;
use axum::{
    Json, Router,
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
    routing::post,
};
use qivxif_api::{OperationAcceptance, TileLayoutPayload, TileLayoutSetRequest};
use qivxif_store_redb::{OperationReceipt, TileLayoutSetInput};

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/tile-layout", post(set_layout))
}

async fn set_layout(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<TileLayoutSetRequest>,
) -> Response {
    let caps = tile_capabilities();
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing::<TileLayoutPayload>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return csrf_missing::<TileLayoutPayload>(caps).into_response();
    }
    let auth = auth_context(&session_user);
    let result = state.store.set_tile_layout(
        &auth,
        TileLayoutSetInput {
            op_id: request.op_id,
            actor_seq: request.actor_seq,
            actor_id: session_user.user.actor_id,
            layout_node_id: request.layout_node_id,
            layout: request.layout,
        },
    );
    match result {
        Ok(result) => ok(
            TileLayoutPayload {
                layout_node: result.layout_node,
                operation: acceptance(result.receipt),
            },
            caps,
        )
        .into_response(),
        Err(error) => graph_store_error::<TileLayoutPayload>(error, caps).into_response(),
    }
}

fn acceptance(receipt: OperationReceipt) -> OperationAcceptance {
    OperationAcceptance {
        op_id: receipt.op_id,
        server_cursor: receipt.server_cursor,
    }
}

fn tile_capabilities() -> Vec<qivxif_core::Capability> {
    capabilities(&["tile.layout_set"])
}
