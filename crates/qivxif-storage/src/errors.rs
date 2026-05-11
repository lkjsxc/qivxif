use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("redb: {0}")]
    Redb(String),
    #[error("archive: {0}")]
    Archive(String),
    #[error(transparent)]
    Codec(#[from] postcard::Error),
}

pub(crate) fn redb_error(error: impl std::fmt::Display) -> StoreError {
    StoreError::Redb(error.to_string())
}

pub(crate) fn archive_error(error: impl std::fmt::Display) -> StoreError {
    StoreError::Archive(error.to_string())
}
