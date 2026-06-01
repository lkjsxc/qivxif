use crate::{
    StoreError, StoreResult, codec::decode, records::EventReceipt, store::QivxifStore, tables,
};
use qivxif_auth::{AuthContext, can_read};
use qivxif_core::{CursorId, EventId};
use qivxif_history::EventEnvelope;
use redb::ReadableTable;

pub(crate) use crate::event_write::{event_cursor_key, insert_event};

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
        let cursors = tx.open_table(tables::SYNC_CURSORS)?;
        let events = tx.open_table(tables::EVENTS_BY_ID)?;
        let after = cursor.map(CursorId::as_str);
        let mut out = Vec::new();
        let mut last_cursor = cursor.cloned();
        let mut has_more = false;
        for item in cursors.iter()? {
            let (key, event_id_bytes) = item?;
            let Some(cursor_text) = key.value().strip_prefix("cursor:") else {
                continue;
            };
            if after.is_some_and(|value| cursor_text <= value) {
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
                out.push(event);
                last_cursor = Some(parse_cursor(cursor_text)?);
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

fn parse_cursor(value: &str) -> StoreResult<CursorId> {
    value.parse().map_err(|_| StoreError::CursorInvalid)
}
