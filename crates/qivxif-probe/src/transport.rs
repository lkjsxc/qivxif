use anyhow::{Result, anyhow};
use qivxif_protocol::{ClientMsg, ServerMsg};
use std::{net::SocketAddr, time::Duration};

pub(crate) struct ProbeClient {
    _endpoint: quinn::Endpoint,
    connection: quinn::Connection,
}

impl ProbeClient {
    pub(crate) async fn connect(addr: &str) -> Result<Self> {
        let remote = wait_for_addr(addr).await?;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(60);
        loop {
            let endpoint = qivxif_net::client_endpoint()?;
            match endpoint.connect(remote, "localhost")?.await {
                Ok(connection) => {
                    return Ok(Self {
                        _endpoint: endpoint,
                        connection,
                    });
                }
                Err(error) if tokio::time::Instant::now() < deadline => {
                    let _ = error;
                    tokio::time::sleep(Duration::from_millis(250)).await;
                }
                Err(error) => return Err(anyhow!("connect failed: {error}")),
            }
        }
    }

    pub(crate) async fn request(&self, msg: ClientMsg) -> Result<ServerMsg> {
        let (mut send, mut recv) = self.connection.open_bi().await?;
        qivxif_net::send_wire(&mut send, &msg).await?;
        qivxif_net::recv_wire(&mut recv).await
    }

    pub(crate) async fn raw_request(&self, bytes: &[u8]) -> Result<ServerMsg> {
        let (mut send, mut recv) = self.connection.open_bi().await?;
        send.write_all(bytes).await?;
        send.finish()?;
        qivxif_net::recv_wire(&mut recv).await
    }
}

async fn wait_for_addr(addr: &str) -> Result<SocketAddr> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(60);
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
