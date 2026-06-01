use crate::{EdgeRecord, GraphError, GraphResult, NodeRecord, Tombstone};
use qivxif_core::{EdgeId, MetadataMap, NodeId, OperationId};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GraphState {
    pub nodes: BTreeMap<NodeId, NodeRecord>,
    pub edges: BTreeMap<EdgeId, EdgeRecord>,
    pub applied_ops: BTreeSet<OperationId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphOp {
    CreateNode {
        op_id: OperationId,
        record: NodeRecord,
    },
    UpdateMetadata {
        op_id: OperationId,
        node_id: NodeId,
        metadata: MetadataMap,
    },
    TombstoneNode {
        op_id: OperationId,
        node_id: NodeId,
        tombstone: Tombstone,
    },
    CreateEdge {
        op_id: OperationId,
        record: EdgeRecord,
    },
    TombstoneEdge {
        op_id: OperationId,
        edge_id: EdgeId,
        tombstone: Tombstone,
    },
}

pub fn apply_graph_op(mut state: GraphState, op: GraphOp) -> GraphResult<GraphState> {
    let op_id = op_id(&op).clone();
    if state.applied_ops.contains(&op_id) {
        return Ok(state);
    }
    match op {
        GraphOp::CreateNode { record, .. } => {
            if state.nodes.contains_key(&record.id) {
                return Err(GraphError::NodeExists);
            }
            state.nodes.insert(record.id.clone(), record);
        }
        GraphOp::UpdateMetadata {
            node_id, metadata, ..
        } => {
            state
                .nodes
                .get_mut(&node_id)
                .ok_or(GraphError::NodeMissing)?
                .metadata_map = metadata;
        }
        GraphOp::TombstoneNode {
            node_id, tombstone, ..
        } => {
            state
                .nodes
                .get_mut(&node_id)
                .ok_or(GraphError::NodeMissing)?
                .tombstone = Some(tombstone);
        }
        GraphOp::CreateEdge { record, .. } => {
            if !state.nodes.contains_key(&record.from_node)
                || !state.nodes.contains_key(&record.to_node)
            {
                return Err(GraphError::NodeMissing);
            }
            if state.edges.contains_key(&record.id) {
                return Err(GraphError::EdgeExists);
            }
            state.edges.insert(record.id.clone(), record);
        }
        GraphOp::TombstoneEdge {
            edge_id, tombstone, ..
        } => {
            state
                .edges
                .get_mut(&edge_id)
                .ok_or(GraphError::EdgeMissing)?
                .tombstone = Some(tombstone);
        }
    }
    state.applied_ops.insert(op_id);
    Ok(state)
}

fn op_id(op: &GraphOp) -> &OperationId {
    match op {
        GraphOp::CreateNode { op_id, .. }
        | GraphOp::UpdateMetadata { op_id, .. }
        | GraphOp::TombstoneNode { op_id, .. }
        | GraphOp::CreateEdge { op_id, .. }
        | GraphOp::TombstoneEdge { op_id, .. } => op_id,
    }
}

#[cfg(test)]
mod tests;
