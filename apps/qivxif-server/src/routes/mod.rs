mod auth;
mod follow;
mod graph;
mod health;
mod history;
mod moderation;
mod neighborhood;
mod publish;
mod server_info;
mod session;
mod social;
mod static_files;
mod support;
mod sync;
mod text;
mod workspace;

use crate::state::AppState;
use axum::Router;
use tower_http::trace::TraceLayer;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(auth::routes())
        .merge(follow::routes())
        .merge(graph::routes())
        .merge(history::routes())
        .merge(health::routes())
        .merge(moderation::routes())
        .merge(neighborhood::routes())
        .merge(publish::routes())
        .merge(server_info::routes())
        .merge(social::routes())
        .merge(sync::routes())
        .merge(text::routes())
        .merge(workspace::routes())
        .fallback_service(static_files::service(&state.config.static_dir))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
