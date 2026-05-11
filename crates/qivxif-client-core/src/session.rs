use crate::{ClientConfig, TlsMode};
use anyhow::{Result, anyhow};
use qivxif_protocol::{CURRENT_PROTOCOL_CONTRACT, ClientMsg, ServerCaps, ServerMsg};
use std::{net::SocketAddr, time::Duration};

pub struct Client {
    _endpoint: quinn::Endpoint,
    connection: quinn::Connection,
}

pub struct HelloReceipt {
    pub session_id: u64,
    pub world_id: String,
    pub _caps: ServerCaps,
}

impl Client {
    pub async fn connect(config: &ClientConfig) -> Result<Self> {
        let remote = wait_for_addr(&config.addr).await?;
        let endpoint = qivxif_net::client_endpoint_with_tls(tls_mode(config.tls_mode))?;
        tracing::info!(
            %remote,
            server_name = %config.server_name,
            tls = config.tls_mode.as_str(),
            "connecting"
        );
        let connection = endpoint
            .connect(remote, &config.server_name)?
            .await
            .map_err(|error| anyhow!("connect failed: {error}"))?;
        Ok(Self {
            _endpoint: endpoint,
            connection,
        })
    }

    pub async fn hello(&self) -> Result<HelloReceipt> {
        match self
            .request(ClientMsg::Hello {
                build_contract: "client-cli".to_string(),
                protocol_contract: CURRENT_PROTOCOL_CONTRACT.to_string(),
            })
            .await?
        {
            ServerMsg::HelloOk {
                session_id,
                world_id,
                caps,
            } => Ok(HelloReceipt {
                session_id,
                world_id,
                _caps: caps,
            }),
            other => anyhow::bail!("unexpected hello response: {other:?}"),
        }
    }

    pub async fn request(&self, msg: ClientMsg) -> Result<ServerMsg> {
        let (mut send, mut recv) = self.connection.open_bi().await?;
        qivxif_net::send_wire(&mut send, &msg).await?;
        qivxif_net::recv_wire(&mut recv).await
    }
}

fn tls_mode(mode: TlsMode) -> qivxif_net::ClientTlsMode {
    match mode {
        TlsMode::Verified => qivxif_net::ClientTlsMode::VerifiedRoots,
        TlsMode::LocalCompose => qivxif_net::ClientTlsMode::LocalComposeInsecure,
    }
}

async fn wait_for_addr(addr: &str) -> Result<SocketAddr> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
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
