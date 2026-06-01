use thiserror::Error;

pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AuthError {
    #[error("password hash failed")]
    PasswordHashFailed,
    #[error("password did not verify")]
    PasswordRejected,
    #[error("password hash is malformed")]
    MalformedPasswordHash,
}
