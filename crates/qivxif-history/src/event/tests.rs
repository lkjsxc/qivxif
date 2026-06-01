use super::*;
use qivxif_core::{ActorId, EventId};

fn event(seq: u64, payload_hash: PayloadHash) -> EventEnvelope {
    let payload = EventPayload {
        bytes: b"{}".to_vec(),
    };
    EventEnvelope {
        event_id: EventId::generate(),
        actor_id: ActorId::generate(),
        actor_seq: seq,
        parents: Vec::new(),
        scope: EventScope::Graph,
        kind: EventKind::NodeCreate,
        target_node_ids: Vec::new(),
        target_edge_ids: Vec::new(),
        target_event_ids: Vec::new(),
        payload,
        payload_hash,
        created_at_client: None,
        received_at_server: None,
        auth_context: None,
    }
}

#[test]
fn rejects_invalid_actor_sequence() {
    let hash = hash_payload(b"{}");
    assert_eq!(
        validate_event_envelope(event(0, hash)).unwrap_err(),
        HistoryError::InvalidActorSeq
    );
}

#[test]
fn rejects_missing_payload_hash() {
    assert_eq!(
        validate_event_envelope(event(1, PayloadHash(String::new()))).unwrap_err(),
        HistoryError::MissingPayloadHash
    );
}

#[test]
fn rejects_payload_hash_mismatch() {
    let hash = hash_payload(b"different");
    assert_eq!(
        validate_event_envelope(event(1, hash)).unwrap_err(),
        HistoryError::PayloadHashMismatch
    );
}
