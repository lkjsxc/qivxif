use crate::transport::ProbeClient;
use anyhow::{Result, bail};
use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::{BlockCell, CURRENT_PROTOCOL_EPOCH, ClientMsg, ServerMsg};

const PLAYER: &str = "probe";
const REQUEST_ID: u64 = 1;
const TEST_POS: BlockPos = BlockPos { x: 1, y: 3, z: 1 };
const TEST_BLOCK: u16 = 9;

pub async fn hello(addr: &str) -> Result<()> {
    let client = ProbeClient::connect(addr).await?;
    expect_hello(client.request(hello_request()).await?)
}

pub async fn join_world(addr: &str) -> Result<()> {
    let client = ProbeClient::connect(addr).await?;
    expect_hello(client.request(hello_request()).await?)?;
    expect_join(client.request(join_request()).await?)
}

pub async fn chunk_request(addr: &str) -> Result<()> {
    let client = connect(addr).await?;
    expect_chunk(client.request(chunk_request_msg()).await?)?;
    Ok(())
}

pub async fn place_block(addr: &str) -> Result<()> {
    let client = connect(addr).await?;
    expect_mutation(client.request(place_request()).await?)
}

pub async fn flush_persistence(addr: &str) -> Result<()> {
    let client = connect(addr).await?;
    expect_mutation(client.request(place_request()).await?)?;
    expect_flush(client.request(flush_request()).await?)
}

async fn connect(addr: &str) -> Result<ProbeClient> {
    let client = ProbeClient::connect(addr).await?;
    expect_hello(client.request(hello_request()).await?)?;
    expect_join(client.request(join_request()).await?)?;
    Ok(client)
}

fn hello_request() -> ClientMsg {
    ClientMsg::Hello {
        build_epoch: "probe".to_string(),
        protocol_epoch: CURRENT_PROTOCOL_EPOCH,
    }
}

fn join_request() -> ClientMsg {
    ClientMsg::JoinWorld {
        player: PLAYER.to_string(),
    }
}

fn chunk_request_msg() -> ClientMsg {
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
