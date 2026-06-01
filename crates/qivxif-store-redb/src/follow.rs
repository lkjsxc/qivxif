use crate::{
    StoreError, StoreResult,
    codec::decode,
    event_log::insert_event,
    feed_audience::{remove_feed_markers_for_author, user_by_profile},
    feed_support::ensure_session_actor,
    follow_support::{
        active_follow, edge_by_id, follow_edge, insert_edge_with_indexes, social_edge_event,
        update_edge,
    },
    records::EventReceipt,
    store::QivxifStore,
};
use qivxif_auth::{AuthContext, can_read};
use qivxif_core::{ActorId, EdgeId, EventId, NodeId, UserId};
use qivxif_graph::{EdgeKind, EdgeRecord, NodeKind, Tombstone};
use qivxif_history::EventKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FollowInput {
    pub event_id: EventId,
    pub actor_seq: u64,
    pub edge_id: EdgeId,
    pub actor_id: ActorId,
    pub follower_user_id: UserId,
    pub follower_profile_node_id: NodeId,
    pub target_profile_node_id: NodeId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnfollowInput {
    pub event_id: EventId,
    pub actor_seq: u64,
    pub edge_id: EdgeId,
    pub actor_id: ActorId,
    pub follower_user_id: UserId,
    pub follower_profile_node_id: NodeId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FollowResult {
    pub edge: EdgeRecord,
    pub receipt: EventReceipt,
}

impl QivxifStore {
    pub fn follow_profile(
        &self,
        auth: &AuthContext,
        input: FollowInput,
    ) -> StoreResult<FollowResult> {
        if self.get_event(&input.event_id)?.is_some() {
            return self.replay_follow(&input.event_id, EventKind::SocialFollow);
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
        let event = social_edge_event(
            &input.event_id,
            input.actor_seq,
            &input.actor_id,
            &input.follower_user_id,
            EventKind::SocialFollow,
            &edge,
        )?;
        let receipt = insert_event(&tx, &event)?;
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
        if self.get_event(&input.event_id)?.is_some() {
            return self.replay_follow(&input.event_id, EventKind::SocialUnfollow);
        }
        ensure_session_actor(auth, &input.actor_id, &input.follower_user_id)?;
        let tx = self.database.begin_write()?;
        let mut edge = edge_by_id(&tx, &input.edge_id)?.ok_or(StoreError::EventConflict)?;
        validate_unfollow_edge(&input, &edge)?;
        edge.tombstone = Some(Tombstone {
            by: input.actor_id.clone(),
            reason: "unfollow".to_owned(),
        });
        let target_user = user_by_profile(&tx, &edge.to_node)?.ok_or(StoreError::EventConflict)?;
        let event = social_edge_event(
            &input.event_id,
            input.actor_seq,
            &input.actor_id,
            &input.follower_user_id,
            EventKind::SocialUnfollow,
            &edge,
        )?;
        let receipt = insert_event(&tx, &event)?;
        update_edge(&tx, &edge)?;
        remove_feed_markers_for_author(&tx, &input.follower_user_id, &target_user.id)?;
        tx.commit()?;
        Ok(FollowResult { edge, receipt })
    }

    fn replay_follow(&self, event_id: &EventId, expected: EventKind) -> StoreResult<FollowResult> {
        let event = self.get_event(event_id)?.ok_or(StoreError::EventConflict)?;
        if event.kind != expected {
            return Err(StoreError::EventConflict);
        }
        let edge = decode(&event.payload.bytes)?;
        let receipt = self
            .event_receipt(event_id)?
            .ok_or(StoreError::EventConflict)?;
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
        return Err(StoreError::InvalidEvent);
    }
    let Some(from) = store.get_node(&input.follower_profile_node_id)? else {
        return Err(StoreError::NodeMissing);
    };
    let Some(to) = store.get_node(&input.target_profile_node_id)? else {
        return Err(StoreError::NodeMissing);
    };
    if from.kind != NodeKind::Profile || to.kind != NodeKind::Profile {
        return Err(StoreError::InvalidEvent);
    }
    if from.owner_user_id != input.follower_user_id || !can_read(auth, &to) {
        return Err(StoreError::Forbidden);
    }
    Ok(())
}

fn validate_unfollow_edge(input: &UnfollowInput, edge: &EdgeRecord) -> StoreResult<()> {
    if edge.kind != EdgeKind::Follows || edge.tombstone.is_some() {
        return Err(StoreError::EventConflict);
    }
    if edge.from_node != input.follower_profile_node_id {
        return Err(StoreError::Forbidden);
    }
    Ok(())
}
