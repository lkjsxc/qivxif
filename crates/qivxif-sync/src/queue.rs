use qivxif_api::{EventRejection, PullResponse, PushResponse};
use qivxif_core::{CursorId, EventId};
use qivxif_history::{EventEnvelope, EventKind};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PendingEventQueue {
    pub pending: Vec<PendingEvent>,
    pub client_uploaded_through: Option<CursorId>,
    pub client_applied_through: Option<CursorId>,
    pub last_rejection: Option<PendingRejection>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PendingEvent {
    pub event_id: EventId,
    pub actor_seq: u64,
    pub kind: EventKind,
    pub status: PendingEventStatus,
    pub rejection: Option<PendingRejection>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PendingEventStatus {
    Dirty,
    PendingValidation,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PendingRejection {
    pub event_id: EventId,
    pub code: String,
    pub message: String,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct QueueCounts {
    pub queued: usize,
    pub dirty: usize,
    pub rejected: usize,
}

pub fn queue_event(mut queue: PendingEventQueue, event: EventEnvelope) -> PendingEventQueue {
    if queue
        .pending
        .iter()
        .any(|entry| entry.event_id == event.event_id)
    {
        return queue;
    }
    queue.pending.push(PendingEvent {
        event_id: event.event_id,
        actor_seq: event.actor_seq,
        kind: event.kind,
        status: PendingEventStatus::Dirty,
        rejection: None,
    });
    queue.pending.sort_by(|left, right| {
        (left.actor_seq, &left.event_id).cmp(&(right.actor_seq, &right.event_id))
    });
    queue
}

pub fn mark_upload_started(
    mut queue: PendingEventQueue,
    event_ids: &[EventId],
) -> PendingEventQueue {
    for entry in &mut queue.pending {
        if event_ids.contains(&entry.event_id) && entry.status == PendingEventStatus::Dirty {
            entry.status = PendingEventStatus::PendingValidation;
        }
    }
    queue
}

pub fn record_network_failure(mut queue: PendingEventQueue) -> PendingEventQueue {
    for entry in &mut queue.pending {
        if entry.status == PendingEventStatus::PendingValidation {
            entry.status = PendingEventStatus::Dirty;
        }
    }
    queue
}

pub fn apply_push_response(
    mut queue: PendingEventQueue,
    response: PushResponse,
) -> PendingEventQueue {
    if let Some(cursor) = response.server_cursor {
        queue.client_uploaded_through = Some(cursor);
    } else if let Some(last) = response.accepted.last() {
        queue.client_uploaded_through = Some(last.server_cursor.clone());
    }
    for accepted in response.accepted {
        queue
            .pending
            .retain(|entry| entry.event_id != accepted.event_id);
    }
    for rejected in response.rejected {
        let rejection = to_pending_rejection(rejected);
        if let Some(entry) = queue
            .pending
            .iter_mut()
            .find(|entry| entry.event_id == rejection.event_id)
        {
            entry.status = PendingEventStatus::Rejected;
            entry.rejection = Some(rejection.clone());
        }
        queue.last_rejection = Some(rejection);
    }
    queue
}

pub fn record_pull_applied(
    mut queue: PendingEventQueue,
    response: &PullResponse,
) -> PendingEventQueue {
    if let Some(cursor) = &response.server_cursor {
        queue.client_applied_through = Some(cursor.clone());
    }
    queue
}

impl PendingEventQueue {
    pub fn counts(&self) -> QueueCounts {
        QueueCounts {
            queued: self.pending.len(),
            dirty: self
                .pending
                .iter()
                .filter(|entry| entry.status != PendingEventStatus::Rejected)
                .count(),
            rejected: self
                .pending
                .iter()
                .filter(|entry| entry.status == PendingEventStatus::Rejected)
                .count(),
        }
    }
}

fn to_pending_rejection(rejection: EventRejection) -> PendingRejection {
    PendingRejection {
        event_id: rejection.event_id,
        code: rejection.code,
        message: rejection.message,
    }
}

#[cfg(test)]
mod tests;
