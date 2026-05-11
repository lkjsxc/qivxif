use crate::Client;
use anyhow::{Result, bail};
use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::{BlockCell, ClientMsg, RequestId, ServerMsg};

pub async fn join_world(client: &Client, player: &str) -> Result<()> {
    match client
        .request(ClientMsg::JoinWorld {
            player: player.to_string(),
        })
        .await?
    {
        ServerMsg::Joined { player: joined } if joined == player => Ok(()),
        other => bail!("unexpected join response: {other:?}"),
    }
}

pub async fn request_chunk(client: &Client, coord: ChunkCoord) -> Result<Vec<BlockCell>> {
    match client.request(ClientMsg::ChunkRequest { coord }).await? {
        ServerMsg::Chunk {
            coord: actual,
            cells,
        } if actual == coord => Ok(cells),
        other => bail!("unexpected chunk response: {other:?}"),
    }
}

pub async fn place_block(
    client: &Client,
    request_id: RequestId,
    pos: BlockPos,
    block: u16,
) -> Result<()> {
    match client
        .request(ClientMsg::PlaceBlock {
            request_id,
            pos,
            block,
        })
        .await?
    {
        ServerMsg::MutationAck {
            request_id: actual,
            cell,
        } if actual == request_id && cell.pos == pos && cell.block == block => Ok(()),
        other => bail!("unexpected place response: {other:?}"),
    }
}

pub async fn flush_persistence(client: &Client, request_id: RequestId) -> Result<()> {
    match client
        .request(ClientMsg::FlushPersistence { request_id })
        .await?
    {
        ServerMsg::FlushAck { request_id: actual } if actual == request_id => Ok(()),
        other => bail!("unexpected flush response: {other:?}"),
    }
}
