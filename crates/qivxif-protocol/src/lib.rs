use qivxif_core::{BlockPos, ChunkCoord};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMsg {
    Hello {
        build_epoch: String,
        protocol_epoch: u32,
    },
    JoinWorld {
        player: String,
    },
    Ping {
        nonce: u64,
    },
    ChunkRequest {
        coord: ChunkCoord,
    },
    PlaceBlock {
        pos: BlockPos,
        block: u16,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMsg {
    HelloOk {
        world_epoch: String,
    },
    Joined {
        player: String,
    },
    Pong {
        nonce: u64,
    },
    Chunk {
        coord: ChunkCoord,
        cells: Vec<BlockCell>,
    },
    MutationAck {
        pos: BlockPos,
        block: u16,
    },
    Error {
        code: String,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockCell {
    pub pos: BlockPos,
    pub block: u16,
}
