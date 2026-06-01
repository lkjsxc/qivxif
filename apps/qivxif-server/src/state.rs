use crate::config::ServerConfig;
use qivxif_store_redb::QivxifStore;

#[derive(Clone)]
pub struct AppState {
    pub config: ServerConfig,
    pub store: QivxifStore,
}
