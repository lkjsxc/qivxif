use std::path::Path;
use tower_http::services::{ServeDir, ServeFile};

pub fn service(static_dir: &Path) -> ServeDir<ServeFile> {
    ServeDir::new(static_dir).fallback(ServeFile::new(static_dir.join("index.html")))
}
