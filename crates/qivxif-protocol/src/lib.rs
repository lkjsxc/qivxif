mod codec;
mod errors;
mod messages;
mod types;

pub use codec::{decode, encode};
pub use errors::ErrorCode;
pub use messages::{ClientMsg, ServerMsg};
pub use types::{BlockCell, LOCAL_COMPOSE_CAPS, RequestId, ServerCaps};

#[cfg(test)]
mod tests {
    use super::*;
    use qivxif_core::{BlockPos, ChunkCoord};

    fn cell() -> BlockCell {
        BlockCell {
            pos: BlockPos { x: 1, y: 2, z: 3 },
            block: 9,
        }
    }

    #[test]
    fn every_client_message_round_trips() {
        let messages = [
            ClientMsg::Hello {
                build_epoch: "probe".to_string(),
                protocol_epoch: 1,
            },
            ClientMsg::JoinWorld {
                player: "probe".to_string(),
            },
            ClientMsg::Ping { nonce: 42 },
            ClientMsg::ChunkRequest {
                coord: ChunkCoord { x: -1, z: 2 },
            },
            ClientMsg::PlaceBlock {
                request_id: 7,
                pos: cell().pos,
                block: 9,
            },
            ClientMsg::FlushPersistence { request_id: 8 },
        ];
        for msg in messages {
            assert_eq!(decode::<ClientMsg>(&encode(&msg).unwrap()).unwrap(), msg);
        }
    }

    #[test]
    fn every_server_message_round_trips() {
        let messages = [
            ServerMsg::HelloOk {
                session_id: 7,
                world_epoch: "world-11".to_string(),
                caps: LOCAL_COMPOSE_CAPS,
            },
            ServerMsg::Joined {
                player: "probe".to_string(),
            },
            ServerMsg::Pong { nonce: 42 },
            ServerMsg::Chunk {
                coord: ChunkCoord { x: -1, z: 2 },
                cells: vec![cell()],
            },
            ServerMsg::MutationAck {
                request_id: 9,
                cell: cell(),
            },
            ServerMsg::FlushAck { request_id: 10 },
            ServerMsg::Error {
                code: ErrorCode::BadRequest,
                message: "bad".to_string(),
            },
        ];
        for msg in messages {
            assert_eq!(decode::<ServerMsg>(&encode(&msg).unwrap()).unwrap(), msg);
        }
    }

    #[test]
    fn every_error_code_round_trips() {
        let codes = [
            ErrorCode::BadRequest,
            ErrorCode::BuildEpochMissing,
            ErrorCode::ProtocolEpochMismatch,
            ErrorCode::HelloRequired,
            ErrorCode::JoinRequired,
            ErrorCode::ChunkError,
            ErrorCode::MutationError,
            ErrorCode::FlushError,
        ];
        for code in codes {
            assert_eq!(decode::<ErrorCode>(&encode(&code).unwrap()).unwrap(), code);
        }
    }

    #[test]
    fn ping_wire_bytes_are_stable() {
        let bytes = encode(&ClientMsg::Ping { nonce: 42 }).unwrap();
        assert_eq!(bytes, vec![2, 42]);
    }
}
