use crate::transport::ProbeClient;
use anyhow::{Result, bail};
use qivxif_protocol::{ClientMsg, ErrorCode, ServerMsg};

pub async fn malformed_wire(addr: &str) -> Result<()> {
    let client = ProbeClient::connect(addr).await?;
    expect_error(
        client.raw_request(&[0xff, 0x00, 0x7f]).await?,
        ErrorCode::BadRequest,
        "malformed wire",
    )?;
    expect_error(
        client.request(join()).await?,
        ErrorCode::HelloRequired,
        "join after malformed wire",
    )
}

fn join() -> ClientMsg {
    ClientMsg::JoinWorld {
        player: "malformed".to_string(),
    }
}

fn expect_error(msg: ServerMsg, expected: ErrorCode, label: &str) -> Result<()> {
    match msg {
        ServerMsg::Error { code, .. } if code == expected => Ok(()),
        other => bail!("{label} returned unexpected response: {other:?}"),
    }
}
