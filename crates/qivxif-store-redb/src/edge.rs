use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    graph::{EdgeCreateInput, EdgeCreateResult, edge_operation},
    operation_log::{insert_operation, receipt},
    store::QivxifStore,
    tables,
};
use qivxif_auth::{AuthContext, can_link};
use qivxif_core::ServerTime;
use qivxif_graph::EdgeRecord;
use redb::ReadableTable;

impl QivxifStore {
    pub fn create_edge(
        &self,
        auth: &AuthContext,
        input: EdgeCreateInput,
    ) -> StoreResult<EdgeCreateResult> {
        if self.get_operation(&input.op_id)?.is_some() {
            let edge = self
                .get_edge(&input.edge_id)?
                .ok_or(StoreError::OperationConflict)?;
            return Ok(EdgeCreateResult {
                edge,
                receipt: receipt(&input.op_id),
            });
        }
        let Some(from) = self.get_node(&input.from_node)? else {
            return Err(StoreError::NodeMissing);
        };
        let Some(to) = self.get_node(&input.to_node)? else {
            return Err(StoreError::NodeMissing);
        };
        if !can_link(auth, &from, &to) {
            return Err(StoreError::Forbidden);
        }
        let edge = EdgeRecord {
            id: input.edge_id.clone(),
            from_node: input.from_node,
            to_node: input.to_node,
            kind: input.kind,
            created_by: input.actor_id.clone(),
            created_at: ServerTime::now(),
            metadata_map: input.metadata_map,
            tombstone: None,
        };
        let op = edge_operation(&input.op_id, input.actor_seq, &input.actor_id, &edge)?;
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
                    edge_index(&edge.from_node, &edge.id).as_str(),
                    encode(&edge.id)?.as_slice(),
                )?;
                let mut by_to = tx.open_table(tables::EDGES_BY_TO)?;
                by_to.insert(
                    edge_index(&edge.to_node, &edge.id).as_str(),
                    encode(&edge.id)?.as_slice(),
                )?;
            }
        }
        tx.commit()?;
        Ok(EdgeCreateResult { edge, receipt })
    }
}

fn edge_index(node_id: &qivxif_core::NodeId, edge_id: &qivxif_core::EdgeId) -> String {
    format!("{}:{}", node_id.as_str(), edge_id.as_str())
}
