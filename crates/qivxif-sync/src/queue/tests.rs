use super::*;
use qivxif_api::{EventAcceptance, EventRejection};
use qivxif_core::{ActorId, NodeId};
use qivxif_history::{EventEnvelope, EventPayload, EventScope, hash_payload};

#[test]
fn queues_events_once_in_actor_order() {
    let later = event(2, EventKind::TextRestore);
    let earlier = event(1, EventKind::NodeCreate);
    let queue = queue_event(PendingEventQueue::default(), later.clone());
    let queue = queue_event(queue, earlier.clone());
    let queue = queue_event(queue, earlier.clone());
    assert_eq!(queue.pending.len(), 2);
    assert_eq!(queue.pending[0].event_id, earlier.event_id);
    assert_eq!(queue.pending[1].event_id, later.event_id);
}

#[test]
fn network_failure_returns_in_flight_events_to_dirty() {
    let event = event(1, EventKind::NodeCreate);
    let queue = queue_event(PendingEventQueue::default(), event.clone());
    let queue = mark_upload_started(queue, std::slice::from_ref(&event.event_id));
    assert_eq!(
        queue.pending[0].status,
        PendingEventStatus::PendingValidation
    );
    let queue = record_network_failure(queue);
    assert_eq!(queue.pending[0].status, PendingEventStatus::Dirty);
}

#[test]
fn accepted_push_removes_pending_event_and_updates_cursor() {
    let event = event(1, EventKind::NodeCreate);
    let cursor = CursorId::generate();
    let queue = queue_event(PendingEventQueue::default(), event.clone());
    let queue = apply_push_response(
        queue,
        PushResponse {
            accepted: vec![EventAcceptance {
                event_id: event.event_id,
                server_cursor: cursor.clone(),
            }],
            rejected: Vec::new(),
            server_cursor: None,
        },
    );
    assert!(queue.pending.is_empty());
    assert_eq!(queue.client_uploaded_through, Some(cursor));
}

#[test]
fn rejected_push_keeps_event_visible() {
    let event = event(1, EventKind::NodeCreate);
    let queue = queue_event(PendingEventQueue::default(), event.clone());
    let queue = apply_push_response(
        queue,
        PushResponse {
            accepted: Vec::new(),
            rejected: vec![EventRejection {
                event_id: event.event_id.clone(),
                code: "auth.forbidden".to_owned(),
                message: "forbidden".to_owned(),
            }],
            server_cursor: None,
        },
    );
    assert_eq!(queue.counts().queued, 1);
    assert_eq!(queue.counts().rejected, 1);
    assert_eq!(queue.pending[0].status, PendingEventStatus::Rejected);
    assert_eq!(queue.last_rejection.unwrap().event_id, event.event_id);
}

#[test]
fn pull_progress_updates_only_applied_cursor() {
    let cursor = CursorId::generate();
    let queue = record_pull_applied(
        PendingEventQueue::default(),
        &PullResponse {
            events: Vec::new(),
            server_cursor: Some(cursor.clone()),
            has_more: false,
        },
    );
    assert_eq!(queue.client_applied_through, Some(cursor));
    assert_eq!(queue.client_uploaded_through, None);
}

fn event(actor_seq: u64, kind: EventKind) -> EventEnvelope {
    let payload = EventPayload {
        bytes: b"{}".to_vec(),
    };
    EventEnvelope {
        event_id: EventId::generate(),
        actor_id: ActorId::generate(),
        actor_seq,
        parents: Vec::new(),
        scope: EventScope::Graph,
        kind,
        target_node_ids: vec![NodeId::generate()],
        target_edge_ids: Vec::new(),
        target_event_ids: Vec::new(),
        payload_hash: hash_payload(&payload.bytes),
        payload,
        created_at_client: None,
        received_at_server: None,
        auth_context: None,
    }
}
