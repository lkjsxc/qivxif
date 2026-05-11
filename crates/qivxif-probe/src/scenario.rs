use crate::transport::ProbeClient;
use anyhow::{Result, bail};
use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::{BlockCell, CURRENT_PROTOCOL_EPOCH, ClientMsg, ServerMsg};

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
    expect_mutation(client.request(place_request()).await?)?;
    expect_flush(client.request(flush_request()).await?)
}

pub async fn request_replay(addr: &str) -> Result<()> {
    let client = ProbeClient::connect(addr).await?;
    expect_hello(client.request(hello()).await?)?;
    expect_join(client.request(join()).await?)?;
    let first_place = client.request(place_request()).await?;
    let second_place = client.request(place_request()).await?;
    expect_same(first_place, second_place, "place replay")?;
    let first_flush = client.request(flush_request()).await?;
    let second_flush = client.request(flush_request()).await?;
    expect_same(first_flush, second_flush, "flush replay")
}

pub async fn persist_check(addr: &str) -> Result<()> {
    let client = ProbeClient::connect(addr).await?;
    expect_hello(client.request(hello()).await?)?;
    expect_join(client.request(join()).await?)?;
    let cells = expect_chunk(client.request(chunk_request()).await?)?;
    if cells.contains(&test_cell()) {
        Ok(())
    } else {
        bail!("persisted block missing at test position")
    }
}

fn hello() -> ClientMsg {
    ClientMsg::Hello {
        build_epoch: "probe".to_string(),
        protocol_epoch: CURRENT_PROTOCOL_EPOCH,
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

fn place_request() -> ClientMsg {
    ClientMsg::PlaceBlock {
        request_id: REQUEST_ID,
        pos: TEST_POS,
        block: TEST_BLOCK,
    }
}

fn flush_request() -> ClientMsg {
    ClientMsg::FlushPersistence {
        request_id: REQUEST_ID + 1,
    }
}

fn test_cell() -> BlockCell {
    BlockCell {
        pos: TEST_POS,
        block: TEST_BLOCK,
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

fn expect_mutation(msg: ServerMsg) -> Result<()> {
    match msg {
        ServerMsg::MutationAck { request_id, cell }
            if request_id == REQUEST_ID && cell == test_cell() =>
        {
            Ok(())
        }
        other => bail!("unexpected mutation response: {other:?}"),
    }
}

fn expect_flush(msg: ServerMsg) -> Result<()> {
    match msg {
        ServerMsg::FlushAck { request_id } if request_id == REQUEST_ID + 1 => Ok(()),
        other => bail!("unexpected flush response: {other:?}"),
    }
}

fn expect_same(first: ServerMsg, second: ServerMsg, label: &str) -> Result<()> {
    if first == second {
        Ok(())
    } else {
        bail!("{label} changed response: first={first:?} second={second:?}")
    }
}
