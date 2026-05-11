use crate::{request, session::Session};
use anyhow::Result;
use qivxif_protocol::{ClientMsg, ErrorCode};
use qivxif_sim::RegionHandle;
use qivxif_storage::WorldStore;
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

pub async fn serve(config: PathBuf) -> Result<()> {
    let cfg = qivxif_core::ServerConfig::load(&config)?;
    tracing::info!(
        config = %config.display(),
        bind_addr = %cfg.bind_addr,
        data_dir = %cfg.data_dir,
        build_epoch = %cfg.build_epoch,
        protocol_epoch = cfg.protocol_epoch,
        "server starting"
    );

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
    tracing::info!("server shutdown");
    Ok(())
}

pub(crate) struct AppState {
    pub(crate) build_epoch: String,
    pub(crate) protocol_epoch: u32,
    pub(crate) world_epoch: String,
    pub(crate) next_session: AtomicU64,
    pub(crate) region: RegionHandle,
}

async fn handle_connection(incoming: quinn::Incoming, state: Arc<AppState>) {
    match incoming.await {
        Ok(connection) => {
            tracing::info!("connection accepted");
            tracing::debug!(remote = %connection.remote_address(), "connection remote address");
            handle_streams(connection, state).await;
        }
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
    tracing::info!(session_id = session.id, "connection closed");
}

async fn handle_request(
    (mut send, mut recv): (quinn::SendStream, quinn::RecvStream),
    state: &AppState,
    session: &mut Session,
) {
    let response = match qivxif_net::recv_wire::<ClientMsg>(&mut recv).await {
        Ok(request) => {
            let name = request_name(&request);
            let response = request::respond(request, state, session).await;
            tracing::info!(session_id = session.id, request = name, "request handled");
            response
        }
        Err(error) => request::error_msg(ErrorCode::BadRequest, error),
    };
    if let Err(error) = qivxif_net::send_wire(&mut send, &response).await {
        tracing::warn!(%error, "response send failed");
    }
}

fn request_name(request: &ClientMsg) -> &'static str {
    match request {
        ClientMsg::Hello { .. } => "hello",
        ClientMsg::JoinWorld { .. } => "join_world",
        ClientMsg::Ping { .. } => "ping",
        ClientMsg::ChunkRequest { .. } => "chunk_request",
        ClientMsg::PlaceBlock { .. } => "place_block",
        ClientMsg::FlushPersistence { .. } => "flush_persistence",
    }
}
