use crate::connect::Client;
use anyhow::{Result, bail};
use qivxif_protocol::{ClientMsg, ServerMsg};

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
