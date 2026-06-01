use qivxif_api::{OperationRejection, PullResponse, PushResponse};
use qivxif_core::{CursorId, OperationId};
use qivxif_history::{OperationEnvelope, OperationKind};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PendingOpQueue {
    pub pending: Vec<PendingOperation>,
    pub client_uploaded_through: Option<CursorId>,
    pub client_applied_through: Option<CursorId>,
    pub last_rejection: Option<PendingRejection>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PendingOperation {
    pub op_id: OperationId,
    pub actor_seq: u64,
    pub kind: OperationKind,
    pub status: PendingOpStatus,
    pub rejection: Option<PendingRejection>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PendingOpStatus {
    Dirty,
    PendingValidation,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PendingRejection {
    pub op_id: OperationId,
    pub code: String,
    pub message: String,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct QueueCounts {
    pub queued: usize,
    pub dirty: usize,
    pub rejected: usize,
}

pub fn queue_operation(mut queue: PendingOpQueue, op: OperationEnvelope) -> PendingOpQueue {
    if queue.pending.iter().any(|entry| entry.op_id == op.op_id) {
        return queue;
    }
    queue.pending.push(PendingOperation {
        op_id: op.op_id,
        actor_seq: op.actor_seq,
        kind: op.kind,
        status: PendingOpStatus::Dirty,
        rejection: None,
    });
    queue
        .pending
        .sort_by(|left, right| (left.actor_seq, &left.op_id).cmp(&(right.actor_seq, &right.op_id)));
    queue
}

pub fn mark_upload_started(mut queue: PendingOpQueue, op_ids: &[OperationId]) -> PendingOpQueue {
    for entry in &mut queue.pending {
        if op_ids.contains(&entry.op_id) && entry.status == PendingOpStatus::Dirty {
            entry.status = PendingOpStatus::PendingValidation;
        }
    }
    queue
}

pub fn record_network_failure(mut queue: PendingOpQueue) -> PendingOpQueue {
    for entry in &mut queue.pending {
        if entry.status == PendingOpStatus::PendingValidation {
            entry.status = PendingOpStatus::Dirty;
        }
    }
    queue
}

pub fn apply_push_response(mut queue: PendingOpQueue, response: PushResponse) -> PendingOpQueue {
    if let Some(cursor) = response.server_cursor {
        queue.client_uploaded_through = Some(cursor);
    } else if let Some(last) = response.accepted.last() {
        queue.client_uploaded_through = Some(last.server_cursor.clone());
    }
    for accepted in response.accepted {
        queue.pending.retain(|entry| entry.op_id != accepted.op_id);
    }
    for rejected in response.rejected {
        let rejection = to_pending_rejection(rejected);
        if let Some(entry) = queue
            .pending
            .iter_mut()
            .find(|entry| entry.op_id == rejection.op_id)
        {
            entry.status = PendingOpStatus::Rejected;
            entry.rejection = Some(rejection.clone());
        }
        queue.last_rejection = Some(rejection);
    }
    queue
}

pub fn record_pull_applied(mut queue: PendingOpQueue, response: &PullResponse) -> PendingOpQueue {
    if let Some(cursor) = &response.server_cursor {
        queue.client_applied_through = Some(cursor.clone());
    }
    queue
}

impl PendingOpQueue {
    pub fn counts(&self) -> QueueCounts {
        QueueCounts {
            queued: self.pending.len(),
            dirty: self
                .pending
                .iter()
                .filter(|entry| entry.status != PendingOpStatus::Rejected)
                .count(),
            rejected: self
                .pending
                .iter()
                .filter(|entry| entry.status == PendingOpStatus::Rejected)
                .count(),
        }
    }
}

fn to_pending_rejection(rejection: OperationRejection) -> PendingRejection {
    PendingRejection {
        op_id: rejection.op_id,
        code: rejection.code,
        message: rejection.message,
    }
}

#[cfg(test)]
mod tests;
