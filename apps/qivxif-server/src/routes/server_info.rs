use crate::state::AppState;
use axum::{Json, Router, extract::State, routing::get};
use qivxif_api::{ApiEnvelope, ServerCapabilities, ServerInfoPayload};
use qivxif_core::{Capability, RequestId};

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/server-info", get(server_info))
}

async fn server_info(State(_state): State<AppState>) -> Json<ApiEnvelope<ServerInfoPayload>> {
    let capabilities = capabilities();
    Json(ApiEnvelope::success(
        RequestId::generate(),
        ServerInfoPayload {
            name: "qivxif".to_owned(),
            capabilities: ServerCapabilities {
                capabilities: capabilities.clone(),
                max_sync_batch: 128,
            },
        },
        capabilities,
    ))
}

fn capabilities() -> Vec<Capability> {
    [
        "server.health",
        "graph.node_create",
        "graph.edge_create",
        "graph.neighborhood",
        "publish.blog",
        "social.short_post",
        "social.follow",
        "social.moderation",
        "feed.home",
        "sync.http_push_pull",
        "client.offline_shell",
        "setup.owner_create",
        "workspace.layout_set",
    ]
    .into_iter()
    .filter_map(|value| value.parse().ok())
    .collect()
}
