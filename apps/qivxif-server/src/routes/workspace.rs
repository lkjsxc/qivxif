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
use qivxif_api::{OperationAcceptance, WorkspaceLayoutPayload, WorkspaceLayoutSetRequest};
use qivxif_store_redb::{OperationReceipt, WorkspaceLayoutSetInput};

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/workspace/layout", post(set_layout))
}

async fn set_layout(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<WorkspaceLayoutSetRequest>,
) -> Response {
    let caps = workspace_capabilities();
    let Some(session_user) = load_session_user(&state, &headers) else {
        return auth_missing::<WorkspaceLayoutPayload>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return csrf_missing::<WorkspaceLayoutPayload>(caps).into_response();
    }
    let auth = auth_context(&session_user);
    let result = state.store.set_workspace_layout(
        &auth,
        WorkspaceLayoutSetInput {
            op_id: request.op_id,
            actor_seq: request.actor_seq,
            actor_id: session_user.user.actor_id,
            layout_node_id: request.layout_node_id,
            layout: request.layout,
        },
    );
    match result {
        Ok(result) => ok(
            WorkspaceLayoutPayload {
                layout_node: result.layout_node,
                operation: acceptance(result.receipt),
            },
            caps,
        )
        .into_response(),
        Err(error) => graph_store_error::<WorkspaceLayoutPayload>(error, caps).into_response(),
    }
}

fn acceptance(receipt: OperationReceipt) -> OperationAcceptance {
    OperationAcceptance {
        op_id: receipt.op_id,
        server_cursor: receipt.server_cursor,
    }
}

fn workspace_capabilities() -> Vec<qivxif_core::Capability> {
    capabilities(&["workspace.layout_set"])
}
