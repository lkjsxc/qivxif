use crate::{
    StoreError, StoreResult,
    codec::decode,
    event_log::insert_event,
    feed_support::ensure_session_actor,
    follow_support::{
        active_edge_kind, edge_by_id, insert_edge_with_indexes, social_edge_event, update_edge,
    },
    records::EventReceipt,
    store::QivxifStore,
};
use qivxif_auth::{AuthContext, can_read};
use qivxif_core::{ActorId, EdgeId, EventId, MetadataMap, NodeId, ServerTime, UserId};
use qivxif_graph::{EdgeRecord, NodeKind, Tombstone};
use qivxif_history::EventKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModerationAction {
    Mute,
    Block,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModerationInput {
    pub event_id: EventId,
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
    pub event_id: EventId,
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
    pub receipt: EventReceipt,
}

impl QivxifStore {
    pub fn create_moderation_edge(
        &self,
        auth: &AuthContext,
        input: ModerationInput,
    ) -> StoreResult<ModerationResult> {
        if self.get_event(&input.event_id)?.is_some() {
            return self.replay_moderation(&input.event_id, input.action.create_event());
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
        let event = social_edge_event(
            &input.event_id,
            input.actor_seq,
            &input.actor_id,
            &input.actor_user_id,
            input.action.create_event(),
            &edge,
        )?;
        let receipt = insert_event(&tx, &event)?;
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
        if self.get_event(&input.event_id)?.is_some() {
            return self.replay_moderation(&input.event_id, input.action.clear_event());
        }
        ensure_session_actor(auth, &input.actor_id, &input.actor_user_id)?;
        let tx = self.database.begin_write()?;
        let mut edge = edge_by_id(&tx, &input.edge_id)?.ok_or(StoreError::EventConflict)?;
        if edge.kind != input.action.edge_kind() || edge.tombstone.is_some() {
            return Err(StoreError::EventConflict);
        }
        if edge.from_node != input.actor_profile_node_id {
            return Err(StoreError::Forbidden);
        }
        edge.tombstone = Some(Tombstone {
            by: input.actor_id.clone(),
            reason: input.action.clear_reason().to_owned(),
        });
        let event = social_edge_event(
            &input.event_id,
            input.actor_seq,
            &input.actor_id,
            &input.actor_user_id,
            input.action.clear_event(),
            &edge,
        )?;
        let receipt = insert_event(&tx, &event)?;
        update_edge(&tx, &edge)?;
        tx.commit()?;
        Ok(ModerationResult { edge, receipt })
    }

    fn replay_moderation(
        &self,
        event_id: &EventId,
        expected: EventKind,
    ) -> StoreResult<ModerationResult> {
        let event = self.get_event(event_id)?.ok_or(StoreError::EventConflict)?;
        if event.kind != expected {
            return Err(StoreError::EventConflict);
        }
        let edge = decode(&event.payload.bytes)?;
        let receipt = self
            .event_receipt(event_id)?
            .ok_or(StoreError::EventConflict)?;
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
        return Err(StoreError::InvalidEvent);
    }
    let Some(from) = store.get_node(&input.actor_profile_node_id)? else {
        return Err(StoreError::NodeMissing);
    };
    let Some(to) = store.get_node(&input.target_profile_node_id)? else {
        return Err(StoreError::NodeMissing);
    };
    if from.kind != NodeKind::Profile || to.kind != NodeKind::Profile {
        return Err(StoreError::InvalidEvent);
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
