use qivxif_core::BlockPos;
use serde::{Deserialize, Serialize};

pub type RequestId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerCaps {
    pub reliable_streams: bool,
    pub datagrams: bool,
    pub persistent_mutations: bool,
}

pub const LOCAL_COMPOSE_CAPS: ServerCaps = ServerCaps {
    reliable_streams: true,
    datagrams: false,
    persistent_mutations: true,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockCell {
    pub pos: BlockPos,
    pub block: u16,
}
