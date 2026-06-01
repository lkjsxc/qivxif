use crate::{EdgeRecord, GraphError, GraphResult, NodeRecord, Tombstone};
use qivxif_core::{EdgeId, EventId, MetadataMap, NodeId};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GraphState {
    pub nodes: BTreeMap<NodeId, NodeRecord>,
    pub edges: BTreeMap<EdgeId, EdgeRecord>,
    pub applied_events: BTreeSet<EventId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphEvent {
    CreateNode {
        event_id: EventId,
        record: NodeRecord,
    },
    UpdateMetadata {
        event_id: EventId,
        node_id: NodeId,
        metadata: MetadataMap,
    },
    TombstoneNode {
        event_id: EventId,
        node_id: NodeId,
        tombstone: Tombstone,
    },
    CreateEdge {
        event_id: EventId,
        record: EdgeRecord,
    },
    TombstoneEdge {
        event_id: EventId,
        edge_id: EdgeId,
        tombstone: Tombstone,
    },
}

pub fn apply_graph_event(mut state: GraphState, event: GraphEvent) -> GraphResult<GraphState> {
    let event_id = event_id(&event).clone();
    if state.applied_events.contains(&event_id) {
        return Ok(state);
    }
    match event {
        GraphEvent::CreateNode { record, .. } => {
            if state.nodes.contains_key(&record.id) {
                return Err(GraphError::NodeExists);
            }
            state.nodes.insert(record.id.clone(), record);
        }
        GraphEvent::UpdateMetadata {
            node_id, metadata, ..
        } => {
            state
                .nodes
                .get_mut(&node_id)
                .ok_or(GraphError::NodeMissing)?
                .metadata_map = metadata;
        }
        GraphEvent::TombstoneNode {
            node_id, tombstone, ..
        } => {
            state
                .nodes
                .get_mut(&node_id)
                .ok_or(GraphError::NodeMissing)?
                .tombstone = Some(tombstone);
        }
        GraphEvent::CreateEdge { record, .. } => {
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
        GraphEvent::TombstoneEdge {
            edge_id, tombstone, ..
        } => {
            state
                .edges
                .get_mut(&edge_id)
                .ok_or(GraphError::EdgeMissing)?
                .tombstone = Some(tombstone);
        }
    }
    state.applied_events.insert(event_id);
    Ok(state)
}

fn event_id(event: &GraphEvent) -> &EventId {
    match event {
        GraphEvent::CreateNode { event_id, .. }
        | GraphEvent::UpdateMetadata { event_id, .. }
        | GraphEvent::TombstoneNode { event_id, .. }
        | GraphEvent::CreateEdge { event_id, .. }
        | GraphEvent::TombstoneEdge { event_id, .. } => event_id,
    }
}

#[cfg(test)]
mod tests;
