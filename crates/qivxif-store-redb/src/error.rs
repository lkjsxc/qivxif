use thiserror::Error;

pub type StoreResult<T> = Result<T, StoreError>;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("database error: {0}")]
    Database(Box<redb::DatabaseError>),
    #[error("transaction error: {0}")]
    Transaction(Box<redb::TransactionError>),
    #[error("table error: {0}")]
    Table(Box<redb::TableError>),
    #[error("storage error: {0}")]
    Storage(Box<redb::StorageError>),
    #[error("commit error: {0}")]
    Commit(Box<redb::CommitError>),
    #[error("encoding error: {0}")]
    Codec(#[from] Box<bincode::ErrorKind>),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("admin already exists")]
    AdminExists,
    #[error("duplicate user name")]
    DuplicateUserName,
    #[error("operation conflicts with existing durable record")]
    OperationConflict,
    #[error("actor sequence already belongs to another operation")]
    DuplicateActorSeq,
    #[error("node already exists")]
    NodeExists,
    #[error("node missing")]
    NodeMissing,
    #[error("edge already exists")]
    EdgeExists,
    #[error("publish slug conflicts with existing post")]
    SlugConflict,
    #[error("access denied")]
    Forbidden,
    #[error("cursor invalid")]
    CursorInvalid,
    #[error("operation envelope invalid")]
    InvalidOperation,
    #[error("operation kind is not accepted by this store path")]
    UnknownOperationKind,
}

impl From<redb::DatabaseError> for StoreError {
    fn from(error: redb::DatabaseError) -> Self {
        Self::Database(Box::new(error))
    }
}

impl From<redb::TransactionError> for StoreError {
    fn from(error: redb::TransactionError) -> Self {
        Self::Transaction(Box::new(error))
    }
}

impl From<redb::TableError> for StoreError {
    fn from(error: redb::TableError) -> Self {
        Self::Table(Box::new(error))
    }
}

impl From<redb::StorageError> for StoreError {
    fn from(error: redb::StorageError) -> Self {
        Self::Storage(Box::new(error))
    }
}

impl From<redb::CommitError> for StoreError {
    fn from(error: redb::CommitError) -> Self {
        Self::Commit(Box::new(error))
    }
}
