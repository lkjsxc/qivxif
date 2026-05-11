use crate::connect::Client;
use anyhow::{Result, bail};
use qivxif_core::BlockPos;
use qivxif_protocol::{ClientMsg, RequestId, ServerMsg};

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
        ServerMsg::MutationAck { request_id: actual, cell }
            if actual == request_id && cell.pos == pos && cell.block == block =>
        {
            Ok(())
        }
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
