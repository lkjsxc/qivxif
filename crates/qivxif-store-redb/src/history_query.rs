use crate::{
    StoreError, StoreResult,
    codec::decode,
    event_log::{cursor_key, event_cursor_key},
    store::QivxifStore,
    tables,
};
use qivxif_auth::{AuthContext, can_read};
use qivxif_core::{EventId, NodeId};
use qivxif_history::EventEnvelope;
use redb::ReadableTable;

impl QivxifStore {
    pub fn list_events_for_node(
        &self,
        auth: &AuthContext,
        node_id: &NodeId,
        limit: usize,
    ) -> StoreResult<Vec<EventEnvelope>> {
        let Some(node) = self.get_node(node_id)? else {
            return Err(StoreError::NodeMissing);
        };
        if !can_read(auth, &node) {
            return Err(StoreError::Forbidden);
        }
        let tx = self.database.begin_read()?;
        let node_events = tx.open_table(tables::EVENT_IDS_BY_TARGET_NODE)?;
        let cursors = tx.open_table(tables::SYNC_CURSORS)?;
        let events = tx.open_table(tables::EVENTS_BY_ID)?;
        let prefix = format!("{}:", node_id.as_str());
        let mut found = Vec::new();
        for item in node_events.iter()? {
            let (key, event_id_bytes) = item?;
            if !key.value().starts_with(&prefix) {
                continue;
            }
            let event_id: EventId = decode(event_id_bytes.value())?;
            let Some(cursor_bytes) = cursors.get(event_cursor_key(&event_id).as_str())? else {
                return Err(StoreError::CursorInvalid);
            };
            let cursor = decode(cursor_bytes.value())?;
            let Some(sequence_bytes) = cursors.get(cursor_key(&cursor).as_str())? else {
                return Err(StoreError::CursorInvalid);
            };
            let sequence: u128 = decode(sequence_bytes.value())?;
            if let Some(event_bytes) = events.get(event_id.as_str())? {
                found.push((sequence, decode(event_bytes.value())?));
            }
        }
        found.sort_by(|left, right| left.0.cmp(&right.0));
        Ok(found
            .into_iter()
            .take(limit)
            .map(|(_, event)| event)
            .collect())
    }
}
