use crate::PaneId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaneKind {
    Editor,
    Explorer,
    Markdown,
    Browser,
    Settings,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pane {
    pub id: PaneId,
    pub kind: PaneKind,
    pub title: String,
}

impl Pane {
    pub fn new(kind: PaneKind, title: impl Into<String>) -> Self {
        Self {
            id: PaneId::fresh(),
            kind,
            title: title.into(),
        }
    }
}
