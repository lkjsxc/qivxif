mod auth;
mod graph;
mod health;
mod history;
mod neighborhood;
mod server_info;
mod session;
mod static_files;
mod support;
mod sync;
mod text;

use crate::state::AppState;
use axum::Router;
use tower_http::trace::TraceLayer;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(auth::routes())
        .merge(graph::routes())
        .merge(history::routes())
        .merge(health::routes())
        .merge(neighborhood::routes())
        .merge(server_info::routes())
        .merge(sync::routes())
        .merge(text::routes())
        .fallback_service(static_files::service(&state.config.static_dir))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
