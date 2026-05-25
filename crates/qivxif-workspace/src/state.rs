use crate::{BufferState, PaneBinding, PaneState};
use qivxif_editor_buffer::BufferId;
use qivxif_tiles::{PaneId, PaneKind};
use std::path::PathBuf;

impl BufferState {
    pub(crate) fn scratch(label: impl Into<String>) -> Self {
        Self {
            id: BufferId::fresh(),
            path: None,
            label: label.into(),
            dirty: false,
        }
    }

    pub(crate) fn file(path: PathBuf) -> Self {
        let label = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("untitled")
            .to_owned();
        Self {
            id: BufferId::fresh(),
            path: Some(path),
            label,
            dirty: false,
        }
    }
}

impl PaneState {
    pub fn editor(buffer_id: BufferId, title: impl Into<String>) -> Self {
        Self::new(PaneKind::Editor, title, PaneBinding::Editor { buffer_id })
    }

    pub fn new(kind: PaneKind, title: impl Into<String>, binding: PaneBinding) -> Self {
        Self {
            id: PaneId::fresh(),
            kind,
            title: title.into(),
            binding,
        }
    }
}
