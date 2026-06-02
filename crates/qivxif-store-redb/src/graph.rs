use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    event_log::insert_event,
    records::EventReceipt,
    store::QivxifStore,
    tables,
};
use qivxif_auth::{AuthContext, can_read};
use qivxif_core::{ActorId, EdgeId, EventId, MetadataMap, NodeId, ServerTime, UserId, Visibility};
use qivxif_graph::{EdgeKind, EdgeRecord, NodeKind, NodeRecord, project_node};
use qivxif_history::{EventEnvelope, EventKind, EventPayload, EventScope, hash_payload};
use redb::ReadableTable;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeCreateInput {
    pub event_id: EventId,
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
    pub receipt: EventReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeCreateInput {
    pub event_id: EventId,
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
    pub receipt: EventReceipt,
}

impl QivxifStore {
    pub fn create_node(&self, input: NodeCreateInput) -> StoreResult<NodeCreateResult> {
        if let Some(existing) = self.get_event(&input.event_id)? {
            return self.node_create_replay(&existing, &input);
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
        let event = node_event(&input.event_id, input.actor_seq, &input.actor_id, &node)?;
        let tx = self.database.begin_write()?;
        let receipt = insert_event(&tx, &event)?;
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

    fn node_create_replay(
        &self,
        event: &EventEnvelope,
        input: &NodeCreateInput,
    ) -> StoreResult<NodeCreateResult> {
        if event.kind != EventKind::NodeCreate
            || event.actor_id != input.actor_id
            || event.actor_seq != input.actor_seq
            || event.target_node_ids.as_slice() != std::slice::from_ref(&input.node_id)
        {
            return Err(StoreError::EventConflict);
        }
        let created: NodeRecord = decode(&event.payload.bytes)?;
        if created.id != input.node_id
            || created.kind != input.kind
            || created.owner_user_id != input.owner_user_id
            || created.created_by != input.actor_id
            || created.visibility != input.visibility
            || created.metadata_map != input.metadata_map
        {
            return Err(StoreError::EventConflict);
        }
        let node = self
            .get_node(&input.node_id)?
            .ok_or(StoreError::EventConflict)?;
        let receipt = self
            .event_receipt(&input.event_id)?
            .ok_or(StoreError::EventConflict)?;
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

pub(crate) fn node_event(
    event_id: &EventId,
    actor_seq: u64,
    actor_id: &ActorId,
    node: &NodeRecord,
) -> StoreResult<EventEnvelope> {
    let bytes = encode(node)?;
    Ok(EventEnvelope {
        event_id: event_id.clone(),
        actor_id: actor_id.clone(),
        actor_seq,
        parents: Vec::new(),
        scope: EventScope::Graph,
        kind: EventKind::NodeCreate,
        target_node_ids: vec![node.id.clone()],
        target_edge_ids: Vec::new(),
        target_event_ids: Vec::new(),
        payload: EventPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: Some(ServerTime::now()),
        auth_context: Some(node.owner_user_id.to_string()),
    })
}

pub(crate) fn edge_event(
    event_id: &EventId,
    actor_seq: u64,
    actor_id: &ActorId,
    edge: &EdgeRecord,
) -> StoreResult<EventEnvelope> {
    let bytes = encode(edge)?;
    Ok(EventEnvelope {
        event_id: event_id.clone(),
        actor_id: actor_id.clone(),
        actor_seq,
        parents: Vec::new(),
        scope: EventScope::Graph,
        kind: EventKind::EdgeCreate,
        target_node_ids: vec![edge.from_node.clone(), edge.to_node.clone()],
        target_edge_ids: vec![edge.id.clone()],
        target_event_ids: Vec::new(),
        payload: EventPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: Some(ServerTime::now()),
        auth_context: Some(edge.created_by.to_string()),
    })
}
