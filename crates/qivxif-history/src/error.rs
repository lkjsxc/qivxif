use thiserror::Error;

pub type HistoryResult<T> = Result<T, HistoryError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HistoryError {
    #[error("actor sequence must be greater than zero")]
    InvalidActorSeq,
    #[error("payload hash is missing")]
    MissingPayloadHash,
    #[error("payload hash mismatch")]
    PayloadHashMismatch,
    #[error("cursor must use a durable operation id")]
    InvalidCursor,
}
