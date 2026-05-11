use qivxif_core::{BlockPos, ChunkCoord};
use serde::{Deserialize, Serialize};

pub type RequestId = u64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
        request_id: RequestId,
        pos: BlockPos,
        block: u16,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerMsg {
    HelloOk {
        session_id: u64,
        world_epoch: String,
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
    Error {
        code: ErrorCode,
        message: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerCaps {
    pub reliable_streams: bool,
    pub datagrams: bool,
    pub persistent_mutations: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    BadRequest,
    BuildEpochMissing,
    ProtocolEpochMismatch,
    HelloRequired,
    JoinRequired,
    ChunkError,
    MutationError,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockCell {
    pub pos: BlockPos,
    pub block: u16,
}

pub fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, postcard::Error> {
    postcard::to_stdvec(value)
}

pub fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, postcard::Error> {
    postcard::from_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_wire_bytes_are_stable() {
        let bytes = encode(&ClientMsg::Ping { nonce: 42 }).unwrap();
        assert_eq!(bytes, vec![2, 42]);
        assert_eq!(
            decode::<ClientMsg>(&bytes).unwrap(),
            ClientMsg::Ping { nonce: 42 }
        );
    }

    #[test]
    fn hello_ok_round_trips() {
        let msg = ServerMsg::HelloOk {
            session_id: 7,
            world_epoch: "world-11".to_string(),
            caps: ServerCaps {
                reliable_streams: true,
                datagrams: false,
                persistent_mutations: true,
            },
        };
        assert_eq!(decode::<ServerMsg>(&encode(&msg).unwrap()).unwrap(), msg);
    }

    #[test]
    fn mutation_ack_round_trips_with_request_id() {
        let cell = BlockCell {
            pos: BlockPos { x: 1, y: 2, z: 3 },
            block: 9,
        };
        let msg = ServerMsg::MutationAck {
            request_id: 99,
            cell,
        };
        assert_eq!(decode::<ServerMsg>(&encode(&msg).unwrap()).unwrap(), msg);
    }
}
