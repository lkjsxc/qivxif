use thiserror::Error;

pub type GraphResult<T> = Result<T, GraphError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GraphError {
    #[error("node already exists")]
    NodeExists,
    #[error("edge already exists")]
    EdgeExists,
    #[error("node missing")]
    NodeMissing,
    #[error("edge missing")]
    EdgeMissing,
    #[error("tree contains a cycle")]
    TreeCycle,
    #[error("tree child has multiple active parents")]
    DuplicateActiveParent,
    #[error("tree relation is tombstoned")]
    TombstonedRelation,
    #[error("unknown node kind")]
    UnknownNodeKind,
    #[error("unknown edge kind")]
    UnknownEdgeKind,
}
