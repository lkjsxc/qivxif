use crate::state::AppState;
use axum::{Json, Router, extract::State, routing::get};
use qivxif_api::{ApiEnvelope, HealthPayload};
use qivxif_core::{Capability, RequestId};

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

async fn health(State(state): State<AppState>) -> Json<ApiEnvelope<HealthPayload>> {
    let store_ok = state
        .store
        .health()
        .map(|health| health.ok)
        .unwrap_or(false);
    Json(ApiEnvelope::success(
        RequestId::generate(),
        HealthPayload {
            status: if store_ok { "ok" } else { "degraded" }.to_owned(),
            store_ok,
        },
        capabilities(),
    ))
}

fn capabilities() -> Vec<Capability> {
    ["server.health"]
        .into_iter()
        .filter_map(|value| value.parse().ok())
        .collect()
}
