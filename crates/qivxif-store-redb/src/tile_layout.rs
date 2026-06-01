use crate::{
    StoreError, StoreResult, codec::encode, event_log::insert_event, records::EventReceipt,
    store::QivxifStore, tables,
};
use qivxif_auth::{AuthContext, Viewer, can_write};
use qivxif_core::{ActorId, EventId, NodeId, ServerTime};
use qivxif_graph::{NodeKind, NodeRecord, TileLayout};
use qivxif_history::{EventEnvelope, EventKind, EventPayload, EventScope, hash_payload};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileLayoutSetInput {
    pub event_id: EventId,
    pub actor_seq: u64,
    pub actor_id: ActorId,
    pub layout_node_id: NodeId,
    pub layout: TileLayout,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileLayoutSetResult {
    pub layout_node: NodeRecord,
    pub receipt: EventReceipt,
}

impl QivxifStore {
    pub fn set_tile_layout(
        &self,
        auth: &AuthContext,
        input: TileLayoutSetInput,
    ) -> StoreResult<TileLayoutSetResult> {
        let event = tile_envelope(&input)?;
        self.accept_tile_layout_event(auth, event, input.layout)
    }

    pub(crate) fn accept_tile_layout_event(
        &self,
        auth: &AuthContext,
        event: EventEnvelope,
        layout: TileLayout,
    ) -> StoreResult<TileLayoutSetResult> {
        if self.get_event(&event.event_id)?.is_some() {
            return self.tile_layout_replay(&event);
        }
        let Some(layout_node_id) = event.target_node_ids.first().cloned() else {
            return Err(StoreError::InvalidEvent);
        };
        if !actor_matches(auth, &event) {
            return Err(StoreError::Forbidden);
        }
        let mut node = self
            .get_node(&layout_node_id)?
            .ok_or(StoreError::NodeMissing)?;
        if node.kind != NodeKind::TileLayout || !can_write(auth, &node) {
            return Err(StoreError::Forbidden);
        }
        let layout_json = serde_json::to_string(&layout).map_err(|_| StoreError::InvalidEvent)?;
        let tx = self.database.begin_write()?;
        let receipt = insert_event(&tx, &event)?;
        {
            node.updated_at = ServerTime::now();
            node.metadata_map.insert("layout_json", layout_json);
            let mut nodes = tx.open_table(tables::NODES)?;
            nodes.insert(node.id.as_str(), encode(&node)?.as_slice())?;
        }
        tx.commit()?;
        Ok(TileLayoutSetResult {
            layout_node: node,
            receipt,
        })
    }

    fn tile_layout_replay(&self, event: &EventEnvelope) -> StoreResult<TileLayoutSetResult> {
        let layout_node_id = event
            .target_node_ids
            .first()
            .ok_or(StoreError::InvalidEvent)?;
        let layout_node = self
            .get_node(layout_node_id)?
            .ok_or(StoreError::EventConflict)?;
        let receipt = self
            .event_receipt(&event.event_id)?
            .ok_or(StoreError::EventConflict)?;
        Ok(TileLayoutSetResult {
            layout_node,
            receipt,
        })
    }
}

fn tile_envelope(input: &TileLayoutSetInput) -> StoreResult<EventEnvelope> {
    let bytes = serde_json::to_vec(&input.layout).map_err(|_| StoreError::InvalidEvent)?;
    Ok(EventEnvelope {
        event_id: input.event_id.clone(),
        actor_id: input.actor_id.clone(),
        actor_seq: input.actor_seq,
        parents: Vec::new(),
        scope: EventScope::Tile,
        kind: EventKind::TileLayoutSet,
        target_node_ids: vec![input.layout_node_id.clone()],
        target_edge_ids: Vec::new(),
        target_event_ids: Vec::new(),
        payload: EventPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: Some(ServerTime::now()),
        auth_context: Some(input.layout_node_id.to_string()),
    })
}

fn actor_matches(auth: &AuthContext, event: &EventEnvelope) -> bool {
    matches!(
        &auth.viewer,
        Viewer::Session { actor_id, .. } if actor_id == &event.actor_id
    )
}
