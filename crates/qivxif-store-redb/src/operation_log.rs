use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    records::OperationReceipt,
    store::QivxifStore,
    tables,
};
use qivxif_core::{ActorId, NodeId, OperationId};
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
        if let Some(existing) = ops.get(op.op_id.as_str())? {
            let existing: OperationEnvelope = decode(existing.value())?;
            if existing == *op {
                return Ok(receipt(&op.op_id));
            }
            return Err(StoreError::OperationConflict);
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
    }
    Ok(receipt(&op.op_id))
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
