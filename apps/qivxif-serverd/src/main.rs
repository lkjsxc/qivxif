use anyhow::Result;
use clap::{Parser, Subcommand};
use qivxif_protocol::{ClientMsg, ServerMsg};
use qivxif_sim::RegionHandle;
use qivxif_storage::WorldStore;
use session::Session;
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

mod session;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Serve {
        #[arg(long, default_value = "config/server.toml")]
        config: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    match Cli::parse().command {
        Command::Serve { config } => {
            let cfg = qivxif_core::ServerConfig::load(&config)?;
            serve(cfg).await?;
        }
    }
    Ok(())
}

async fn serve(cfg: qivxif_core::ServerConfig) -> Result<()> {
    let store = Arc::new(WorldStore::open(
        PathBuf::from(&cfg.data_dir).as_path(),
        cfg.world_seed,
    )?);
    let state = Arc::new(AppState {
        build_epoch: cfg.build_epoch,
        protocol_epoch: cfg.protocol_epoch,
        world_epoch: store.meta().world_epoch.clone(),
        next_session: AtomicU64::new(1),
        region: RegionHandle::spawn(cfg.world_seed, store),
    });
    let addr: SocketAddr = cfg.bind_addr.parse()?;
    let endpoint = quinn::Endpoint::server(qivxif_net::server_config()?, addr)?;
    tracing::info!(bind_addr = %endpoint.local_addr()?, "server listening");
    loop {
        tokio::select! {
            incoming = endpoint.accept() => {
                if let Some(incoming) = incoming {
                    tokio::spawn(handle_connection(incoming, state.clone()));
                }
            }
            _ = tokio::signal::ctrl_c() => break,
        }
    }
    Ok(())
}

struct AppState {
    build_epoch: String,
    protocol_epoch: u32,
    world_epoch: String,
    next_session: AtomicU64,
    region: RegionHandle,
}

async fn handle_connection(incoming: quinn::Incoming, state: Arc<AppState>) {
    match incoming.await {
        Ok(connection) => handle_streams(connection, state).await,
        Err(error) => tracing::warn!(%error, "connection failed"),
    }
}

async fn handle_streams(connection: quinn::Connection, state: Arc<AppState>) {
    let mut session = Session::new(state.next_session.fetch_add(1, Ordering::Relaxed));
    loop {
        match connection.accept_bi().await {
            Ok(stream) => handle_request(stream, &state, &mut session).await,
            Err(quinn::ConnectionError::ApplicationClosed { .. }) => break,
            Err(error) => {
                tracing::warn!(%error, "stream accept failed");
                break;
            }
        };
    }
}

async fn handle_request(
    (mut send, mut recv): (quinn::SendStream, quinn::RecvStream),
    state: &AppState,
    session: &mut Session,
) {
    let response = match qivxif_net::recv_wire::<ClientMsg>(&mut recv).await {
        Ok(request) => respond(request, state, session).await,
        Err(error) => error_msg("bad_request", error),
    };
    if let Err(error) = qivxif_net::send_wire(&mut send, &response).await {
        tracing::warn!(%error, "response send failed");
    }
}

async fn respond(request: ClientMsg, state: &AppState, session: &mut Session) -> ServerMsg {
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

fn error_msg(code: &str, error: impl std::fmt::Display) -> ServerMsg {
    ServerMsg::Error {
        code: code.to_string(),
        message: error.to_string(),
    }
}
