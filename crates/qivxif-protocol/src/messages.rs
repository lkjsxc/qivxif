use crate::{BlockCell, ErrorCode, RequestId, ServerCaps};
use qivxif_core::{BlockPos, ChunkCoord};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientMsg {
    Hello {
        build_contract: String,
        protocol_contract: String,
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
        request_id: RequestId,
        pos: BlockPos,
        block: u16,
    },
    FlushPersistence {
        request_id: RequestId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerMsg {
    HelloOk {
        session_id: u64,
        world_id: String,
        caps: ServerCaps,
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
        request_id: RequestId,
        cell: BlockCell,
    },
    FlushAck {
        request_id: RequestId,
    },
    Error {
        code: ErrorCode,
        message: String,
    },
}
