use qivxif_core::{OperationId, TextDocId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextSnapshotRef {
    pub doc_id: TextDocId,
    pub after_operation: OperationId,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextDocState {
    pub content: String,
    pub applied_operations: Vec<OperationId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextSnapshot {
    pub doc_id: TextDocId,
    pub after_operation: OperationId,
    pub content: String,
}
