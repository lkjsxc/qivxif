use crate::{
    StoreError, StoreResult, codec::decode, operation_log::op_cursor_key, store::QivxifStore,
    tables,
};
use qivxif_auth::{AuthContext, can_read};
use qivxif_core::{CursorId, NodeId, OperationId};
use qivxif_history::OperationEnvelope;
use redb::ReadableTable;

impl QivxifStore {
    pub fn list_operations_for_node(
        &self,
        auth: &AuthContext,
        node_id: &NodeId,
        limit: usize,
    ) -> StoreResult<Vec<OperationEnvelope>> {
        let Some(node) = self.get_node(node_id)? else {
            return Err(StoreError::NodeMissing);
        };
        if !can_read(auth, &node) {
            return Err(StoreError::Forbidden);
        }
        let tx = self.database.begin_read()?;
        let node_ops = tx.open_table(tables::OPS_BY_NODE)?;
        let cursors = tx.open_table(tables::SYNC_CURSORS)?;
        let ops = tx.open_table(tables::OPS)?;
        let prefix = format!("{}:", node_id.as_str());
        let mut found = Vec::new();
        for item in node_ops.iter()? {
            let (key, op_id_bytes) = item?;
            if !key.value().starts_with(&prefix) {
                continue;
            }
            let op_id: OperationId = decode(op_id_bytes.value())?;
            let Some(cursor_bytes) = cursors.get(op_cursor_key(&op_id).as_str())? else {
                return Err(StoreError::CursorInvalid);
            };
            let cursor: CursorId = decode(cursor_bytes.value())?;
            if let Some(op_bytes) = ops.get(op_id.as_str())? {
                found.push((cursor, decode(op_bytes.value())?));
            }
        }
        found.sort_by(|left, right| left.0.cmp(&right.0));
        Ok(found.into_iter().take(limit).map(|(_, op)| op).collect())
    }
}
