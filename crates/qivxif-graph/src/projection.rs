use crate::{EdgeRecord, NodeRecord};
use qivxif_core::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodeProjection {
    pub node: NodeRecord,
    pub outgoing: Vec<EdgeRecord>,
    pub incoming: Vec<EdgeRecord>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GraphProjection {
    pub nodes: Vec<NodeProjection>,
}

pub fn project_node(
    record: &NodeRecord,
    edges: impl IntoIterator<Item = EdgeRecord>,
    include_tombstoned: bool,
) -> Option<NodeProjection> {
    if record.tombstone.is_some() && !include_tombstoned {
        return None;
    }
    let mut outgoing = Vec::new();
    let mut incoming = Vec::new();
    let id: &NodeId = &record.id;
    for edge in edges {
        if edge.tombstone.is_some() && !include_tombstoned {
            continue;
        }
        if &edge.from_node == id {
            outgoing.push(edge);
        } else if &edge.to_node == id {
            incoming.push(edge);
        }
    }
    Some(NodeProjection {
        node: record.clone(),
        outgoing,
        incoming,
    })
}
