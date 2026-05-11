use crate::transport::ProbeClient;
use anyhow::{Result, bail};
use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::{ClientMsg, ErrorCode, ServerMsg};

const PROTOCOL_EPOCH: u32 = 1;

pub async fn protocol_guards(addr: &str) -> Result<()> {
    let client = ProbeClient::connect(addr).await?;

    expect_error(
        client.request(join()).await?,
        ErrorCode::HelloRequired,
        "join before hello",
    )?;
    expect_error(
        client.request(ClientMsg::Ping { nonce: 1 }).await?,
        ErrorCode::HelloRequired,
        "ping before hello",
    )?;
    expect_error(
        client.request(chunk_request()).await?,
        ErrorCode::JoinRequired,
        "chunk before join",
    )?;
    expect_error(
        client.request(place_request()).await?,
        ErrorCode::JoinRequired,
        "place before join",
    )?;
    expect_error(
        client.request(flush_request()).await?,
        ErrorCode::JoinRequired,
        "flush before join",
    )?;

    let missing_build = ProbeClient::connect(addr).await?;
    expect_error(
        missing_build.request(hello("", PROTOCOL_EPOCH)).await?,
        ErrorCode::BuildEpochMissing,
        "missing build epoch",
    )?;

    let wrong_protocol = ProbeClient::connect(addr).await?;
    expect_error(
        wrong_protocol
            .request(hello("probe", PROTOCOL_EPOCH + 1))
            .await?,
        ErrorCode::ProtocolEpochMismatch,
        "protocol epoch mismatch",
    )
}

fn hello(build_epoch: &str, protocol_epoch: u32) -> ClientMsg {
    ClientMsg::Hello {
        build_epoch: build_epoch.to_string(),
        protocol_epoch,
    }
}

fn join() -> ClientMsg {
    ClientMsg::JoinWorld {
        player: "guard".to_string(),
    }
}

fn chunk_request() -> ClientMsg {
    ClientMsg::ChunkRequest {
        coord: ChunkCoord { x: 0, z: 0 },
    }
}

fn place_request() -> ClientMsg {
    ClientMsg::PlaceBlock {
        request_id: 91,
        pos: BlockPos { x: 1, y: 3, z: 1 },
        block: 9,
    }
}

fn flush_request() -> ClientMsg {
    ClientMsg::FlushPersistence { request_id: 92 }
}

fn expect_error(msg: ServerMsg, expected: ErrorCode, label: &str) -> Result<()> {
    match msg {
        ServerMsg::Error { code, .. } if code == expected => Ok(()),
        other => bail!("{label} returned unexpected response: {other:?}"),
    }
}
