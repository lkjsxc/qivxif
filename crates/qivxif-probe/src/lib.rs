use anyhow::{Result, anyhow, bail};
use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::{BlockCell, ClientMsg, ServerMsg};
use std::{net::SocketAddr, time::Duration};

const PROTOCOL_EPOCH: u32 = 1;
const PLAYER: &str = "probe";
const REQUEST_ID: u64 = 1;
const TEST_POS: BlockPos = BlockPos { x: 1, y: 3, z: 1 };
const TEST_BLOCK: u16 = 9;

pub async fn smoke(addr: &str) -> Result<()> {
    let client = ProbeClient::connect(addr).await?;
    expect_hello(client.request(hello()).await?)?;
    expect_join(client.request(join()).await?)?;
    expect_chunk(client.request(chunk_request()).await?)?;
    match client.request(ClientMsg::Ping { nonce: 42 }).await? {
        ServerMsg::Pong { nonce: 42 } => Ok(()),
        other => bail!("unexpected ping response: {other:?}"),
    }
}

pub async fn persist_place(addr: &str) -> Result<()> {
    let client = ProbeClient::connect(addr).await?;
    expect_hello(client.request(hello()).await?)?;
    expect_join(client.request(join()).await?)?;
    match client
        .request(ClientMsg::PlaceBlock {
            request_id: REQUEST_ID,
            pos: TEST_POS,
            block: TEST_BLOCK,
        })
        .await?
    {
        ServerMsg::MutationAck { request_id, cell }
            if request_id == REQUEST_ID && cell == test_cell() =>
        {
            Ok(())
        }
        other => bail!("unexpected mutation response: {other:?}"),
    }
}

fn test_cell() -> BlockCell {
    BlockCell {
        pos: TEST_POS,
        block: TEST_BLOCK,
    }
}

pub async fn persist_check(addr: &str) -> Result<()> {
    let client = ProbeClient::connect(addr).await?;
    expect_hello(client.request(hello()).await?)?;
    expect_join(client.request(join()).await?)?;
    let cells = expect_chunk(client.request(chunk_request()).await?)?;
    if cells.contains(&BlockCell {
        pos: TEST_POS,
        block: TEST_BLOCK,
    }) {
        Ok(())
    } else {
        bail!("persisted block missing at test position")
    }
}

struct ProbeClient {
    _endpoint: quinn::Endpoint,
    connection: quinn::Connection,
}

impl ProbeClient {
    async fn connect(addr: &str) -> Result<Self> {
        let remote = wait_for_addr(addr).await?;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(60);
        loop {
            let endpoint = qivxif_net::client_endpoint()?;
            match endpoint.connect(remote, "localhost")?.await {
                Ok(connection) => {
                    return Ok(Self {
                        _endpoint: endpoint,
                        connection,
                    });
                }
                Err(error) if tokio::time::Instant::now() < deadline => {
                    let _ = error;
                    tokio::time::sleep(Duration::from_millis(250)).await;
                }
                Err(error) => return Err(anyhow!("connect failed: {error}")),
            }
        }
    }

    async fn request(&self, msg: ClientMsg) -> Result<ServerMsg> {
        let (mut send, mut recv) = self.connection.open_bi().await?;
        qivxif_net::send_wire(&mut send, &msg).await?;
        qivxif_net::recv_wire(&mut recv).await
    }
}

async fn wait_for_addr(addr: &str) -> Result<SocketAddr> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(60);
    loop {
        match tokio::net::lookup_host(addr).await {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    return Ok(addr);
                }
            }
            Err(error) if tokio::time::Instant::now() >= deadline => {
                return Err(anyhow!("resolve failed: {error}"));
            }
            Err(_) => {}
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}

fn hello() -> ClientMsg {
    ClientMsg::Hello {
        build_epoch: "probe".to_string(),
        protocol_epoch: PROTOCOL_EPOCH,
    }
}

fn join() -> ClientMsg {
    ClientMsg::JoinWorld {
        player: PLAYER.to_string(),
    }
}

fn chunk_request() -> ClientMsg {
    ClientMsg::ChunkRequest {
        coord: ChunkCoord { x: 0, z: 0 },
    }
}

fn expect_hello(msg: ServerMsg) -> Result<()> {
    match msg {
        ServerMsg::HelloOk { .. } => Ok(()),
        other => bail!("unexpected hello response: {other:?}"),
    }
}

fn expect_join(msg: ServerMsg) -> Result<()> {
    match msg {
        ServerMsg::Joined { player } if player == PLAYER => Ok(()),
        other => bail!("unexpected join response: {other:?}"),
    }
}

fn expect_chunk(msg: ServerMsg) -> Result<Vec<BlockCell>> {
    match msg {
        ServerMsg::Chunk { cells, .. } if !cells.is_empty() => Ok(cells),
        other => bail!("unexpected chunk response: {other:?}"),
    }
}
