use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    event_log::insert_event,
    event_write::event_matches_for_replay,
    records::EventReceipt,
    store::QivxifStore,
    tables,
    text_store::TextApplyInput,
};
use qivxif_auth::{AuthContext, Viewer, can_link};
use qivxif_graph::{EdgeRecord, NodeRecord};
use qivxif_history::{EventEnvelope, EventKind, text::TextEvent, validate_event_envelope};
use redb::ReadableTable;

impl QivxifStore {
    pub fn accept_event(
        &self,
        auth: &AuthContext,
        event: EventEnvelope,
    ) -> StoreResult<EventReceipt> {
        validate_event_envelope(event.clone()).map_err(|_| StoreError::InvalidEvent)?;
        if !actor_matches(auth, &event) {
            return Err(StoreError::Forbidden);
        }
        if let Some(receipt) = self.replay_receipt_if_match(&event)? {
            return Ok(receipt);
        }
        match event.kind {
            EventKind::NodeCreate => self.accept_node_create_event(auth, event),
            EventKind::EdgeCreate => self.accept_edge_create_event(auth, event),
            EventKind::TextInsert | EventKind::TextDelete | EventKind::TextRestore => {
                self.accept_text_event(auth, event)
            }
            EventKind::TileLayoutSet => self.accept_tile_event(auth, event),
            _ => Err(StoreError::UnknownEventKind),
        }
    }

    fn replay_receipt_if_match(&self, event: &EventEnvelope) -> StoreResult<Option<EventReceipt>> {
        let Some(existing) = self.get_event(&event.event_id)? else {
            return Ok(None);
        };
        if !event_matches_for_replay(&existing, event) {
            return Err(StoreError::EventConflict);
        }
        self.event_receipt(&event.event_id)?
            .map(Some)
            .ok_or(StoreError::EventConflict)
    }

    fn accept_node_create_event(
        &self,
        auth: &AuthContext,
        event: EventEnvelope,
    ) -> StoreResult<EventReceipt> {
        let node: NodeRecord = decode(&event.payload.bytes)?;
        if !actor_matches(auth, &event) || Some(&node.owner_user_id) != auth.user_id() {
            return Err(StoreError::Forbidden);
        }
        if node.created_by != event.actor_id || !event.target_node_ids.contains(&node.id) {
            return Err(StoreError::InvalidEvent);
        }
        let tx = self.database.begin_write()?;
        let receipt = insert_event(&tx, &event)?;
        {
            let mut nodes = tx.open_table(tables::NODES)?;
            if let Some(existing) = nodes.get(node.id.as_str())? {
                let existing: NodeRecord = decode(existing.value())?;
                if existing != node {
                    return Err(StoreError::NodeExists);
                }
            } else {
                nodes.insert(node.id.as_str(), encode(&node)?.as_slice())?;
            }
        }
        tx.commit()?;
        Ok(receipt)
    }

    fn accept_edge_create_event(
        &self,
        auth: &AuthContext,
        event: EventEnvelope,
    ) -> StoreResult<EventReceipt> {
        let edge: EdgeRecord = decode(&event.payload.bytes)?;
        let Some(from) = self.get_node(&edge.from_node)? else {
            return Err(StoreError::NodeMissing);
        };
        let Some(to) = self.get_node(&edge.to_node)? else {
            return Err(StoreError::NodeMissing);
        };
        if !actor_matches(auth, &event)
            || edge.created_by != event.actor_id
            || !can_link(auth, &from, &to)
        {
            return Err(StoreError::Forbidden);
        }
        let tx = self.database.begin_write()?;
        let receipt = insert_event(&tx, &event)?;
        {
            let mut edges = tx.open_table(tables::EDGES)?;
            if let Some(existing) = edges.get(edge.id.as_str())? {
                let existing: EdgeRecord = decode(existing.value())?;
                if existing != edge {
                    return Err(StoreError::EdgeExists);
                }
            } else {
                edges.insert(edge.id.as_str(), encode(&edge)?.as_slice())?;
                let mut by_from = tx.open_table(tables::EDGES_BY_FROM)?;
                by_from.insert(
                    index_key(edge.from_node.as_str(), edge.id.as_str()).as_str(),
                    encode(&edge.id)?.as_slice(),
                )?;
                let mut by_to = tx.open_table(tables::EDGES_BY_TO)?;
                by_to.insert(
                    index_key(edge.to_node.as_str(), edge.id.as_str()).as_str(),
                    encode(&edge.id)?.as_slice(),
                )?;
            }
        }
        tx.commit()?;
        Ok(receipt)
    }

    fn accept_text_event(
        &self,
        auth: &AuthContext,
        event: EventEnvelope,
    ) -> StoreResult<EventReceipt> {
        if self.get_event(&event.event_id)?.is_some() {
            return self
                .event_receipt(&event.event_id)?
                .ok_or(StoreError::EventConflict);
        }
        if !actor_matches(auth, &event) {
            return Err(StoreError::Forbidden);
        }
        let Some(node_id) = event.target_node_ids.first().cloned() else {
            return Err(StoreError::InvalidEvent);
        };
        let text_event: TextEvent =
            serde_json::from_slice(&event.payload.bytes).map_err(|_| StoreError::InvalidEvent)?;
        if text_event.event_id != event.event_id {
            return Err(StoreError::InvalidEvent);
        }
        self.apply_text_event(
            auth,
            TextApplyInput {
                actor_id: event.actor_id,
                actor_seq: event.actor_seq,
                node_id,
                event: text_event,
            },
        )
        .map(|result| result.receipt)
    }

    fn accept_tile_event(
        &self,
        auth: &AuthContext,
        event: EventEnvelope,
    ) -> StoreResult<EventReceipt> {
        let layout =
            serde_json::from_slice(&event.payload.bytes).map_err(|_| StoreError::InvalidEvent)?;
        self.accept_tile_layout_event(auth, event, layout)
            .map(|result| result.receipt)
    }
}

fn actor_matches(auth: &AuthContext, event: &EventEnvelope) -> bool {
    matches!(
        &auth.viewer,
        Viewer::Session { actor_id, .. } if actor_id == &event.actor_id
    )
}

fn index_key(node_id: &str, edge_id: &str) -> String {
    format!("{node_id}:{edge_id}")
}
