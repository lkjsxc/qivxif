use crate::{app::AppState, session::Session};
use qivxif_protocol::{ClientMsg, ServerMsg};

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
        ClientMsg::JoinWorld { .. } => phase_error("hello_required"),
        ClientMsg::Ping { nonce } if session.can_ping() => ServerMsg::Pong { nonce },
        ClientMsg::Ping { .. } => phase_error("hello_required"),
        ClientMsg::ChunkRequest { coord } if session.can_play() => {
            match state.region.chunk(coord).await {
                Ok(cells) => ServerMsg::Chunk { coord, cells },
                Err(error) => error_msg("chunk_error", error),
            }
        }
        ClientMsg::ChunkRequest { .. } => phase_error("join_required"),
        ClientMsg::PlaceBlock { pos, block } if session.can_play() => {
            match state.region.place_block(pos, block).await {
                Ok(cell) => ServerMsg::MutationAck {
                    pos: cell.pos,
                    block: cell.block,
                },
                Err(error) => error_msg("mutation_error", error),
            }
        }
        ClientMsg::PlaceBlock { .. } => phase_error("join_required"),
    }
}

fn hello(
    build_epoch: String,
    protocol_epoch: u32,
    state: &AppState,
    session: &mut Session,
) -> ServerMsg {
    if build_epoch.is_empty() || state.build_epoch.is_empty() {
        return error_msg("build_epoch_missing", "build epoch must not be empty");
    }
    if protocol_epoch != state.protocol_epoch {
        return error_msg(
            "protocol_epoch_mismatch",
            format!("expected {}", state.protocol_epoch),
        );
    }
    session.mark_hello();
    ServerMsg::HelloOk {
        session_id: session.id,
        world_epoch: state.world_epoch.clone(),
    }
}

fn phase_error(code: &str) -> ServerMsg {
    ServerMsg::Error {
        code: code.to_string(),
        message: "request is not valid in the current session phase".to_string(),
    }
}

pub fn error_msg(code: &str, error: impl std::fmt::Display) -> ServerMsg {
    ServerMsg::Error {
        code: code.to_string(),
        message: error.to_string(),
    }
}
