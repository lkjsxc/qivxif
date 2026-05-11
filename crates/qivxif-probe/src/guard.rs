use crate::transport::ProbeClient;
use anyhow::{Result, bail};
use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::{CURRENT_PROTOCOL_CONTRACT, ClientMsg, ErrorCode, ServerMsg};

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
        missing_build
            .request(hello("", CURRENT_PROTOCOL_CONTRACT))
            .await?,
        ErrorCode::BuildContractMissing,
        "missing build contract",
    )?;

    let wrong_protocol = ProbeClient::connect(addr).await?;
    expect_error(
        wrong_protocol.request(hello("probe", "mismatch")).await?,
        ErrorCode::ProtocolContractMismatch,
        "protocol contract mismatch",
    )
}

fn hello(build_contract: &str, protocol_contract: &str) -> ClientMsg {
    ClientMsg::Hello {
        build_contract: build_contract.to_string(),
        protocol_contract: protocol_contract.to_string(),
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
