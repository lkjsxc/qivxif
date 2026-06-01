use crate::{EdgeKind, EdgeRecord, GraphError, GraphResult, NodeRecord};
use qivxif_core::{EdgeId, NodeId};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TreeProjection {
    pub root: TreeNode,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TreeNode {
    pub node_id: NodeId,
    pub via_edge_id: Option<EdgeId>,
    pub children: Vec<TreeNode>,
}

#[derive(Clone, Debug)]
struct ChildLink {
    edge_id: EdgeId,
    child_id: NodeId,
    order: EdgeOrder,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct EdgeOrder {
    tier: u8,
    value: String,
    edge_id: EdgeId,
}

pub fn project_tree(
    root_id: &NodeId,
    nodes: impl IntoIterator<Item = NodeRecord>,
    edges: impl IntoIterator<Item = EdgeRecord>,
) -> GraphResult<TreeProjection> {
    project_tree_with_kinds(root_id, nodes, edges, &default_tree_kinds())
}

pub fn project_tree_with_kinds(
    root_id: &NodeId,
    nodes: impl IntoIterator<Item = NodeRecord>,
    edges: impl IntoIterator<Item = EdgeRecord>,
    relation_kinds: &[EdgeKind],
) -> GraphResult<TreeProjection> {
    let node_ids = active_node_ids(nodes);
    if !node_ids.contains(root_id) {
        return Err(GraphError::NodeMissing);
    }
    let children = child_links(edges, &node_ids, relation_kinds)?;
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    Ok(TreeProjection {
        root: build_node(root_id, None, &children, &mut visiting, &mut visited)?,
    })
}

fn active_node_ids(nodes: impl IntoIterator<Item = NodeRecord>) -> BTreeSet<NodeId> {
    nodes
        .into_iter()
        .filter(|node| node.tombstone.is_none())
        .map(|node| node.id)
        .collect()
}

fn child_links(
    edges: impl IntoIterator<Item = EdgeRecord>,
    node_ids: &BTreeSet<NodeId>,
    relation_kinds: &[EdgeKind],
) -> GraphResult<BTreeMap<NodeId, Vec<ChildLink>>> {
    let mut parent_by_child: BTreeMap<NodeId, EdgeId> = BTreeMap::new();
    let mut children: BTreeMap<NodeId, Vec<ChildLink>> = BTreeMap::new();
    for edge in edges {
        if !relation_kinds.contains(&edge.kind) {
            continue;
        }
        if edge.tombstone.is_some() {
            continue;
        }
        if !node_ids.contains(&edge.from_node) || !node_ids.contains(&edge.to_node) {
            return Err(GraphError::NodeMissing);
        }
        if parent_by_child
            .insert(edge.to_node.clone(), edge.id.clone())
            .is_some()
        {
            return Err(GraphError::DuplicateActiveParent);
        }
        children
            .entry(edge.from_node.clone())
            .or_default()
            .push(ChildLink {
                edge_id: edge.id.clone(),
                child_id: edge.to_node.clone(),
                order: edge_order(&edge),
            });
    }
    for links in children.values_mut() {
        links.sort_by(|left, right| left.order.cmp(&right.order));
    }
    Ok(children)
}

fn build_node(
    node_id: &NodeId,
    via_edge_id: Option<EdgeId>,
    children: &BTreeMap<NodeId, Vec<ChildLink>>,
    visiting: &mut BTreeSet<NodeId>,
    visited: &mut BTreeSet<NodeId>,
) -> GraphResult<TreeNode> {
    if visiting.contains(node_id) {
        return Err(GraphError::TreeCycle);
    }
    if visited.contains(node_id) {
        return Ok(TreeNode {
            node_id: node_id.clone(),
            via_edge_id,
            children: Vec::new(),
        });
    }
    visiting.insert(node_id.clone());
    let mut child_nodes = Vec::new();
    for link in children.get(node_id).into_iter().flatten() {
        child_nodes.push(build_node(
            &link.child_id,
            Some(link.edge_id.clone()),
            children,
            visiting,
            visited,
        )?);
    }
    visiting.remove(node_id);
    visited.insert(node_id.clone());
    Ok(TreeNode {
        node_id: node_id.clone(),
        via_edge_id,
        children: child_nodes,
    })
}

fn edge_order(edge: &EdgeRecord) -> EdgeOrder {
    if let Some(position_key) = edge.metadata_map.get("position_key") {
        return order(0, position_key, edge);
    }
    if let Some(ordinal) = edge.metadata_map.get("ordinal") {
        return order(1, ordinal, edge);
    }
    order(2, "", edge)
}

fn order(tier: u8, value: &str, edge: &EdgeRecord) -> EdgeOrder {
    EdgeOrder {
        tier,
        value: value.to_owned(),
        edge_id: edge.id.clone(),
    }
}

fn default_tree_kinds() -> [EdgeKind; 3] {
    [
        EdgeKind::Contains,
        EdgeKind::ParentOf,
        EdgeKind::OrderedChild,
    ]
}

#[cfg(test)]
mod tests;
