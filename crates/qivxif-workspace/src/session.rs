use qivxif_editor_buffer::BufferId;
use qivxif_tiles::{Pane, PaneId, PaneKind, TileLayout};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BufferState {
    pub id: BufferId,
    pub path: Option<PathBuf>,
    pub label: String,
    pub dirty: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceSession {
    pub panes: Vec<Pane>,
    pub layout: TileLayout,
    pub buffers: Vec<BufferState>,
    pub focused_pane: PaneId,
}

impl WorkspaceSession {
    pub fn new_editor(label: impl Into<String>) -> Self {
        let buffer = BufferState {
            id: BufferId::fresh(),
            path: None,
            label: label.into(),
            dirty: false,
        };
        let pane = Pane::new(PaneKind::Editor, buffer.label.clone());
        let focused_pane = pane.id;
        Self {
            panes: vec![pane],
            layout: TileLayout::single(focused_pane),
            buffers: vec![buffer],
            focused_pane,
        }
    }

    pub fn add_pane(&mut self, kind: PaneKind, title: impl Into<String>) -> PaneId {
        let pane = Pane::new(kind, title);
        let id = pane.id;
        self.panes.push(pane);
        self.focused_pane = id;
        id
    }

    pub fn add_file_buffer(&mut self, path: PathBuf) -> BufferId {
        let label = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("untitled")
            .to_owned();
        let state = BufferState {
            id: BufferId::fresh(),
            path: Some(path),
            label,
            dirty: false,
        };
        let id = state.id;
        self.buffers.push(state);
        id
    }

    pub fn buffer_mut(&mut self, id: BufferId) -> Option<&mut BufferState> {
        self.buffers.iter_mut().find(|buffer| buffer.id == id)
    }

    pub fn focused_editor_buffer(&self) -> Option<BufferId> {
        self.buffers.last().map(|buffer| buffer.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_session_has_editor_pane_and_buffer() {
        let session = WorkspaceSession::new_editor("scratch");
        assert_eq!(session.panes.len(), 1);
        assert_eq!(session.buffers.len(), 1);
        assert_eq!(session.focused_pane, session.layout.focused);
    }
}
