use crate::{StoreResult, codec::decode, store::QivxifStore, tables};
use qivxif_core::{EdgeId, NodeId};
use qivxif_graph::{EdgeRecord, NodeRecord};
use redb::ReadableTable;

impl QivxifStore {
    pub fn get_node(&self, node_id: &NodeId) -> StoreResult<Option<NodeRecord>> {
        let tx = self.database.begin_read()?;
        let nodes = tx.open_table(tables::NODES)?;
        nodes
            .get(node_id.as_str())?
            .map(|bytes| decode(bytes.value()))
            .transpose()
    }

    pub fn get_edge(&self, edge_id: &EdgeId) -> StoreResult<Option<EdgeRecord>> {
        let tx = self.database.begin_read()?;
        let edges = tx.open_table(tables::EDGES)?;
        edges
            .get(edge_id.as_str())?
            .map(|bytes| decode(bytes.value()))
            .transpose()
    }

    pub fn list_edges_from(&self, node_id: &NodeId) -> StoreResult<Vec<EdgeRecord>> {
        self.list_edge_index(tables::EDGES_BY_FROM, node_id)
    }

    pub fn list_edges_to(&self, node_id: &NodeId) -> StoreResult<Vec<EdgeRecord>> {
        self.list_edge_index(tables::EDGES_BY_TO, node_id)
    }

    pub(crate) fn list_edges_for_node(&self, node_id: &NodeId) -> StoreResult<Vec<EdgeRecord>> {
        let mut edges = self.list_edges_from(node_id)?;
        edges.extend(self.list_edges_to(node_id)?);
        Ok(edges)
    }

    fn list_edge_index(
        &self,
        index: redb::TableDefinition<&str, &[u8]>,
        node_id: &NodeId,
    ) -> StoreResult<Vec<EdgeRecord>> {
        let tx = self.database.begin_read()?;
        let index = tx.open_table(index)?;
        let edges = tx.open_table(tables::EDGES)?;
        let prefix = format!("{}:", node_id.as_str());
        let mut records = Vec::new();
        for item in index.iter()? {
            let (key, edge_id_bytes) = item?;
            if !key.value().starts_with(&prefix) {
                continue;
            }
            let edge_id: EdgeId = decode(edge_id_bytes.value())?;
            if let Some(edge_bytes) = edges.get(edge_id.as_str())? {
                records.push(decode(edge_bytes.value())?);
            }
        }
        Ok(records)
    }
}
