use crate::{
    StoreError, StoreResult, codec::decode, records::EventReceipt, store::QivxifStore, tables,
};
use qivxif_auth::{AuthContext, can_read};
use qivxif_core::{CursorId, EventId};
use qivxif_history::EventEnvelope;
use redb::ReadableTable;

pub(crate) use crate::event_write::{cursor_key, event_cursor_key, insert_event};

impl QivxifStore {
    pub fn get_event(&self, event_id: &EventId) -> StoreResult<Option<EventEnvelope>> {
        let tx = self.database.begin_read()?;
        let events = tx.open_table(tables::EVENTS_BY_ID)?;
        events
            .get(event_id.as_str())?
            .map(|bytes| decode(bytes.value()))
            .transpose()
    }

    pub fn event_receipt(&self, event_id: &EventId) -> StoreResult<Option<EventReceipt>> {
        let tx = self.database.begin_read()?;
        let cursors = tx.open_table(tables::SYNC_CURSORS)?;
        cursors
            .get(event_cursor_key(event_id).as_str())?
            .map(|bytes| {
                let server_cursor = decode(bytes.value())?;
                Ok(EventReceipt {
                    event_id: event_id.clone(),
                    server_cursor,
                })
            })
            .transpose()
    }

    pub fn list_events_after_cursor(
        &self,
        auth: &AuthContext,
        cursor: Option<&CursorId>,
        limit: usize,
    ) -> StoreResult<(Vec<EventEnvelope>, Option<CursorId>, bool)> {
        let tx = self.database.begin_read()?;
        let after = cursor_sequence(&tx, cursor)?;
        let acceptance = tx.open_table(tables::EVENT_IDS_BY_ACCEPTANCE)?;
        let events = tx.open_table(tables::EVENTS_BY_ID)?;
        let cursors = tx.open_table(tables::SYNC_CURSORS)?;
        let mut out = Vec::new();
        let mut last_cursor = cursor.cloned();
        let mut has_more = false;
        for item in acceptance.iter()? {
            let (key, event_id_bytes) = item?;
            if sequence_from_key(key.value())? <= after {
                continue;
            }
            let event_id: EventId = decode(event_id_bytes.value())?;
            if let Some(event_bytes) = events.get(event_id.as_str())? {
                let event = decode(event_bytes.value())?;
                if !self.can_pull_event(auth, &event)? {
                    continue;
                }
                if out.len() == limit {
                    has_more = true;
                    break;
                }
                last_cursor = event_cursor(&cursors, &event_id)?.or(last_cursor);
                out.push(event);
            }
        }
        Ok((out, last_cursor, has_more))
    }

    fn can_pull_event(&self, auth: &AuthContext, event: &EventEnvelope) -> StoreResult<bool> {
        if event.target_node_ids.is_empty() {
            return Ok(false);
        }
        for node_id in &event.target_node_ids {
            let Some(node) = self.get_node(node_id)? else {
                return Ok(false);
            };
            if !can_read(auth, &node) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

fn cursor_sequence(tx: &redb::ReadTransaction, cursor: Option<&CursorId>) -> StoreResult<u128> {
    let Some(cursor) = cursor else {
        return Ok(0);
    };
    let cursors = tx.open_table(tables::SYNC_CURSORS)?;
    let Some(sequence_bytes) = cursors.get(cursor_key(cursor).as_str())? else {
        return Err(StoreError::CursorInvalid);
    };
    decode(sequence_bytes.value())
}

fn event_cursor(
    cursors: &redb::ReadOnlyTable<&str, &[u8]>,
    event_id: &EventId,
) -> StoreResult<Option<CursorId>> {
    cursors
        .get(event_cursor_key(event_id).as_str())?
        .map(|bytes| decode(bytes.value()))
        .transpose()
}

fn sequence_from_key(key: &str) -> StoreResult<u128> {
    key.strip_prefix("acceptance:")
        .ok_or(StoreError::CursorInvalid)?
        .parse()
        .map_err(|_| StoreError::CursorInvalid)
}
