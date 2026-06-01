use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    records::EventReceipt,
    tables,
};
use qivxif_core::{ActorId, CursorId, EdgeId, EventId, NodeId};
use qivxif_history::EventEnvelope;
use redb::ReadableTable;

pub(crate) fn actor_seq_key(actor_id: &ActorId, seq: u64) -> String {
    format!("{}:{seq:020}", actor_id.as_str())
}

pub(crate) fn node_event_key(node_id: &NodeId, event_id: &EventId) -> String {
    format!("{}:{}", node_id.as_str(), event_id.as_str())
}

pub(crate) fn edge_event_key(edge_id: &EdgeId, event_id: &EventId) -> String {
    format!("{}:{}", edge_id.as_str(), event_id.as_str())
}

pub(crate) fn event_target_key(target_event_id: &EventId, event_id: &EventId) -> String {
    format!("{}:{}", target_event_id.as_str(), event_id.as_str())
}

pub(crate) fn parent_event_key(parent_id: &EventId, event_id: &EventId) -> String {
    format!("{}:{}", parent_id.as_str(), event_id.as_str())
}

pub(crate) fn insert_event(
    tx: &redb::WriteTransaction,
    event: &EventEnvelope,
) -> StoreResult<EventReceipt> {
    {
        let events = tx.open_table(tables::EVENTS_BY_ID)?;
        if events.get(event.event_id.as_str())?.is_some() {
            return read_receipt(tx, &event.event_id);
        }
    }
    let actor_key = actor_seq_key(&event.actor_id, event.actor_seq);
    {
        let actor_events = tx.open_table(tables::EVENT_IDS_BY_ACTOR)?;
        if let Some(existing) = actor_events.get(actor_key.as_str())? {
            let existing: EventId = decode(existing.value())?;
            if existing != event.event_id {
                return Err(StoreError::DuplicateActorSeq);
            }
        }
    }
    insert_new_event(tx, event, &actor_key)
}

fn insert_new_event(
    tx: &redb::WriteTransaction,
    event: &EventEnvelope,
    actor_key: &str,
) -> StoreResult<EventReceipt> {
    let cursor = next_cursor(tx)?;
    let mut events = tx.open_table(tables::EVENTS_BY_ID)?;
    events.insert(event.event_id.as_str(), encode(event)?.as_slice())?;
    let mut actor_events = tx.open_table(tables::EVENT_IDS_BY_ACTOR)?;
    actor_events.insert(actor_key, encode(&event.event_id)?.as_slice())?;
    insert_event_indexes(tx, event)?;
    let mut cursors = tx.open_table(tables::SYNC_CURSORS)?;
    cursors.insert(
        event_cursor_key(&event.event_id).as_str(),
        encode(&cursor)?.as_slice(),
    )?;
    cursors.insert(
        cursor_key(&cursor).as_str(),
        encode(&event.event_id)?.as_slice(),
    )?;
    Ok(EventReceipt {
        event_id: event.event_id.clone(),
        server_cursor: cursor,
    })
}

fn insert_event_indexes(tx: &redb::WriteTransaction, event: &EventEnvelope) -> StoreResult<()> {
    let mut parent_events = tx.open_table(tables::EVENT_IDS_BY_PARENT)?;
    for parent_id in &event.parents {
        parent_events.insert(
            parent_event_key(parent_id, &event.event_id).as_str(),
            encode(&event.event_id)?.as_slice(),
        )?;
    }
    let mut node_events = tx.open_table(tables::EVENT_IDS_BY_TARGET_NODE)?;
    for node_id in &event.target_node_ids {
        node_events.insert(
            node_event_key(node_id, &event.event_id).as_str(),
            encode(&event.event_id)?.as_slice(),
        )?;
    }
    let mut edge_events = tx.open_table(tables::EVENT_IDS_BY_TARGET_EDGE)?;
    for edge_id in &event.target_edge_ids {
        edge_events.insert(
            edge_event_key(edge_id, &event.event_id).as_str(),
            encode(&event.event_id)?.as_slice(),
        )?;
    }
    let mut target_events = tx.open_table(tables::EVENT_IDS_BY_TARGET_EVENT)?;
    for target_event_id in &event.target_event_ids {
        target_events.insert(
            event_target_key(target_event_id, &event.event_id).as_str(),
            encode(&event.event_id)?.as_slice(),
        )?;
    }
    Ok(())
}

pub(crate) fn receipt(event_id: &EventId) -> EventReceipt {
    let digest = blake3::hash(event_id.as_str().as_bytes()).to_hex();
    let cursor = format!("cur_{}", digest.as_str())
        .parse()
        .expect("cursor digest is lowercase hex");
    EventReceipt {
        event_id: event_id.clone(),
        server_cursor: cursor,
    }
}

fn read_receipt(tx: &redb::WriteTransaction, event_id: &EventId) -> StoreResult<EventReceipt> {
    let cursors = tx.open_table(tables::SYNC_CURSORS)?;
    let Some(cursor_bytes) = cursors.get(event_cursor_key(event_id).as_str())? else {
        return Ok(receipt(event_id));
    };
    Ok(EventReceipt {
        event_id: event_id.clone(),
        server_cursor: decode(cursor_bytes.value())?,
    })
}

fn next_cursor(tx: &redb::WriteTransaction) -> StoreResult<CursorId> {
    let events = tx.open_table(tables::EVENTS_BY_ID)?;
    let mut count = 0_u128;
    for item in events.iter()? {
        item?;
        count += 1;
    }
    cursor_from_index(count + 1)
}

fn cursor_from_index(index: u128) -> StoreResult<CursorId> {
    format!("cur_{index:064x}")
        .parse()
        .map_err(|_| StoreError::CursorInvalid)
}

pub(crate) fn event_cursor_key(event_id: &EventId) -> String {
    format!("event:{}", event_id.as_str())
}

fn cursor_key(cursor: &CursorId) -> String {
    format!("cursor:{}", cursor.as_str())
}
