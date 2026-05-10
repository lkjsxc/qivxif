use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn send_json<W, T>(writer: &mut W, value: &T) -> Result<()>
where
    W: AsyncWriteExt + Unpin,
    T: Serialize,
{
    let bytes = serde_json::to_vec(value)?;
    writer.write_u32(bytes.len() as u32).await?;
    writer.write_all(&bytes).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn recv_json<R, T>(reader: &mut R) -> Result<T>
where
    R: AsyncReadExt + Unpin,
    T: DeserializeOwned,
{
    let len = reader.read_u32().await? as usize;
    let mut bytes = vec![0; len];
    reader.read_exact(&mut bytes).await?;
    Ok(serde_json::from_slice(&bytes)?)
}
