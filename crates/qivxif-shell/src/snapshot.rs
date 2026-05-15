use crate::{ShellEvent, ShellModel};
use qivxif_browser::{BrowserPolicy, BrowserState};
use qivxif_editor_buffer::{BufferId, TextBuffer};
use qivxif_editor_view::EditorView;
use qivxif_explorer::ExplorerModel;
use qivxif_markdown::MarkdownDocument;
use qivxif_persistence::{JsonStore, PersistenceError};
use qivxif_workspace::{PaneBinding, WorkspaceSession};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShellSnapshot {
    pub session: WorkspaceSession,
    pub buffers: Vec<BufferSnapshot>,
    pub explorer: ExplorerModel,
    pub browser_state: Option<BrowserState>,
    pub markdown: MarkdownDocument,
    pub events: Vec<ShellEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BufferSnapshot {
    pub id: BufferId,
    pub text: String,
}

#[derive(Debug, Clone, Copy)]
pub struct SnapshotSummary {
    pub panes: usize,
    pub buffers: usize,
    pub markdown_blocks: usize,
    pub explorer_entries: usize,
}

impl ShellSnapshot {
    pub fn load(path: PathBuf) -> Result<Self, PersistenceError> {
        JsonStore::new(path).load()
    }

    pub fn save(&self, path: PathBuf) -> Result<(), PersistenceError> {
        JsonStore::new(path).save(self)
    }

    pub fn from_shell(shell: &ShellModel) -> Self {
        Self {
            session: shell.session.clone(),
            buffers: shell
                .buffers
                .iter()
                .map(|buffer| BufferSnapshot {
                    id: buffer.id(),
                    text: buffer.text(),
                })
                .collect(),
            explorer: shell.explorer.clone(),
            browser_state: shell.browser_state.clone(),
            markdown: shell.markdown.clone(),
            events: shell.events.clone(),
        }
    }

    pub fn into_shell(self, policy: BrowserPolicy) -> ShellModel {
        ShellModel {
            editor_views: editor_views(&self.session),
            session: self.session,
            buffers: self
                .buffers
                .into_iter()
                .map(|buffer| TextBuffer::with_id(buffer.id, buffer.text))
                .collect(),
            explorer: self.explorer,
            browser_policy: policy,
            browser_state: self.browser_state,
            markdown: self.markdown,
            events: self.events,
        }
    }

    pub fn summary(&self) -> SnapshotSummary {
        SnapshotSummary {
            panes: self.session.panes.len(),
            buffers: self.buffers.len(),
            markdown_blocks: self.markdown.blocks.len(),
            explorer_entries: self.explorer.entries.len(),
        }
    }
}

fn editor_views(session: &WorkspaceSession) -> Vec<EditorView> {
    session
        .panes
        .iter()
        .filter_map(|pane| match pane.binding {
            PaneBinding::Editor { buffer_id } => Some(EditorView::new(buffer_id)),
            _ => None,
        })
        .collect()
}
