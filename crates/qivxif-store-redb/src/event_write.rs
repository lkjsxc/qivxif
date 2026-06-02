use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    records::EventReceipt,
    tables,
};
use qivxif_core::{ActorId, CursorId, EdgeId, EventId, NodeId};
use qivxif_history::EventEnvelope;
use redb::ReadableTable;

type WriteTx = redb::WriteTransaction;

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

pub(crate) fn insert_event(tx: &WriteTx, event: &EventEnvelope) -> StoreResult<EventReceipt> {
    if let Some(existing) = existing_event(tx, &event.event_id)? {
        if !event_matches_for_replay(&existing, event) {
            return Err(StoreError::EventConflict);
        }
        return read_receipt(tx, &event.event_id);
    }
    let actor_key = actor_seq_key(&event.actor_id, event.actor_seq);
    reject_actor_seq_conflict(tx, event, &actor_key)?;
    insert_new_event(tx, event, &actor_key)
}

pub(crate) fn event_matches_for_replay(
    existing: &EventEnvelope,
    submitted: &EventEnvelope,
) -> bool {
    existing.event_id == submitted.event_id
        && existing.actor_id == submitted.actor_id
        && existing.actor_seq == submitted.actor_seq
        && existing.parents == submitted.parents
        && existing.scope == submitted.scope
        && existing.kind == submitted.kind
        && existing.target_node_ids == submitted.target_node_ids
        && existing.target_edge_ids == submitted.target_edge_ids
        && existing.target_event_ids == submitted.target_event_ids
        && existing.payload == submitted.payload
        && existing.payload_hash == submitted.payload_hash
        && existing.created_at_client == submitted.created_at_client
}

pub(crate) fn event_cursor_key(event_id: &EventId) -> String {
    format!("event:{}", event_id.as_str())
}

pub(crate) fn cursor_key(cursor: &CursorId) -> String {
    format!("cursor:{}", cursor.as_str())
}

fn existing_event(tx: &WriteTx, event_id: &EventId) -> StoreResult<Option<EventEnvelope>> {
    let events = tx.open_table(tables::EVENTS_BY_ID)?;
    events
        .get(event_id.as_str())?
        .map(|bytes| decode(bytes.value()))
        .transpose()
}

fn reject_actor_seq_conflict(
    tx: &WriteTx,
    event: &EventEnvelope,
    actor_key: &str,
) -> StoreResult<()> {
    let actor_events = tx.open_table(tables::EVENT_IDS_BY_ACTOR)?;
    if let Some(existing) = actor_events.get(actor_key)? {
        let existing: EventId = decode(existing.value())?;
        if existing != event.event_id {
            return Err(StoreError::DuplicateActorSeq);
        }
    }
    Ok(())
}

fn insert_new_event(
    tx: &WriteTx,
    event: &EventEnvelope,
    actor_key: &str,
) -> StoreResult<EventReceipt> {
    let sequence = next_sequence(tx)?;
    let cursor = generate_cursor(tx)?;
    let mut events = tx.open_table(tables::EVENTS_BY_ID)?;
    events.insert(event.event_id.as_str(), encode(event)?.as_slice())?;
    let mut actor_events = tx.open_table(tables::EVENT_IDS_BY_ACTOR)?;
    actor_events.insert(actor_key, encode(&event.event_id)?.as_slice())?;
    insert_event_indexes(tx, event)?;
    insert_acceptance_cursor(tx, event, sequence, &cursor)?;
    Ok(EventReceipt {
        event_id: event.event_id.clone(),
        server_cursor: cursor,
    })
}

fn insert_event_indexes(tx: &WriteTx, event: &EventEnvelope) -> StoreResult<()> {
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

fn insert_acceptance_cursor(
    tx: &WriteTx,
    event: &EventEnvelope,
    sequence: u128,
    cursor: &CursorId,
) -> StoreResult<()> {
    let mut acceptance = tx.open_table(tables::EVENT_IDS_BY_ACCEPTANCE)?;
    acceptance.insert(
        acceptance_key(sequence).as_str(),
        encode(&event.event_id)?.as_slice(),
    )?;
    let mut cursors = tx.open_table(tables::SYNC_CURSORS)?;
    cursors.insert(
        event_cursor_key(&event.event_id).as_str(),
        encode(cursor)?.as_slice(),
    )?;
    cursors.insert(cursor_key(cursor).as_str(), encode(&sequence)?.as_slice())?;
    Ok(())
}

fn read_receipt(tx: &WriteTx, event_id: &EventId) -> StoreResult<EventReceipt> {
    let cursors = tx.open_table(tables::SYNC_CURSORS)?;
    let Some(cursor_bytes) = cursors.get(event_cursor_key(event_id).as_str())? else {
        return Err(StoreError::EventConflict);
    };
    Ok(EventReceipt {
        event_id: event_id.clone(),
        server_cursor: decode(cursor_bytes.value())?,
    })
}

fn next_sequence(tx: &WriteTx) -> StoreResult<u128> {
    let acceptance = tx.open_table(tables::EVENT_IDS_BY_ACCEPTANCE)?;
    let mut count = 0_u128;
    for item in acceptance.iter()? {
        item?;
        count += 1;
    }
    Ok(count + 1)
}

fn generate_cursor(tx: &WriteTx) -> StoreResult<CursorId> {
    let cursors = tx.open_table(tables::SYNC_CURSORS)?;
    for _ in 0..16 {
        let cursor = CursorId::generate();
        if cursors.get(cursor_key(&cursor).as_str())?.is_none() {
            return Ok(cursor);
        }
    }
    Err(StoreError::EventConflict)
}

pub(crate) fn acceptance_key(sequence: u128) -> String {
    format!("acceptance:{sequence:039}")
}
