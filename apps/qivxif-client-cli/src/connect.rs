use anyhow::{Result, anyhow, bail};
use qivxif_protocol::{CURRENT_PROTOCOL_EPOCH, ClientMsg, ServerCaps, ServerMsg};
use std::{net::SocketAddr, time::Duration};

pub struct Client {
    _endpoint: quinn::Endpoint,
    connection: quinn::Connection,
}

pub struct HelloReceipt {
    pub session_id: u64,
    pub world_epoch: String,
    pub _caps: ServerCaps,
}

impl Client {
    pub async fn connect(addr: &str) -> Result<Self> {
        let remote = wait_for_addr(addr).await?;
        let endpoint = qivxif_net::client_endpoint()?;
        tracing::info!(%remote, "connecting with verified TLS config");
        let connection = endpoint
            .connect(remote, "localhost")?
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
                build_epoch: "client-cli".to_string(),
                protocol_epoch: CURRENT_PROTOCOL_EPOCH,
            })
            .await?
        {
            ServerMsg::HelloOk {
                session_id,
                world_epoch,
                caps,
            } => Ok(HelloReceipt {
                session_id,
                world_epoch,
                _caps: caps,
            }),
            other => bail!("unexpected hello response: {other:?}"),
        }
    }

    pub async fn request(&self, msg: ClientMsg) -> Result<ServerMsg> {
        let (mut send, mut recv) = self.connection.open_bi().await?;
        qivxif_net::send_wire(&mut send, &msg).await?;
        qivxif_net::recv_wire(&mut recv).await
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
