use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BufferError {
    #[error("range starts after it ends")]
    ReversedRange,
    #[error("range is outside the buffer")]
    OutOfBounds,
    #[error("range does not start or end on a character boundary")]
    InvalidBoundary,
}
