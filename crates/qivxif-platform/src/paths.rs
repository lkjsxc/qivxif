use std::{env, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("could not resolve current directory: {0}")]
    CurrentDir(std::io::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatePaths {
    pub root: PathBuf,
    pub settings_toml: PathBuf,
    pub workspace_json: PathBuf,
    pub recovery_dir: PathBuf,
}

impl StatePaths {
    pub fn resolve() -> Result<Self, PlatformError> {
        let root = match env::var_os("QIVXIF_STATE_DIR") {
            Some(value) => PathBuf::from(value),
            None => env::current_dir()
                .map_err(PlatformError::CurrentDir)?
                .join(".qivxif-state"),
        };
        Ok(Self {
            settings_toml: root.join("settings.toml"),
            workspace_json: root.join("workspace.json"),
            recovery_dir: root.join("recovery"),
            root,
        })
    }
}
