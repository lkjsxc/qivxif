use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    BadRequest,
    BuildEpochMissing,
    ProtocolEpochMismatch,
    HelloRequired,
    JoinRequired,
    ChunkError,
    MutationError,
    FlushError,
}
