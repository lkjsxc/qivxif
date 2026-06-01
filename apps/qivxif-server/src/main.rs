use anyhow::Result;
use qivxif_server::{config::ServerConfig, routes, state::AppState};
use qivxif_store_redb::{StoreConfig, open_or_create};
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<()> {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();
    let config = ServerConfig::from_env();
    let store = open_or_create(StoreConfig::new(&config.database_file))?;
    let state = AppState { config, store };
    let listener = TcpListener::bind(&state.config.bind).await?;
    tracing::info!(
        bind = %state.config.bind,
        data_dir = %state.config.data_dir.display(),
        cookie_secure = state.config.cookie_secure,
        "qivxif server listening"
    );
    axum::serve(listener, routes::router(state)).await?;
    Ok(())
}
