use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    event_log::insert_event,
    event_write::event_matches_for_replay,
    records::EventReceipt,
    store::QivxifStore,
    tables,
};
use qivxif_auth::{AuthContext, can_read, can_write};
use qivxif_core::{ActorId, NodeId, ServerTime};
use qivxif_graph::NodeKind;
use qivxif_history::{
    EventEnvelope, EventKind, EventPayload, EventScope, hash_payload,
    text::{TextDocState, TextEdit, TextEvent, apply_text_event},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextApplyInput {
    pub actor_id: ActorId,
    pub actor_seq: u64,
    pub node_id: NodeId,
    pub event: TextEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextApplyResult {
    pub state: TextDocState,
    pub receipt: EventReceipt,
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

    pub fn apply_text_event(
        &self,
        auth: &AuthContext,
        input: TextApplyInput,
    ) -> StoreResult<TextApplyResult> {
        let event = text_envelope(&input)?;
        if let Some(existing) = self.get_event(&input.event.event_id)? {
            if !event_matches_for_replay(&existing, &event) {
                return Err(StoreError::EventConflict);
            }
            let state = self
                .get_text_state(auth, &input.node_id)?
                .ok_or(StoreError::NodeMissing)?;
            let receipt = self
                .event_receipt(&input.event.event_id)?
                .ok_or(StoreError::EventConflict)?;
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
        let next =
            apply_text_event(current, input.event.clone()).map_err(|_| StoreError::InvalidEvent)?;
        let tx = self.database.begin_write()?;
        let receipt = insert_event(&tx, &event)?;
        {
            node.current_text_ref = Some(input.event.doc_id.clone());
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

fn text_envelope(input: &TextApplyInput) -> StoreResult<EventEnvelope> {
    let bytes = serde_json::to_vec(&input.event).map_err(|_| StoreError::InvalidEvent)?;
    Ok(EventEnvelope {
        event_id: input.event.event_id.clone(),
        actor_id: input.actor_id.clone(),
        actor_seq: input.actor_seq,
        parents: Vec::new(),
        scope: EventScope::Text,
        kind: text_kind(&input.event.edit),
        target_node_ids: vec![input.node_id.clone()],
        target_edge_ids: Vec::new(),
        target_event_ids: Vec::new(),
        payload: EventPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: Some(ServerTime::now()),
        auth_context: Some(input.node_id.to_string()),
    })
}

fn text_kind(edit: &TextEdit) -> EventKind {
    match edit {
        TextEdit::Insert(_) => EventKind::TextInsert,
        TextEdit::Delete(_) => EventKind::TextDelete,
        TextEdit::Restore(_) => EventKind::TextRestore,
    }
}
