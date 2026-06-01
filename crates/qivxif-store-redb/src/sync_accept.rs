use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    operation_log::insert_operation,
    records::OperationReceipt,
    store::QivxifStore,
    tables,
};
use qivxif_auth::{AuthContext, Viewer, can_link};
use qivxif_graph::{EdgeRecord, NodeRecord};
use qivxif_history::{OperationEnvelope, OperationKind, validate_operation_envelope};
use redb::ReadableTable;

impl QivxifStore {
    pub fn accept_operation(
        &self,
        auth: &AuthContext,
        op: OperationEnvelope,
    ) -> StoreResult<OperationReceipt> {
        validate_operation_envelope(op.clone()).map_err(|_| StoreError::InvalidOperation)?;
        match op.kind {
            OperationKind::NodeCreate => self.accept_node_create_op(auth, op),
            OperationKind::EdgeCreate => self.accept_edge_create_op(auth, op),
            _ => Err(StoreError::UnknownOperationKind),
        }
    }

    fn accept_node_create_op(
        &self,
        auth: &AuthContext,
        op: OperationEnvelope,
    ) -> StoreResult<OperationReceipt> {
        let node: NodeRecord = decode(&op.payload.bytes)?;
        if !actor_matches(auth, &op) || Some(&node.owner_user_id) != auth.user_id() {
            return Err(StoreError::Forbidden);
        }
        if node.created_by != op.actor_id || !op.target_node_ids.contains(&node.id) {
            return Err(StoreError::InvalidOperation);
        }
        let tx = self.database.begin_write()?;
        let receipt = insert_operation(&tx, &op)?;
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

    fn accept_edge_create_op(
        &self,
        auth: &AuthContext,
        op: OperationEnvelope,
    ) -> StoreResult<OperationReceipt> {
        let edge: EdgeRecord = decode(&op.payload.bytes)?;
        let Some(from) = self.get_node(&edge.from_node)? else {
            return Err(StoreError::NodeMissing);
        };
        let Some(to) = self.get_node(&edge.to_node)? else {
            return Err(StoreError::NodeMissing);
        };
        if !actor_matches(auth, &op)
            || edge.created_by != op.actor_id
            || !can_link(auth, &from, &to)
        {
            return Err(StoreError::Forbidden);
        }
        let tx = self.database.begin_write()?;
        let receipt = insert_operation(&tx, &op)?;
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
}

fn actor_matches(auth: &AuthContext, op: &OperationEnvelope) -> bool {
    matches!(
        &auth.viewer,
        Viewer::Session { actor_id, .. } if actor_id == &op.actor_id
    )
}

fn index_key(node_id: &str, edge_id: &str) -> String {
    format!("{node_id}:{edge_id}")
}
