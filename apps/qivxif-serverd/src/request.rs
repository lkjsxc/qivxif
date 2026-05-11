use crate::{app::AppState, session::Session};
use qivxif_protocol::{ClientMsg, ErrorCode, LOCAL_COMPOSE_CAPS, ServerMsg};

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
                Ok(cells) => {
                    tracing::info!(
                        session_id = session.id,
                        cell_count = cells.len(),
                        "chunk request completed"
                    );
                    ServerMsg::Chunk { coord, cells }
                }
                Err(error) => {
                    tracing::warn!(session_id = session.id, code = ?ErrorCode::ChunkError, "chunk request rejected");
                    error_msg(ErrorCode::ChunkError, error)
                }
            }
        }
        ClientMsg::ChunkRequest { .. } => phase_error(ErrorCode::JoinRequired),
        ClientMsg::PlaceBlock {
            request_id,
            pos,
            block,
        } if session.can_play() => {
            if let Some(response) = session.replayed_response(request_id) {
                return response;
            }
            let response = match state.region.place_block(pos, block).await {
                Ok(cell) => ServerMsg::MutationAck { request_id, cell },
                Err(error) => error_msg(ErrorCode::MutationError, error),
            };
            match &response {
                ServerMsg::MutationAck { request_id, .. } => {
                    tracing::info!(session_id = session.id, request_id, "mutation accepted")
                }
                ServerMsg::Error { code, .. } => {
                    tracing::warn!(session_id = session.id, code = ?code, "mutation rejected");
                }
                _ => {}
            }
            session.remember_response(request_id, &response);
            response
        }
        ClientMsg::PlaceBlock { .. } => {
            tracing::warn!(
                session_id = session.id,
                code = ?ErrorCode::JoinRequired,
                "mutation rejected"
            );
            phase_error(ErrorCode::JoinRequired)
        }
        ClientMsg::FlushPersistence { request_id } if session.can_play() => {
            if let Some(response) = session.replayed_response(request_id) {
                return response;
            }
            match state.region.flush().await {
                Ok(()) => {
                    tracing::info!(session_id = session.id, request_id, "persistence flushed");
                    let response = ServerMsg::FlushAck { request_id };
                    session.remember_response(request_id, &response);
                    response
                }
                Err(error) => {
                    let response = error_msg(ErrorCode::FlushError, error);
                    session.remember_response(request_id, &response);
                    response
                }
            }
        }
        ClientMsg::FlushPersistence { .. } => phase_error(ErrorCode::JoinRequired),
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
        caps: LOCAL_COMPOSE_CAPS,
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
#[path = "request_tests.rs"]
mod request_tests;
