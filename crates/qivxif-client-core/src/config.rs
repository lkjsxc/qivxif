use anyhow::{Result, bail};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsMode {
    Verified,
    LocalCompose,
}

impl FromStr for TlsMode {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "verified" => Ok(Self::Verified),
            "local-compose" => Ok(Self::LocalCompose),
            other => bail!("unknown TLS mode: {other}"),
        }
    }
}

impl TlsMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::LocalCompose => "local-compose",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub addr: String,
    pub server_name: String,
    pub tls_mode: TlsMode,
}

impl ClientConfig {
    pub fn local_compose(addr: impl Into<String>) -> Self {
        Self {
            addr: addr.into(),
            server_name: "localhost".to_string(),
            tls_mode: TlsMode::LocalCompose,
        }
    }
}
