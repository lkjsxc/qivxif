use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("redb: {0}")]
    Redb(String),
    #[error(transparent)]
    Codec(#[from] postcard::Error),
}

pub(crate) fn redb_error(error: impl std::fmt::Display) -> StoreError {
    StoreError::Redb(error.to_string())
}
