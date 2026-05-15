use qivxif_editor_buffer::BufferId;
use qivxif_tiles::{PaneId, PaneKind, SplitAxis, TileLayout};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BufferState {
    pub id: BufferId,
    pub path: Option<PathBuf>,
    pub label: String,
    pub dirty: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaneBinding {
    Editor { buffer_id: BufferId },
    Markdown { source_buffer_id: Option<BufferId> },
    Explorer { root: Option<PathBuf> },
    Browser { browser_id: u64 },
    Settings,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneState {
    pub id: PaneId,
    pub kind: PaneKind,
    pub title: String,
    pub binding: PaneBinding,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceSession {
    pub panes: Vec<PaneState>,
    pub layout: TileLayout,
    pub buffers: Vec<BufferState>,
    pub focused_pane: PaneId,
}

impl WorkspaceSession {
    pub fn new_editor(label: impl Into<String>) -> Self {
        let buffer = BufferState::scratch(label);
        let pane = PaneState::editor(buffer.id, buffer.label.clone());
        let focused_pane = pane.id;
        Self {
            panes: vec![pane],
            layout: TileLayout::single(focused_pane),
            buffers: vec![buffer],
            focused_pane,
        }
    }

    pub fn add_file_buffer(&mut self, path: PathBuf) -> BufferId {
        let state = BufferState::file(path);
        let id = state.id;
        self.buffers.push(state);
        id
    }

    pub fn add_editor(&mut self, buffer_id: BufferId, title: impl Into<String>) -> PaneId {
        self.add_bound_pane(PaneState::editor(buffer_id, title))
    }

    pub fn add_explorer(&mut self, root: Option<PathBuf>) -> PaneId {
        self.add_bound_pane(PaneState::new(
            PaneKind::Explorer,
            "Explorer",
            PaneBinding::Explorer { root },
        ))
    }

    pub fn add_markdown(&mut self, source_buffer_id: Option<BufferId>) -> PaneId {
        self.add_bound_pane(PaneState::new(
            PaneKind::Markdown,
            "Markdown",
            PaneBinding::Markdown { source_buffer_id },
        ))
    }

    pub fn add_browser(&mut self, browser_id: u64) -> PaneId {
        self.add_bound_pane(PaneState::new(
            PaneKind::Browser,
            "Browser",
            PaneBinding::Browser { browser_id },
        ))
    }

    pub fn add_settings(&mut self) -> PaneId {
        self.add_bound_pane(PaneState::new(
            PaneKind::Settings,
            "Settings",
            PaneBinding::Settings,
        ))
    }

    pub fn split_focused(&mut self, pane: PaneId, axis: SplitAxis, ratio: f32) {
        self.layout.split_focused(pane, axis, ratio);
        self.focused_pane = self.layout.focused;
    }

    pub fn focus(&mut self, pane: PaneId) -> bool {
        let changed = self.layout.focus(pane);
        if changed {
            self.focused_pane = pane;
        }
        changed
    }

    pub fn close(&mut self, pane: PaneId) -> bool {
        let changed = self.layout.close(pane);
        if changed {
            self.panes.retain(|state| state.id != pane);
            self.focused_pane = self.layout.focused;
        }
        changed
    }

    pub fn buffer_mut(&mut self, id: BufferId) -> Option<&mut BufferState> {
        self.buffers.iter_mut().find(|buffer| buffer.id == id)
    }

    pub fn focused_editor_buffer(&self) -> Option<BufferId> {
        self.panes
            .iter()
            .find_map(|pane| match (&pane.binding, pane.id == self.focused_pane) {
                (PaneBinding::Editor { buffer_id }, true) => Some(*buffer_id),
                _ => None,
            })
    }

    pub fn pane(&self, id: PaneId) -> Option<&PaneState> {
        self.panes.iter().find(|pane| pane.id == id)
    }

    fn add_bound_pane(&mut self, pane: PaneState) -> PaneId {
        let id = pane.id;
        self.panes.push(pane);
        self.focused_pane = id;
        id
    }
}

impl BufferState {
    fn scratch(label: impl Into<String>) -> Self {
        Self {
            id: BufferId::fresh(),
            path: None,
            label: label.into(),
            dirty: false,
        }
    }

    fn file(path: PathBuf) -> Self {
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
    fn editor(buffer_id: BufferId, title: impl Into<String>) -> Self {
        Self::new(PaneKind::Editor, title, PaneBinding::Editor { buffer_id })
    }

    fn new(kind: PaneKind, title: impl Into<String>, binding: PaneBinding) -> Self {
        Self {
            id: PaneId::fresh(),
            kind,
            title: title.into(),
            binding,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focused_editor_buffer_follows_pane_binding() {
        let mut session = WorkspaceSession::new_editor("scratch");
        let first = session.focused_editor_buffer().unwrap();
        let second = session.add_file_buffer(PathBuf::from("notes.md"));
        let pane = session.add_editor(second, "notes.md");
        session.split_focused(pane, SplitAxis::Vertical, 0.5);
        assert_eq!(session.focused_editor_buffer(), Some(second));
        session.focus(session.panes[0].id);
        assert_eq!(session.focused_editor_buffer(), Some(first));
    }
}
