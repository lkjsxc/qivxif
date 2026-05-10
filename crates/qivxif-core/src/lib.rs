use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub data_dir: String,
    pub world_seed: u64,
    pub build_epoch: String,
    pub protocol_epoch: u32,
}

impl ServerConfig {
    pub fn load(path: &Path) -> Result<Self, CoreError> {
        let raw = fs::read_to_string(path)?;
        Ok(toml::from_str(&raw)?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoord {
    pub x: i32,
    pub z: i32,
}

#[derive(Debug, Error)]
pub enum CoreError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}
