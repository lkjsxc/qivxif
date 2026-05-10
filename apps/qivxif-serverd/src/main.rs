use anyhow::Result;
use clap::{Parser, Subcommand};
use qivxif_protocol::{ClientMsg, ServerMsg};
use qivxif_sim::Region;
use qivxif_storage::WorldStore;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};

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
    let store = Arc::new(WorldStore::open(PathBuf::from(&cfg.data_dir).as_path())?);
    let region = Arc::new(Region::new(cfg.world_seed, store));
    let state = Arc::new(AppState {
        build_epoch: cfg.build_epoch,
        protocol_epoch: cfg.protocol_epoch,
        region,
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
    region: Arc<Region>,
}

async fn handle_connection(incoming: quinn::Incoming, state: Arc<AppState>) {
    match incoming.await {
        Ok(connection) => handle_streams(connection, state).await,
        Err(error) => tracing::warn!(%error, "connection failed"),
    }
}

async fn handle_streams(connection: quinn::Connection, state: Arc<AppState>) {
    loop {
        match connection.accept_bi().await {
            Ok(stream) => tokio::spawn(handle_request(stream, state.clone())),
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
    state: Arc<AppState>,
) {
    let response = match qivxif_net::recv_json::<ClientMsg>(&mut recv).await {
        Ok(request) => respond(request, &state),
        Err(error) => ServerMsg::Error {
            code: "bad_request".to_string(),
            message: error.to_string(),
        },
    };
    if let Err(error) = qivxif_net::send_json(&mut send, &response).await {
        tracing::warn!(%error, "response send failed");
    }
}

fn respond(request: ClientMsg, state: &AppState) -> ServerMsg {
    match request {
        ClientMsg::Hello {
            build_epoch,
            protocol_epoch,
        } => hello(build_epoch, protocol_epoch, state),
        ClientMsg::JoinWorld { player } => ServerMsg::Joined { player },
        ClientMsg::Ping { nonce } => ServerMsg::Pong { nonce },
        ClientMsg::ChunkRequest { coord } => match state.region.chunk(coord) {
            Ok(cells) => ServerMsg::Chunk { coord, cells },
            Err(error) => error_msg("chunk_error", error),
        },
        ClientMsg::PlaceBlock { pos, block } => match state.region.place_block(pos, block) {
            Ok(cell) => ServerMsg::MutationAck {
                pos: cell.pos,
                block: cell.block,
            },
            Err(error) => error_msg("mutation_error", error),
        },
    }
}

fn hello(build_epoch: String, protocol_epoch: u32, state: &AppState) -> ServerMsg {
    if protocol_epoch != state.protocol_epoch {
        return ServerMsg::Error {
            code: "protocol_epoch_mismatch".to_string(),
            message: format!("expected {}", state.protocol_epoch),
        };
    }
    ServerMsg::HelloOk {
        world_epoch: format!("{}:{}", build_epoch, state.build_epoch),
    }
}

fn error_msg(code: &str, error: impl std::fmt::Display) -> ServerMsg {
    ServerMsg::Error {
        code: code.to_string(),
        message: error.to_string(),
    }
}
