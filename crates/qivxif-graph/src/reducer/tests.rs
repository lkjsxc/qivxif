use super::*;
use crate::{EdgeKind, NodeKind, project_node};
use qivxif_core::{ActorId, MetadataMap, ServerTime, UserId, Visibility};

#[test]
fn creates_node_and_ignores_duplicate_event() {
    let record = node_record(NodeKind::Text);
    let event = GraphEvent::CreateNode {
        event_id: EventId::generate(),
        record: record.clone(),
    };
    let state = apply_graph_event(GraphState::default(), event.clone()).unwrap();
    assert_eq!(state.nodes.get(&record.id), Some(&record));
    assert_eq!(apply_graph_event(state.clone(), event).unwrap(), state);
}

#[test]
fn creates_edge_as_independent_record() {
    let left = node_record(NodeKind::Text);
    let right = node_record(NodeKind::Topic);
    let edge = edge_record(&left, &right);
    let state = apply_graph_event(GraphState::default(), create_node(&left)).unwrap();
    let state = apply_graph_event(state, create_node(&right)).unwrap();
    let state = apply_graph_event(
        state,
        GraphEvent::CreateEdge {
            event_id: EventId::generate(),
            record: edge.clone(),
        },
    )
    .unwrap();
    assert_eq!(state.edges.get(&edge.id), Some(&edge));
    let projection = project_node(&left, state.edges.values().cloned(), false).unwrap();
    assert_eq!(projection.outgoing, vec![edge]);
}

#[test]
fn rejects_edge_when_endpoint_is_missing() {
    let left = node_record(NodeKind::Text);
    let right = node_record(NodeKind::Topic);
    let edge = edge_record(&left, &right);
    let state = apply_graph_event(GraphState::default(), create_node(&left)).unwrap();
    let result = apply_graph_event(
        state,
        GraphEvent::CreateEdge {
            event_id: EventId::generate(),
            record: edge,
        },
    );
    assert_eq!(result.unwrap_err(), GraphError::NodeMissing);
}

#[test]
fn tombstoned_node_is_hidden_from_normal_projection() {
    let node = node_record(NodeKind::Text);
    let tombstone = Tombstone {
        by: node.created_by.clone(),
        reason: "test".to_owned(),
    };
    let state = apply_graph_event(GraphState::default(), create_node(&node)).unwrap();
    let state = apply_graph_event(
        state,
        GraphEvent::TombstoneNode {
            event_id: EventId::generate(),
            node_id: node.id.clone(),
            tombstone,
        },
    )
    .unwrap();
    let record = state.nodes.get(&node.id).unwrap();
    assert!(project_node(record, Vec::<EdgeRecord>::new(), false).is_none());
    assert!(project_node(record, Vec::<EdgeRecord>::new(), true).is_some());
}

fn create_node(record: &NodeRecord) -> GraphEvent {
    GraphEvent::CreateNode {
        event_id: EventId::generate(),
        record: record.clone(),
    }
}

fn node_record(kind: NodeKind) -> NodeRecord {
    let now = ServerTime::now();
    NodeRecord {
        id: NodeId::generate(),
        kind,
        owner_user_id: UserId::generate(),
        created_by: ActorId::generate(),
        created_at: now,
        updated_at: now,
        visibility: Visibility::Private,
        acl_ref: None,
        current_commit_group: None,
        current_text_ref: None,
        metadata_map: MetadataMap::empty(),
        tombstone: None,
    }
}

fn edge_record(from: &NodeRecord, to: &NodeRecord) -> EdgeRecord {
    EdgeRecord {
        id: EdgeId::generate(),
        from_node: from.id.clone(),
        to_node: to.id.clone(),
        kind: EdgeKind::LinksTo,
        created_by: from.created_by.clone(),
        created_at: ServerTime::now(),
        metadata_map: MetadataMap::empty(),
        tombstone: None,
    }
}
