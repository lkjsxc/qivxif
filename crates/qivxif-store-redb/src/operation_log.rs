use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    records::OperationReceipt,
    store::QivxifStore,
    tables,
};
use qivxif_core::{ActorId, CursorId, NodeId, OperationId};
use qivxif_history::OperationEnvelope;
use redb::ReadableTable;

impl QivxifStore {
    pub fn get_operation(&self, op_id: &OperationId) -> StoreResult<Option<OperationEnvelope>> {
        let tx = self.database.begin_read()?;
        let ops = tx.open_table(tables::OPS)?;
        ops.get(op_id.as_str())?
            .map(|bytes| decode(bytes.value()))
            .transpose()
    }

    pub fn operation_receipt(&self, op_id: &OperationId) -> StoreResult<Option<OperationReceipt>> {
        let tx = self.database.begin_read()?;
        let cursors = tx.open_table(tables::SYNC_CURSORS)?;
        cursors
            .get(op_cursor_key(op_id).as_str())?
            .map(|bytes| {
                let server_cursor = decode(bytes.value())?;
                Ok(OperationReceipt {
                    op_id: op_id.clone(),
                    server_cursor,
                })
            })
            .transpose()
    }

    pub fn list_operations_after_cursor(
        &self,
        cursor: Option<&CursorId>,
        limit: usize,
    ) -> StoreResult<(Vec<OperationEnvelope>, Option<CursorId>, bool)> {
        let tx = self.database.begin_read()?;
        let cursors = tx.open_table(tables::SYNC_CURSORS)?;
        let ops = tx.open_table(tables::OPS)?;
        let after = cursor.map(CursorId::as_str);
        let mut out = Vec::new();
        let mut last_cursor = cursor.cloned();
        let mut has_more = false;
        for item in cursors.iter()? {
            let (key, op_id_bytes) = item?;
            let Some(cursor_text) = key.value().strip_prefix("cursor:") else {
                continue;
            };
            if after.is_some_and(|value| cursor_text <= value) {
                continue;
            }
            if out.len() == limit {
                has_more = true;
                break;
            }
            let op_id: OperationId = decode(op_id_bytes.value())?;
            if let Some(op_bytes) = ops.get(op_id.as_str())? {
                out.push(decode(op_bytes.value())?);
                last_cursor = Some(cursor_text.parse().map_err(|_| StoreError::CursorInvalid)?);
            }
        }
        Ok((out, last_cursor, has_more))
    }
}

pub(crate) fn actor_seq_key(actor_id: &ActorId, seq: u64) -> String {
    format!("{}:{seq:020}", actor_id.as_str())
}

pub(crate) fn node_op_key(node_id: &NodeId, op_id: &OperationId) -> String {
    format!("{}:{}", node_id.as_str(), op_id.as_str())
}

pub(crate) fn insert_operation(
    tx: &redb::WriteTransaction,
    op: &OperationEnvelope,
) -> StoreResult<OperationReceipt> {
    {
        let ops = tx.open_table(tables::OPS)?;
        if ops.get(op.op_id.as_str())?.is_some() {
            return read_receipt(tx, &op.op_id);
        }
    }
    let actor_key = actor_seq_key(&op.actor_id, op.actor_seq);
    {
        let actor_ops = tx.open_table(tables::OPS_BY_ACTOR)?;
        if let Some(existing) = actor_ops.get(actor_key.as_str())? {
            let existing: OperationId = decode(existing.value())?;
            if existing != op.op_id {
                return Err(StoreError::DuplicateActorSeq);
            }
        }
    }
    {
        let cursor = next_cursor(tx)?;
        let mut ops = tx.open_table(tables::OPS)?;
        ops.insert(op.op_id.as_str(), encode(op)?.as_slice())?;
        let mut actor_ops = tx.open_table(tables::OPS_BY_ACTOR)?;
        actor_ops.insert(actor_key.as_str(), encode(&op.op_id)?.as_slice())?;
        let mut node_ops = tx.open_table(tables::OPS_BY_NODE)?;
        for node_id in &op.target_node_ids {
            node_ops.insert(
                node_op_key(node_id, &op.op_id).as_str(),
                encode(&op.op_id)?.as_slice(),
            )?;
        }
        let mut cursors = tx.open_table(tables::SYNC_CURSORS)?;
        cursors.insert(
            op_cursor_key(&op.op_id).as_str(),
            encode(&cursor)?.as_slice(),
        )?;
        cursors.insert(cursor_key(&cursor).as_str(), encode(&op.op_id)?.as_slice())?;
        Ok(OperationReceipt {
            op_id: op.op_id.clone(),
            server_cursor: cursor,
        })
    }
}

pub(crate) fn receipt(op_id: &OperationId) -> OperationReceipt {
    let digest = blake3::hash(op_id.as_str().as_bytes()).to_hex();
    let cursor = format!("cur_{}", &digest.as_str()[..32])
        .parse()
        .expect("cursor digest is lowercase hex");
    OperationReceipt {
        op_id: op_id.clone(),
        server_cursor: cursor,
    }
}

fn read_receipt(tx: &redb::WriteTransaction, op_id: &OperationId) -> StoreResult<OperationReceipt> {
    let cursors = tx.open_table(tables::SYNC_CURSORS)?;
    let Some(cursor_bytes) = cursors.get(op_cursor_key(op_id).as_str())? else {
        return Ok(receipt(op_id));
    };
    Ok(OperationReceipt {
        op_id: op_id.clone(),
        server_cursor: decode(cursor_bytes.value())?,
    })
}

fn next_cursor(tx: &redb::WriteTransaction) -> StoreResult<CursorId> {
    let ops = tx.open_table(tables::OPS)?;
    let mut count = 0_u128;
    for item in ops.iter()? {
        item?;
        count += 1;
    }
    cursor_from_index(count + 1)
}

fn cursor_from_index(index: u128) -> StoreResult<CursorId> {
    format!("cur_{index:032x}")
        .parse()
        .map_err(|_| StoreError::CursorInvalid)
}

fn op_cursor_key(op_id: &OperationId) -> String {
    format!("op:{}", op_id.as_str())
}

fn cursor_key(cursor: &CursorId) -> String {
    format!("cursor:{}", cursor.as_str())
}
