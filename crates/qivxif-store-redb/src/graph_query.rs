use crate::{StoreError, StoreResult, codec::decode, store::QivxifStore, tables};
use qivxif_auth::{AuthContext, can_read};
use qivxif_core::{EdgeId, NodeId};
use qivxif_graph::{EdgeRecord, GraphProjection, NodeRecord, project_node};
use redb::ReadableTable;
use std::collections::{BTreeSet, VecDeque};

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

    pub fn get_neighborhood(
        &self,
        auth: &AuthContext,
        center: &NodeId,
        depth: usize,
        limit: usize,
    ) -> StoreResult<GraphProjection> {
        let Some(center_node) = self.get_node(center)? else {
            return Err(StoreError::NodeMissing);
        };
        if !can_read(auth, &center_node) {
            return Err(StoreError::Forbidden);
        }
        let visible = self.visible_neighborhood_ids(auth, center, depth, limit)?;
        let mut projections = Vec::new();
        for node_id in &visible {
            let Some(node) = self.get_node(node_id)? else {
                continue;
            };
            let edges = self
                .list_edges_for_node(node_id)?
                .into_iter()
                .filter(|edge| {
                    visible.contains(&edge.from_node) && visible.contains(&edge.to_node)
                });
            if let Some(projection) = project_node(&node, edges, false) {
                projections.push(projection);
            }
        }
        Ok(GraphProjection { nodes: projections })
    }

    fn visible_neighborhood_ids(
        &self,
        auth: &AuthContext,
        center: &NodeId,
        depth: usize,
        limit: usize,
    ) -> StoreResult<BTreeSet<NodeId>> {
        let mut visible = BTreeSet::new();
        let mut queue = VecDeque::from([(center.clone(), 0_usize)]);
        while let Some((node_id, distance)) = queue.pop_front() {
            if visible.len() >= limit || visible.contains(&node_id) {
                continue;
            }
            let Some(node) = self.get_node(&node_id)? else {
                continue;
            };
            if !can_read(auth, &node) {
                continue;
            }
            visible.insert(node_id.clone());
            if distance >= depth {
                continue;
            }
            for edge in self.list_edges_for_node(&node_id)? {
                queue.push_back((other_endpoint(&edge, &node_id), distance + 1));
            }
        }
        Ok(visible)
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

fn other_endpoint(edge: &EdgeRecord, node_id: &NodeId) -> NodeId {
    if &edge.from_node == node_id {
        edge.to_node.clone()
    } else {
        edge.from_node.clone()
    }
}
