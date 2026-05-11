use crate::connect::Client;
use anyhow::{Result, bail};
use qivxif_core::ChunkCoord;
use qivxif_protocol::{BlockCell, ClientMsg, ServerMsg};

pub async fn request_chunk(client: &Client, coord: ChunkCoord) -> Result<Vec<BlockCell>> {
    match client.request(ClientMsg::ChunkRequest { coord }).await? {
        ServerMsg::Chunk {
            coord: actual,
            cells,
        } if actual == coord => Ok(cells),
        other => bail!("unexpected chunk response: {other:?}"),
    }
}
