use thiserror::Error;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("invalid id")]
    InvalidId,
    #[error("invalid time")]
    InvalidTime,
    #[error("invalid capability")]
    InvalidCapability,
}
