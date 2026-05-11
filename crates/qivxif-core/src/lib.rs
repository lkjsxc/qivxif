use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

const WORLD_SCHEMA_EPOCH: u32 = 1;

/// Server bootstrap config loaded from TOML.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub data_dir: String,
    pub world_seed: u64,
    pub build_epoch: String,
    pub protocol_epoch: u32,
}

impl ServerConfig {
    /// Load a config file from disk.
    pub fn load(path: &Path) -> Result<Self, CoreError> {
        let raw = fs::read_to_string(path)?;
        Ok(toml::from_str(&raw)?)
    }
}

/// Absolute block position in the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Horizontal chunk coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoord {
    pub x: i32,
    pub z: i32,
}

/// Persisted world metadata stored under the `world` key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldMeta {
    pub schema_epoch: u32,
    pub world_seed: u64,
    pub world_epoch: String,
}

impl WorldMeta {
    /// Build the current world metadata for a seed.
    pub fn new(world_seed: u64) -> Self {
        Self {
            schema_epoch: WORLD_SCHEMA_EPOCH,
            world_seed,
            world_epoch: format!("world-{world_seed}"),
        }
    }
}

/// Core error for IO and config parsing.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_meta_matches_contract() {
        let meta = WorldMeta::new(11);
        assert_eq!(meta.schema_epoch, 1);
        assert_eq!(meta.world_seed, 11);
        assert_eq!(meta.world_epoch, "world-11");
    }

    #[test]
    fn server_config_loads_toml() {
        let path =
            std::env::temp_dir().join(format!("qivxif-core-config-{}.toml", std::process::id()));
        fs::write(
            &path,
            "bind_addr = \"127.0.0.1:3000\"\ndata_dir = \"/tmp/world\"\nworld_seed = 7\nbuild_epoch = \"test\"\nprotocol_epoch = 1\n",
        )
        .unwrap();

        let config = ServerConfig::load(&path).unwrap();
        assert_eq!(config.bind_addr, "127.0.0.1:3000");
        assert_eq!(config.data_dir, "/tmp/world");
        assert_eq!(config.world_seed, 7);
        assert_eq!(config.build_epoch, "test");
        assert_eq!(config.protocol_epoch, 1);

        fs::remove_file(&path).unwrap();
    }
}
