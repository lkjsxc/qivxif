use std::{env, net::SocketAddr, path::PathBuf};

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub bind: SocketAddr,
    pub data_dir: PathBuf,
    pub database_file: PathBuf,
    pub static_dir: PathBuf,
    pub cookie_secure: bool,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let data_dir = env_path("QIVXIF_DATA_DIR", "data");
        Self {
            bind: env::var("QIVXIF_BIND")
                .unwrap_or_else(|_| "127.0.0.1:8080".to_owned())
                .parse()
                .expect("QIVXIF_BIND must be a socket address"),
            database_file: env_path("QIVXIF_DATABASE_FILE", data_dir.join("qivxif.redb")),
            static_dir: env_path("QIVXIF_STATIC_DIR", "apps/qivxif-web/dist"),
            cookie_secure: env_bool("QIVXIF_COOKIE_SECURE", false),
            data_dir,
        }
    }
}

fn env_path(name: &str, fallback: impl Into<PathBuf>) -> PathBuf {
    env::var(name)
        .map(PathBuf::from)
        .unwrap_or_else(|_| fallback.into())
}

fn env_bool(name: &str, fallback: bool) -> bool {
    env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(fallback)
}
