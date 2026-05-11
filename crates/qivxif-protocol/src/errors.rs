use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    BadRequest,
    BuildContractMissing,
    ProtocolContractMismatch,
    HelloRequired,
    JoinRequired,
    ChunkError,
    MutationError,
    FlushError,
}
