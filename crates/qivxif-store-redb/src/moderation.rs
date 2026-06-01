use crate::{
    StoreError, StoreResult,
    codec::decode,
    feed_support::ensure_session_actor,
    follow_support::{
        active_edge_kind, edge_by_id, insert_edge_with_indexes, social_edge_operation, update_edge,
    },
    operation_log::insert_operation,
    records::OperationReceipt,
    store::QivxifStore,
};
use qivxif_auth::{AuthContext, can_read};
use qivxif_core::{ActorId, EdgeId, MetadataMap, NodeId, OperationId, ServerTime, UserId};
use qivxif_graph::{EdgeRecord, NodeKind, Tombstone};
use qivxif_history::OperationKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModerationAction {
    Mute,
    Block,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModerationInput {
    pub op_id: OperationId,
    pub actor_seq: u64,
    pub edge_id: EdgeId,
    pub actor_id: ActorId,
    pub actor_user_id: UserId,
    pub actor_profile_node_id: NodeId,
    pub target_profile_node_id: NodeId,
    pub action: ModerationAction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModerationClearInput {
    pub op_id: OperationId,
    pub actor_seq: u64,
    pub edge_id: EdgeId,
    pub actor_id: ActorId,
    pub actor_user_id: UserId,
    pub actor_profile_node_id: NodeId,
    pub action: ModerationAction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModerationResult {
    pub edge: EdgeRecord,
    pub receipt: OperationReceipt,
}

impl QivxifStore {
    pub fn create_moderation_edge(
        &self,
        auth: &AuthContext,
        input: ModerationInput,
    ) -> StoreResult<ModerationResult> {
        if self.get_operation(&input.op_id)?.is_some() {
            return self.replay_moderation(&input.op_id, input.action.create_op());
        }
        validate_target(self, auth, &input)?;
        let tx = self.database.begin_write()?;
        let existing = active_edge_kind(
            &tx,
            &input.actor_profile_node_id,
            &input.target_profile_node_id,
            input.action.edge_kind(),
        )?;
        let already_active = existing.is_some();
        let edge = existing.unwrap_or_else(|| moderation_edge(&input));
        if edge.id != input.edge_id && edge_by_id(&tx, &input.edge_id)?.is_some() {
            return Err(StoreError::EdgeExists);
        }
        let op = social_edge_operation(
            &input.op_id,
            input.actor_seq,
            &input.actor_id,
            &input.actor_user_id,
            input.action.create_op(),
            &edge,
        )?;
        let receipt = insert_operation(&tx, &op)?;
        if !already_active {
            insert_edge_with_indexes(&tx, &edge)?;
        }
        tx.commit()?;
        Ok(ModerationResult { edge, receipt })
    }

    pub fn clear_moderation_edge(
        &self,
        auth: &AuthContext,
        input: ModerationClearInput,
    ) -> StoreResult<ModerationResult> {
        if self.get_operation(&input.op_id)?.is_some() {
            return self.replay_moderation(&input.op_id, input.action.clear_op());
        }
        ensure_session_actor(auth, &input.actor_id, &input.actor_user_id)?;
        let tx = self.database.begin_write()?;
        let mut edge = edge_by_id(&tx, &input.edge_id)?.ok_or(StoreError::OperationConflict)?;
        if edge.kind != input.action.edge_kind() || edge.tombstone.is_some() {
            return Err(StoreError::OperationConflict);
        }
        if edge.from_node != input.actor_profile_node_id {
            return Err(StoreError::Forbidden);
        }
        edge.tombstone = Some(Tombstone {
            by: input.actor_id.clone(),
            reason: input.action.clear_reason().to_owned(),
        });
        let op = social_edge_operation(
            &input.op_id,
            input.actor_seq,
            &input.actor_id,
            &input.actor_user_id,
            input.action.clear_op(),
            &edge,
        )?;
        let receipt = insert_operation(&tx, &op)?;
        update_edge(&tx, &edge)?;
        tx.commit()?;
        Ok(ModerationResult { edge, receipt })
    }

    fn replay_moderation(
        &self,
        op_id: &OperationId,
        expected: OperationKind,
    ) -> StoreResult<ModerationResult> {
        let operation = self
            .get_operation(op_id)?
            .ok_or(StoreError::OperationConflict)?;
        if operation.kind != expected {
            return Err(StoreError::OperationConflict);
        }
        let edge = decode(&operation.payload.bytes)?;
        let receipt = self
            .operation_receipt(op_id)?
            .ok_or(StoreError::OperationConflict)?;
        Ok(ModerationResult { edge, receipt })
    }
}

fn validate_target(
    store: &QivxifStore,
    auth: &AuthContext,
    input: &ModerationInput,
) -> StoreResult<()> {
    ensure_session_actor(auth, &input.actor_id, &input.actor_user_id)?;
    if input.actor_profile_node_id == input.target_profile_node_id {
        return Err(StoreError::InvalidOperation);
    }
    let Some(from) = store.get_node(&input.actor_profile_node_id)? else {
        return Err(StoreError::NodeMissing);
    };
    let Some(to) = store.get_node(&input.target_profile_node_id)? else {
        return Err(StoreError::NodeMissing);
    };
    if from.kind != NodeKind::Profile || to.kind != NodeKind::Profile {
        return Err(StoreError::InvalidOperation);
    }
    if from.owner_user_id != input.actor_user_id || !can_read(auth, &to) {
        return Err(StoreError::Forbidden);
    }
    Ok(())
}

fn moderation_edge(input: &ModerationInput) -> EdgeRecord {
    let mut metadata = MetadataMap::empty();
    metadata.insert("moderation_state", input.action.state_name());
    EdgeRecord {
        id: input.edge_id.clone(),
        from_node: input.actor_profile_node_id.clone(),
        to_node: input.target_profile_node_id.clone(),
        kind: input.action.edge_kind(),
        created_by: input.actor_id.clone(),
        created_at: ServerTime::now(),
        metadata_map: metadata,
        tombstone: None,
    }
}
