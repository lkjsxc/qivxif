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
    for msg in client_messages() {
        assert_eq!(decode::<ClientMsg>(&encode(&msg).unwrap()).unwrap(), msg);
    }
}

#[test]
fn every_server_message_round_trips() {
    for msg in server_messages() {
        assert_eq!(decode::<ServerMsg>(&encode(&msg).unwrap()).unwrap(), msg);
    }
}

#[test]
fn every_error_code_round_trips_in_doc_order() {
    for (tag, code) in documented_error_codes().into_iter().enumerate() {
        assert_eq!(decode::<ErrorCode>(&encode(&code).unwrap()).unwrap(), code);
        assert_eq!(encode(&code).unwrap(), vec![tag as u8]);
    }
}

#[test]
fn current_protocol_contract_matches_docs() {
    assert_eq!(CURRENT_PROTOCOL_CONTRACT, "postcard-reliable-streams");
}

#[test]
fn postcard_client_message_envelopes_are_stable() {
    let expected = [
        (
            client_messages()[0].clone(),
            vec![
                0, 5, 112, 114, 111, 98, 101, 25, 112, 111, 115, 116, 99, 97, 114, 100, 45, 114,
                101, 108, 105, 97, 98, 108, 101, 45, 115, 116, 114, 101, 97, 109, 115,
            ],
        ),
        (
            client_messages()[1].clone(),
            vec![1, 5, 112, 114, 111, 98, 101],
        ),
        (ClientMsg::Ping { nonce: 42 }, vec![2, 42]),
    ];
    for (msg, bytes) in expected {
        assert_eq!(encode(&msg).unwrap(), bytes);
        assert_eq!(decode::<ClientMsg>(&bytes).unwrap(), msg);
    }
}

#[test]
fn invalid_postcard_envelope_is_bad_request_shape() {
    assert!(decode::<ClientMsg>(&[99]).is_err());
    assert!(decode::<ServerMsg>(&[99]).is_err());
    assert!(decode::<ErrorCode>(&[99]).is_err());
}

fn client_messages() -> [ClientMsg; 6] {
    [
        ClientMsg::Hello {
            build_contract: "probe".to_string(),
            protocol_contract: CURRENT_PROTOCOL_CONTRACT.to_string(),
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
    ]
}

fn server_messages() -> [ServerMsg; 7] {
    [
        ServerMsg::HelloOk {
            session_id: 7,
            world_id: "world-11".to_string(),
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
    ]
}

fn documented_error_codes() -> [ErrorCode; 8] {
    [
        ErrorCode::BadRequest,
        ErrorCode::BuildContractMissing,
        ErrorCode::ProtocolContractMismatch,
        ErrorCode::HelloRequired,
        ErrorCode::JoinRequired,
        ErrorCode::ChunkError,
        ErrorCode::MutationError,
        ErrorCode::FlushError,
    ]
}
