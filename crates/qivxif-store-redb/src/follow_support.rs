use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    tables,
};
use qivxif_core::{ActorId, EdgeId, MetadataMap, NodeId, OperationId, ServerTime, UserId};
use qivxif_graph::{EdgeKind, EdgeRecord};
use qivxif_history::{
    OperationEnvelope, OperationKind, OperationPayload, OperationScope, hash_payload,
};
use redb::ReadableTable;

pub(crate) fn active_follow(
    tx: &redb::WriteTransaction,
    from_node: &NodeId,
    to_node: &NodeId,
) -> StoreResult<Option<EdgeRecord>> {
    let by_from = tx.open_table(tables::EDGES_BY_FROM)?;
    let edges = tx.open_table(tables::EDGES)?;
    let prefix = format!("{}:", from_node.as_str());
    for item in by_from.iter()? {
        let (key, edge_id) = item?;
        if !key.value().starts_with(&prefix) {
            continue;
        }
        let edge_id: EdgeId = decode(edge_id.value())?;
        let Some(edge_bytes) = edges.get(edge_id.as_str())? else {
            continue;
        };
        let edge: EdgeRecord = decode(edge_bytes.value())?;
        if edge.kind == EdgeKind::Follows && &edge.to_node == to_node && edge.tombstone.is_none() {
            return Ok(Some(edge));
        }
    }
    Ok(None)
}

pub(crate) fn edge_by_id(
    tx: &redb::WriteTransaction,
    edge_id: &EdgeId,
) -> StoreResult<Option<EdgeRecord>> {
    let edges = tx.open_table(tables::EDGES)?;
    edges
        .get(edge_id.as_str())?
        .map(|bytes| decode(bytes.value()))
        .transpose()
}

pub(crate) fn follow_edge(
    edge_id: EdgeId,
    from_node: NodeId,
    to_node: NodeId,
    actor_id: ActorId,
) -> EdgeRecord {
    let mut metadata = MetadataMap::empty();
    metadata.insert("follow_state", "active");
    EdgeRecord {
        id: edge_id,
        from_node,
        to_node,
        kind: EdgeKind::Follows,
        created_by: actor_id,
        created_at: ServerTime::now(),
        metadata_map: metadata,
        tombstone: None,
    }
}

pub(crate) fn insert_edge_with_indexes(
    tx: &redb::WriteTransaction,
    edge: &EdgeRecord,
) -> StoreResult<()> {
    let mut edges = tx.open_table(tables::EDGES)?;
    if edges.get(edge.id.as_str())?.is_some() {
        return Err(StoreError::EdgeExists);
    }
    edges.insert(edge.id.as_str(), encode(edge)?.as_slice())?;
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
    Ok(())
}

pub(crate) fn update_edge(tx: &redb::WriteTransaction, edge: &EdgeRecord) -> StoreResult<()> {
    let mut edges = tx.open_table(tables::EDGES)?;
    edges.insert(edge.id.as_str(), encode(edge)?.as_slice())?;
    Ok(())
}

pub(crate) fn follow_operation(
    op_id: &OperationId,
    actor_seq: u64,
    actor_id: &ActorId,
    actor_user_id: &UserId,
    kind: OperationKind,
    edge: &EdgeRecord,
) -> StoreResult<OperationEnvelope> {
    let bytes = encode(edge)?;
    Ok(OperationEnvelope {
        op_id: op_id.clone(),
        actor_id: actor_id.clone(),
        actor_seq,
        parents: Vec::new(),
        scope: OperationScope::Social,
        kind,
        target_node_ids: vec![edge.from_node.clone(), edge.to_node.clone()],
        payload: OperationPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: Some(ServerTime::now()),
        auth_context: Some(actor_user_id.to_string()),
    })
}

fn edge_index(node_id: &NodeId, edge_id: &EdgeId) -> String {
    format!("{}:{}", node_id.as_str(), edge_id.as_str())
}
