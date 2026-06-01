mod health;
mod server_info;
mod static_files;

use crate::state::AppState;
use axum::Router;
use tower_http::trace::TraceLayer;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(health::routes())
        .merge(server_info::routes())
        .fallback_service(static_files::service(&state.config.static_dir))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
