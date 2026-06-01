use super::*;
use qivxif_api::{OperationAcceptance, OperationRejection};
use qivxif_core::{ActorId, NodeId};
use qivxif_history::{OperationEnvelope, OperationPayload, OperationScope, hash_payload};

#[test]
fn queues_operations_once_in_actor_order() {
    let later = operation(2, OperationKind::TextRestore);
    let earlier = operation(1, OperationKind::NodeCreate);
    let queue = queue_operation(PendingOpQueue::default(), later.clone());
    let queue = queue_operation(queue, earlier.clone());
    let queue = queue_operation(queue, earlier.clone());
    assert_eq!(queue.pending.len(), 2);
    assert_eq!(queue.pending[0].op_id, earlier.op_id);
    assert_eq!(queue.pending[1].op_id, later.op_id);
}

#[test]
fn network_failure_returns_in_flight_operations_to_dirty() {
    let op = operation(1, OperationKind::NodeCreate);
    let queue = queue_operation(PendingOpQueue::default(), op.clone());
    let queue = mark_upload_started(queue, std::slice::from_ref(&op.op_id));
    assert_eq!(queue.pending[0].status, PendingOpStatus::PendingValidation);
    let queue = record_network_failure(queue);
    assert_eq!(queue.pending[0].status, PendingOpStatus::Dirty);
}

#[test]
fn accepted_push_removes_pending_operation_and_updates_cursor() {
    let op = operation(1, OperationKind::NodeCreate);
    let cursor = CursorId::generate();
    let queue = queue_operation(PendingOpQueue::default(), op.clone());
    let queue = apply_push_response(
        queue,
        PushResponse {
            accepted: vec![OperationAcceptance {
                op_id: op.op_id,
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
fn rejected_push_keeps_operation_visible() {
    let op = operation(1, OperationKind::NodeCreate);
    let queue = queue_operation(PendingOpQueue::default(), op.clone());
    let queue = apply_push_response(
        queue,
        PushResponse {
            accepted: Vec::new(),
            rejected: vec![OperationRejection {
                op_id: op.op_id.clone(),
                code: "auth.forbidden".to_owned(),
                message: "forbidden".to_owned(),
            }],
            server_cursor: None,
        },
    );
    assert_eq!(queue.counts().queued, 1);
    assert_eq!(queue.counts().rejected, 1);
    assert_eq!(queue.pending[0].status, PendingOpStatus::Rejected);
    assert_eq!(queue.last_rejection.unwrap().op_id, op.op_id);
}

#[test]
fn pull_progress_updates_only_applied_cursor() {
    let cursor = CursorId::generate();
    let queue = record_pull_applied(
        PendingOpQueue::default(),
        &PullResponse {
            operations: Vec::new(),
            server_cursor: Some(cursor.clone()),
            has_more: false,
        },
    );
    assert_eq!(queue.client_applied_through, Some(cursor));
    assert_eq!(queue.client_uploaded_through, None);
}

fn operation(actor_seq: u64, kind: OperationKind) -> OperationEnvelope {
    let payload = OperationPayload {
        bytes: b"{}".to_vec(),
    };
    OperationEnvelope {
        op_id: OperationId::generate(),
        actor_id: ActorId::generate(),
        actor_seq,
        parents: Vec::new(),
        scope: OperationScope::Graph,
        kind,
        target_node_ids: vec![NodeId::generate()],
        payload_hash: hash_payload(&payload.bytes),
        payload,
        created_at_client: None,
        received_at_server: None,
        auth_context: None,
    }
}
