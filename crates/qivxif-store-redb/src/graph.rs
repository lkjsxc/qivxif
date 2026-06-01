use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    operation_log::{insert_operation, receipt},
    records::OperationReceipt,
    store::QivxifStore,
    tables,
};
use qivxif_auth::{AuthContext, can_read};
use qivxif_core::{
    ActorId, EdgeId, MetadataMap, NodeId, OperationId, ServerTime, UserId, Visibility,
};
use qivxif_graph::{EdgeKind, EdgeRecord, NodeKind, NodeRecord, project_node};
use qivxif_history::{
    OperationEnvelope, OperationKind, OperationPayload, OperationScope, hash_payload,
};
use redb::ReadableTable;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeCreateInput {
    pub op_id: OperationId,
    pub actor_seq: u64,
    pub node_id: NodeId,
    pub owner_user_id: UserId,
    pub actor_id: ActorId,
    pub kind: NodeKind,
    pub visibility: Visibility,
    pub metadata_map: MetadataMap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeCreateResult {
    pub node: NodeRecord,
    pub receipt: OperationReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeCreateInput {
    pub op_id: OperationId,
    pub actor_seq: u64,
    pub edge_id: EdgeId,
    pub from_node: NodeId,
    pub to_node: NodeId,
    pub actor_id: ActorId,
    pub kind: EdgeKind,
    pub metadata_map: MetadataMap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeCreateResult {
    pub edge: EdgeRecord,
    pub receipt: OperationReceipt,
}

impl QivxifStore {
    pub fn create_node(&self, input: NodeCreateInput) -> StoreResult<NodeCreateResult> {
        if self.get_operation(&input.op_id)?.is_some() {
            let node = self
                .get_node(&input.node_id)?
                .ok_or(StoreError::OperationConflict)?;
            return Ok(NodeCreateResult {
                node,
                receipt: receipt(&input.op_id),
            });
        }
        let now = ServerTime::now();
        let node = NodeRecord {
            id: input.node_id.clone(),
            kind: input.kind,
            owner_user_id: input.owner_user_id,
            created_by: input.actor_id.clone(),
            created_at: now,
            updated_at: now,
            visibility: input.visibility,
            acl_ref: None,
            current_commit_group: None,
            current_text_ref: None,
            metadata_map: input.metadata_map,
            tombstone: None,
        };
        let op = node_operation(&input.op_id, input.actor_seq, &input.actor_id, &node)?;
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
        Ok(NodeCreateResult { node, receipt })
    }

    pub fn get_node_projection(
        &self,
        auth: &AuthContext,
        node_id: &NodeId,
    ) -> StoreResult<Option<qivxif_graph::NodeProjection>> {
        let Some(node) = self.get_node(node_id)? else {
            return Ok(None);
        };
        if !can_read(auth, &node) {
            return Err(StoreError::Forbidden);
        }
        let edges = self.list_edges_for_node(node_id)?;
        Ok(project_node(&node, edges, false))
    }
}

pub(crate) fn node_operation(
    op_id: &OperationId,
    actor_seq: u64,
    actor_id: &ActorId,
    node: &NodeRecord,
) -> StoreResult<OperationEnvelope> {
    let bytes = encode(node)?;
    Ok(OperationEnvelope {
        op_id: op_id.clone(),
        actor_id: actor_id.clone(),
        actor_seq,
        parents: Vec::new(),
        scope: OperationScope::Graph,
        kind: OperationKind::NodeCreate,
        target_node_ids: vec![node.id.clone()],
        payload: OperationPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: Some(ServerTime::now()),
        auth_context: Some(node.owner_user_id.to_string()),
    })
}

pub(crate) fn edge_operation(
    op_id: &OperationId,
    actor_seq: u64,
    actor_id: &ActorId,
    edge: &EdgeRecord,
) -> StoreResult<OperationEnvelope> {
    let bytes = encode(edge)?;
    Ok(OperationEnvelope {
        op_id: op_id.clone(),
        actor_id: actor_id.clone(),
        actor_seq,
        parents: Vec::new(),
        scope: OperationScope::Graph,
        kind: OperationKind::EdgeCreate,
        target_node_ids: vec![edge.from_node.clone(), edge.to_node.clone()],
        payload: OperationPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: Some(ServerTime::now()),
        auth_context: Some(edge.created_by.to_string()),
    })
}
