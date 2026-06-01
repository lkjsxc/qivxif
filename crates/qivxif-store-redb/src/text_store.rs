use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    operation_log::insert_operation,
    records::OperationReceipt,
    store::QivxifStore,
    tables,
};
use qivxif_auth::{AuthContext, can_read, can_write};
use qivxif_core::{ActorId, NodeId, ServerTime};
use qivxif_graph::NodeKind;
use qivxif_history::{
    OperationEnvelope, OperationKind, OperationPayload, OperationScope, hash_payload,
    text::{TextDocState, TextEdit, TextOperation, apply_text_op},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextApplyInput {
    pub actor_id: ActorId,
    pub actor_seq: u64,
    pub node_id: NodeId,
    pub operation: TextOperation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextApplyResult {
    pub state: TextDocState,
    pub receipt: OperationReceipt,
}

impl QivxifStore {
    pub fn get_text_state(
        &self,
        auth: &AuthContext,
        node_id: &NodeId,
    ) -> StoreResult<Option<TextDocState>> {
        let Some(node) = self.get_node(node_id)? else {
            return Ok(None);
        };
        if !can_read(auth, &node) {
            return Err(StoreError::Forbidden);
        }
        let tx = self.database.begin_read()?;
        let docs = tx.open_table(tables::TEXT_DOCS)?;
        docs.get(node_id.as_str())?
            .map(|bytes| decode(bytes.value()))
            .transpose()
            .map(|state| Some(state.unwrap_or_default()))
    }

    pub fn apply_text_operation(
        &self,
        auth: &AuthContext,
        input: TextApplyInput,
    ) -> StoreResult<TextApplyResult> {
        if self.get_operation(&input.operation.op_id)?.is_some() {
            let state = self
                .get_text_state(auth, &input.node_id)?
                .ok_or(StoreError::NodeMissing)?;
            let receipt = self
                .operation_receipt(&input.operation.op_id)?
                .ok_or(StoreError::OperationConflict)?;
            return Ok(TextApplyResult { state, receipt });
        }
        let mut node = self
            .get_node(&input.node_id)?
            .ok_or(StoreError::NodeMissing)?;
        if node.kind != NodeKind::Text || !can_write(auth, &node) {
            return Err(StoreError::Forbidden);
        }
        let current = self
            .get_text_state(auth, &input.node_id)?
            .unwrap_or_default();
        let next = apply_text_op(current, input.operation.clone())
            .map_err(|_| StoreError::InvalidOperation)?;
        let op = text_envelope(&input)?;
        let tx = self.database.begin_write()?;
        let receipt = insert_operation(&tx, &op)?;
        {
            node.current_text_ref = Some(input.operation.doc_id.clone());
            node.updated_at = ServerTime::now();
            let mut nodes = tx.open_table(tables::NODES)?;
            nodes.insert(node.id.as_str(), encode(&node)?.as_slice())?;
            let mut docs = tx.open_table(tables::TEXT_DOCS)?;
            docs.insert(input.node_id.as_str(), encode(&next)?.as_slice())?;
        }
        tx.commit()?;
        Ok(TextApplyResult {
            state: next,
            receipt,
        })
    }
}

fn text_envelope(input: &TextApplyInput) -> StoreResult<OperationEnvelope> {
    let bytes = encode(&input.operation)?;
    Ok(OperationEnvelope {
        op_id: input.operation.op_id.clone(),
        actor_id: input.actor_id.clone(),
        actor_seq: input.actor_seq,
        parents: Vec::new(),
        scope: OperationScope::Text,
        kind: text_kind(&input.operation.edit),
        target_node_ids: vec![input.node_id.clone()],
        payload: OperationPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: Some(ServerTime::now()),
        auth_context: Some(input.node_id.to_string()),
    })
}

fn text_kind(edit: &TextEdit) -> OperationKind {
    match edit {
        TextEdit::Insert(_) => OperationKind::TextInsert,
        TextEdit::Delete(_) => OperationKind::TextDelete,
        TextEdit::Restore(_) => OperationKind::TextRestore,
    }
}
