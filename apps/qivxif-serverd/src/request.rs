use crate::{app::AppState, session::Session};
use qivxif_protocol::{ClientMsg, ErrorCode, ServerCaps, ServerMsg};

pub async fn respond(request: ClientMsg, state: &AppState, session: &mut Session) -> ServerMsg {
    match request {
        ClientMsg::Hello {
            build_epoch,
            protocol_epoch,
        } => hello(build_epoch, protocol_epoch, state, session),
        ClientMsg::JoinWorld { player } if session.can_join() => {
            session.mark_joined();
            ServerMsg::Joined { player }
        }
        ClientMsg::JoinWorld { .. } => phase_error(ErrorCode::HelloRequired),
        ClientMsg::Ping { nonce } if session.can_ping() => ServerMsg::Pong { nonce },
        ClientMsg::Ping { .. } => phase_error(ErrorCode::HelloRequired),
        ClientMsg::ChunkRequest { coord } if session.can_play() => {
            match state.region.chunk(coord).await {
                Ok(cells) => ServerMsg::Chunk { coord, cells },
                Err(error) => error_msg(ErrorCode::ChunkError, error),
            }
        }
        ClientMsg::ChunkRequest { .. } => phase_error(ErrorCode::JoinRequired),
        ClientMsg::PlaceBlock {
            request_id,
            pos,
            block,
        } if session.can_play() => match state.region.place_block(pos, block).await {
            Ok(cell) => ServerMsg::MutationAck { request_id, cell },
            Err(error) => error_msg(ErrorCode::MutationError, error),
        },
        ClientMsg::PlaceBlock { .. } => phase_error(ErrorCode::JoinRequired),
    }
}

fn hello(
    build_epoch: String,
    protocol_epoch: u32,
    state: &AppState,
    session: &mut Session,
) -> ServerMsg {
    if build_epoch.is_empty() || state.build_epoch.is_empty() {
        return error_msg(
            ErrorCode::BuildEpochMissing,
            "build epoch must not be empty",
        );
    }
    if protocol_epoch != state.protocol_epoch {
        return error_msg(ErrorCode::ProtocolEpochMismatch, state.protocol_epoch);
    }
    session.mark_hello();
    ServerMsg::HelloOk {
        session_id: session.id,
        world_epoch: state.world_epoch.clone(),
        caps: ServerCaps {
            reliable_streams: true,
            datagrams: false,
            persistent_mutations: true,
        },
    }
}

fn phase_error(code: ErrorCode) -> ServerMsg {
    ServerMsg::Error {
        code,
        message: "request is not valid in the current session phase".to_string(),
    }
}

pub fn error_msg(code: ErrorCode, error: impl std::fmt::Display) -> ServerMsg {
    ServerMsg::Error {
        code,
        message: error.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qivxif_sim::RegionHandle;
    use qivxif_storage::WorldStore;
    use std::sync::{Arc, atomic::AtomicU64};

    fn test_state() -> (tempfile::TempDir, AppState) {
        let root = tempfile::tempdir().unwrap();
        let store = Arc::new(WorldStore::open(root.path(), 5).unwrap());
        let world_epoch = store.meta().world_epoch.clone();
        let state = AppState {
            build_epoch: "test".to_string(),
            protocol_epoch: 1,
            world_epoch,
            next_session: AtomicU64::new(1),
            region: RegionHandle::spawn(5, store),
        };
        (root, state)
    }

    #[tokio::test]
    async fn join_before_hello_is_rejected() {
        let (_root, state) = test_state();
        let mut session = Session::new(1);
        let msg = respond(
            ClientMsg::JoinWorld {
                player: "probe".to_string(),
            },
            &state,
            &mut session,
        )
        .await;
        assert!(matches!(
            msg,
            ServerMsg::Error {
                code: ErrorCode::HelloRequired,
                ..
            }
        ));
    }

    #[tokio::test]
    async fn protocol_epoch_mismatch_is_rejected() {
        let (_root, state) = test_state();
        let mut session = Session::new(1);
        let msg = respond(
            ClientMsg::Hello {
                build_epoch: "test".to_string(),
                protocol_epoch: 99,
            },
            &state,
            &mut session,
        )
        .await;
        assert!(matches!(
            msg,
            ServerMsg::Error {
                code: ErrorCode::ProtocolEpochMismatch,
                ..
            }
        ));
    }
}
