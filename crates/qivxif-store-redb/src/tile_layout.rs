use crate::{
    StoreError, StoreResult, codec::encode, operation_log::insert_operation,
    records::OperationReceipt, store::QivxifStore, tables,
};
use qivxif_auth::{AuthContext, Viewer, can_write};
use qivxif_core::{ActorId, NodeId, OperationId, ServerTime};
use qivxif_graph::{NodeKind, NodeRecord, TileLayout};
use qivxif_history::{
    OperationEnvelope, OperationKind, OperationPayload, OperationScope, hash_payload,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileLayoutSetInput {
    pub op_id: OperationId,
    pub actor_seq: u64,
    pub actor_id: ActorId,
    pub layout_node_id: NodeId,
    pub layout: TileLayout,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileLayoutSetResult {
    pub layout_node: NodeRecord,
    pub receipt: OperationReceipt,
}

impl QivxifStore {
    pub fn set_tile_layout(
        &self,
        auth: &AuthContext,
        input: TileLayoutSetInput,
    ) -> StoreResult<TileLayoutSetResult> {
        let op = tile_envelope(&input)?;
        self.accept_tile_layout_op(auth, op, input.layout)
    }

    pub(crate) fn accept_tile_layout_op(
        &self,
        auth: &AuthContext,
        op: OperationEnvelope,
        layout: TileLayout,
    ) -> StoreResult<TileLayoutSetResult> {
        if self.get_operation(&op.op_id)?.is_some() {
            return self.tile_layout_replay(&op);
        }
        let Some(layout_node_id) = op.target_node_ids.first().cloned() else {
            return Err(StoreError::InvalidOperation);
        };
        if !actor_matches(auth, &op) {
            return Err(StoreError::Forbidden);
        }
        let mut node = self
            .get_node(&layout_node_id)?
            .ok_or(StoreError::NodeMissing)?;
        if node.kind != NodeKind::TileLayout || !can_write(auth, &node) {
            return Err(StoreError::Forbidden);
        }
        let layout_json =
            serde_json::to_string(&layout).map_err(|_| StoreError::InvalidOperation)?;
        let tx = self.database.begin_write()?;
        let receipt = insert_operation(&tx, &op)?;
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

    fn tile_layout_replay(&self, op: &OperationEnvelope) -> StoreResult<TileLayoutSetResult> {
        let layout_node_id = op
            .target_node_ids
            .first()
            .ok_or(StoreError::InvalidOperation)?;
        let layout_node = self
            .get_node(layout_node_id)?
            .ok_or(StoreError::OperationConflict)?;
        let receipt = self
            .operation_receipt(&op.op_id)?
            .ok_or(StoreError::OperationConflict)?;
        Ok(TileLayoutSetResult {
            layout_node,
            receipt,
        })
    }
}

fn tile_envelope(input: &TileLayoutSetInput) -> StoreResult<OperationEnvelope> {
    let bytes = serde_json::to_vec(&input.layout).map_err(|_| StoreError::InvalidOperation)?;
    Ok(OperationEnvelope {
        op_id: input.op_id.clone(),
        actor_id: input.actor_id.clone(),
        actor_seq: input.actor_seq,
        parents: Vec::new(),
        scope: OperationScope::Tile,
        kind: OperationKind::TileLayoutSet,
        target_node_ids: vec![input.layout_node_id.clone()],
        payload: OperationPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: Some(ServerTime::now()),
        auth_context: Some(input.layout_node_id.to_string()),
    })
}

fn actor_matches(auth: &AuthContext, op: &OperationEnvelope) -> bool {
    matches!(
        &auth.viewer,
        Viewer::Session { actor_id, .. } if actor_id == &op.actor_id
    )
}
