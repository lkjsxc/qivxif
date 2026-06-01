use crate::{
    StoreError, StoreResult,
    codec::decode,
    feed_audience::{remove_feed_markers_for_author, user_by_profile},
    feed_support::ensure_session_actor,
    follow_support::{
        active_follow, edge_by_id, follow_edge, follow_operation, insert_edge_with_indexes,
        update_edge,
    },
    operation_log::insert_operation,
    records::OperationReceipt,
    store::QivxifStore,
};
use qivxif_auth::{AuthContext, can_read};
use qivxif_core::{ActorId, EdgeId, NodeId, OperationId, UserId};
use qivxif_graph::{EdgeKind, EdgeRecord, NodeKind, Tombstone};
use qivxif_history::OperationKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FollowInput {
    pub op_id: OperationId,
    pub actor_seq: u64,
    pub edge_id: EdgeId,
    pub actor_id: ActorId,
    pub follower_user_id: UserId,
    pub follower_profile_node_id: NodeId,
    pub target_profile_node_id: NodeId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnfollowInput {
    pub op_id: OperationId,
    pub actor_seq: u64,
    pub edge_id: EdgeId,
    pub actor_id: ActorId,
    pub follower_user_id: UserId,
    pub follower_profile_node_id: NodeId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FollowResult {
    pub edge: EdgeRecord,
    pub receipt: OperationReceipt,
}

impl QivxifStore {
    pub fn follow_profile(
        &self,
        auth: &AuthContext,
        input: FollowInput,
    ) -> StoreResult<FollowResult> {
        if self.get_operation(&input.op_id)?.is_some() {
            return self.replay_follow(&input.op_id, OperationKind::SocialFollow);
        }
        validate_follow_target(self, auth, &input)?;
        let tx = self.database.begin_write()?;
        let existing = active_follow(
            &tx,
            &input.follower_profile_node_id,
            &input.target_profile_node_id,
        )?;
        let already_active = existing.is_some();
        let edge = existing.unwrap_or_else(|| {
            follow_edge(
                input.edge_id.clone(),
                input.follower_profile_node_id.clone(),
                input.target_profile_node_id.clone(),
                input.actor_id.clone(),
            )
        });
        if edge.id != input.edge_id && edge_by_id(&tx, &input.edge_id)?.is_some() {
            return Err(StoreError::EdgeExists);
        }
        let op = follow_operation(
            &input.op_id,
            input.actor_seq,
            &input.actor_id,
            &input.follower_user_id,
            OperationKind::SocialFollow,
            &edge,
        )?;
        let receipt = insert_operation(&tx, &op)?;
        if !already_active {
            insert_edge_with_indexes(&tx, &edge)?;
        }
        tx.commit()?;
        Ok(FollowResult { edge, receipt })
    }

    pub fn unfollow_profile(
        &self,
        auth: &AuthContext,
        input: UnfollowInput,
    ) -> StoreResult<FollowResult> {
        if self.get_operation(&input.op_id)?.is_some() {
            return self.replay_follow(&input.op_id, OperationKind::SocialUnfollow);
        }
        ensure_session_actor(auth, &input.actor_id, &input.follower_user_id)?;
        let tx = self.database.begin_write()?;
        let mut edge = edge_by_id(&tx, &input.edge_id)?.ok_or(StoreError::OperationConflict)?;
        validate_unfollow_edge(&input, &edge)?;
        edge.tombstone = Some(Tombstone {
            by: input.actor_id.clone(),
            reason: "unfollow".to_owned(),
        });
        let target_user =
            user_by_profile(&tx, &edge.to_node)?.ok_or(StoreError::OperationConflict)?;
        let op = follow_operation(
            &input.op_id,
            input.actor_seq,
            &input.actor_id,
            &input.follower_user_id,
            OperationKind::SocialUnfollow,
            &edge,
        )?;
        let receipt = insert_operation(&tx, &op)?;
        update_edge(&tx, &edge)?;
        remove_feed_markers_for_author(&tx, &input.follower_user_id, &target_user.id)?;
        tx.commit()?;
        Ok(FollowResult { edge, receipt })
    }

    fn replay_follow(
        &self,
        op_id: &OperationId,
        expected: OperationKind,
    ) -> StoreResult<FollowResult> {
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
        Ok(FollowResult { edge, receipt })
    }
}

fn validate_follow_target(
    store: &QivxifStore,
    auth: &AuthContext,
    input: &FollowInput,
) -> StoreResult<()> {
    ensure_session_actor(auth, &input.actor_id, &input.follower_user_id)?;
    if input.follower_profile_node_id == input.target_profile_node_id {
        return Err(StoreError::InvalidOperation);
    }
    let Some(from) = store.get_node(&input.follower_profile_node_id)? else {
        return Err(StoreError::NodeMissing);
    };
    let Some(to) = store.get_node(&input.target_profile_node_id)? else {
        return Err(StoreError::NodeMissing);
    };
    if from.kind != NodeKind::Profile || to.kind != NodeKind::Profile {
        return Err(StoreError::InvalidOperation);
    }
    if from.owner_user_id != input.follower_user_id || !can_read(auth, &to) {
        return Err(StoreError::Forbidden);
    }
    Ok(())
}

fn validate_unfollow_edge(input: &UnfollowInput, edge: &EdgeRecord) -> StoreResult<()> {
    if edge.kind != EdgeKind::Follows || edge.tombstone.is_some() {
        return Err(StoreError::OperationConflict);
    }
    if edge.from_node != input.follower_profile_node_id {
        return Err(StoreError::Forbidden);
    }
    Ok(())
}
