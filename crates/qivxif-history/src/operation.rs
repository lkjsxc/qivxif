use crate::{HistoryError, HistoryResult};
use qivxif_core::{ActorId, ClientTime, NodeId, OperationId, ServerTime};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PayloadHash(String);

impl PayloadHash {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperationPayload {
    pub bytes: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationScope {
    Auth,
    Graph,
    Text,
    Tile,
    Sync,
    Publish,
    Social,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OperationKind {
    #[serde(rename = "user.bootstrap_admin")]
    UserBootstrapAdmin,
    #[serde(rename = "auth.login_session_created")]
    AuthLoginSessionCreated,
    #[serde(rename = "node.create")]
    NodeCreate,
    #[serde(rename = "node.update_metadata")]
    NodeUpdateMetadata,
    #[serde(rename = "node.tombstone")]
    NodeTombstone,
    #[serde(rename = "edge.create")]
    EdgeCreate,
    #[serde(rename = "edge.tombstone")]
    EdgeTombstone,
    #[serde(rename = "text.create_doc")]
    TextCreateDoc,
    #[serde(rename = "text.insert")]
    TextInsert,
    #[serde(rename = "text.delete")]
    TextDelete,
    #[serde(rename = "text.restore")]
    TextRestore,
    #[serde(rename = "tile.layout_set")]
    TileLayoutSet,
    #[serde(rename = "sync.cursor_advance")]
    SyncCursorAdvance,
    #[serde(rename = "publish.post")]
    PublishPost,
    #[serde(rename = "publish.unpublish")]
    PublishUnpublish,
    #[serde(rename = "social.short_post_create")]
    SocialShortPostCreate,
    #[serde(rename = "social.follow")]
    SocialFollow,
    #[serde(rename = "social.unfollow")]
    SocialUnfollow,
    #[serde(rename = "social.mute")]
    SocialMute,
    #[serde(rename = "social.unmute")]
    SocialUnmute,
    #[serde(rename = "social.block")]
    SocialBlock,
    #[serde(rename = "social.unblock")]
    SocialUnblock,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperationEnvelope {
    pub op_id: OperationId,
    pub actor_id: ActorId,
    pub actor_seq: u64,
    pub parents: Vec<OperationId>,
    pub scope: OperationScope,
    pub kind: OperationKind,
    pub target_node_ids: Vec<NodeId>,
    pub payload: OperationPayload,
    pub payload_hash: PayloadHash,
    pub created_at_client: Option<ClientTime>,
    pub received_at_server: Option<ServerTime>,
    pub auth_context: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedOperation(pub OperationEnvelope);

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TargetSet {
    pub nodes: BTreeSet<NodeId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayCursor {
    pub after_operation: Option<OperationId>,
}

pub fn hash_payload(bytes: &[u8]) -> PayloadHash {
    PayloadHash(blake3::hash(bytes).to_hex().to_string())
}

pub fn validate_operation_envelope(op: OperationEnvelope) -> HistoryResult<ValidatedOperation> {
    if op.actor_seq == 0 {
        return Err(HistoryError::InvalidActorSeq);
    }
    if op.payload_hash.as_str().is_empty() {
        return Err(HistoryError::MissingPayloadHash);
    }
    if op.payload_hash != hash_payload(&op.payload.bytes) {
        return Err(HistoryError::PayloadHashMismatch);
    }
    Ok(ValidatedOperation(op))
}

pub fn operation_targets(op: &OperationEnvelope) -> TargetSet {
    TargetSet {
        nodes: op.target_node_ids.iter().cloned().collect(),
    }
}

impl ReplayCursor {
    pub fn from_operation(op_id: OperationId) -> Self {
        Self {
            after_operation: Some(op_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qivxif_core::{ActorId, OperationId};

    fn op(seq: u64, payload_hash: PayloadHash) -> OperationEnvelope {
        let payload = OperationPayload {
            bytes: b"{}".to_vec(),
        };
        OperationEnvelope {
            op_id: OperationId::generate(),
            actor_id: ActorId::generate(),
            actor_seq: seq,
            parents: Vec::new(),
            scope: OperationScope::Graph,
            kind: OperationKind::NodeCreate,
            target_node_ids: Vec::new(),
            payload,
            payload_hash,
            created_at_client: None,
            received_at_server: None,
            auth_context: None,
        }
    }

    #[test]
    fn rejects_invalid_actor_sequence() {
        let hash = hash_payload(b"{}");
        assert_eq!(
            validate_operation_envelope(op(0, hash)).unwrap_err(),
            HistoryError::InvalidActorSeq
        );
    }

    #[test]
    fn rejects_missing_payload_hash() {
        assert_eq!(
            validate_operation_envelope(op(1, PayloadHash(String::new()))).unwrap_err(),
            HistoryError::MissingPayloadHash
        );
    }

    #[test]
    fn rejects_payload_hash_mismatch() {
        let hash = hash_payload(b"different");
        assert_eq!(
            validate_operation_envelope(op(1, hash)).unwrap_err(),
            HistoryError::PayloadHashMismatch
        );
    }
}
