mod reducer;

use qivxif_core::{ActorId, EventId, TextDocId};
use serde::{Deserialize, Serialize};

pub use reducer::{apply_text_event, restore_text, snapshot_text};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextSnapshotRef {
    pub doc_id: TextDocId,
    pub after_event: EventId,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, Serialize)]
pub struct TextCharId {
    pub actor_id: ActorId,
    pub seq: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextAtom {
    pub id: TextCharId,
    pub after: Option<TextCharId>,
    pub value: char,
    pub deleted: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextDocState {
    pub content: String,
    pub atoms: Vec<TextAtom>,
    pub applied_events: Vec<EventId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextEvent {
    pub event_id: EventId,
    pub doc_id: TextDocId,
    pub edit: TextEdit,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TextEdit {
    Insert(TextInsert),
    Delete(TextDelete),
    Restore(TextRestore),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextInsert {
    pub after: Option<TextCharId>,
    pub chars: Vec<TextInsertedChar>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextInsertedChar {
    pub id: TextCharId,
    pub value: char,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextDelete {
    pub ids: Vec<TextCharId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextRestore {
    pub content: String,
    pub actor_id: ActorId,
    pub first_seq: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextSnapshot {
    pub doc_id: TextDocId,
    pub after_event: EventId,
    pub content: String,
}
