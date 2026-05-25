use crate::reducer::reduce_shell;
use qivxif_action::{CommandEnvelope, PanePlacement, SplitDirection};
use qivxif_browser::{BrowserPolicy, BrowserState};
use qivxif_editor_buffer::{BufferId, TextBuffer, UndoHistory};
use qivxif_editor_view::EditorView;
use qivxif_explorer::ExplorerModel;
use qivxif_markdown::{MarkdownDocument, parse_markdown};
use qivxif_tiles::SplitAxis;
use qivxif_workspace::{PaneState, WorkspaceSession};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ShellModel {
    pub session: WorkspaceSession,
    pub buffers: Vec<TextBuffer>,
    pub histories: Vec<EditorHistory>,
    pub editor_views: Vec<EditorView>,
    pub explorer: ExplorerModel,
    pub browser_policy: BrowserPolicy,
    pub browser_state: Option<BrowserState>,
    pub markdown: MarkdownDocument,
    pub events: Vec<ShellEvent>,
}

#[derive(Debug, Clone)]
pub struct EditorHistory {
    pub buffer_id: BufferId,
    pub history: UndoHistory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShellEvent {
    OpenedPath(PathBuf),
    SavedFocused,
    PaneChanged,
    BrowserOpened(String),
    MarkdownRendered(usize),
    Error(String),
}

impl ShellModel {
    pub fn new(browser_policy: BrowserPolicy) -> Self {
        let session = WorkspaceSession::default_workspace(None);
        let buffer_id = session.buffers[0].id;
        Self {
            session,
            buffers: vec![TextBuffer::with_id(buffer_id, "")],
            histories: vec![EditorHistory {
                buffer_id,
                history: UndoHistory::default(),
            }],
            editor_views: vec![EditorView::new(buffer_id)],
            explorer: ExplorerModel::default(),
            browser_policy,
            browser_state: None,
            markdown: MarkdownDocument::default(),
            events: Vec::new(),
        }
    }

    pub fn dispatch(&mut self, envelope: CommandEnvelope) {
        let transition = reduce_shell(self, envelope);
        *self = transition.state;
        self.events.extend(transition.events);
        for effect in transition.effects {
            self.execute_effect(effect);
        }
    }

    pub fn render_markdown(&mut self, source: &str) {
        self.markdown = parse_markdown(source);
        self.events
            .push(ShellEvent::MarkdownRendered(self.markdown.blocks.len()));
    }

    pub fn new_scratch(&mut self, label: impl Into<String>) {
        let label = label.into();
        let buffer = self.session.add_file_buffer(PathBuf::new());
        if let Some(state) = self.session.buffer_mut(buffer) {
            state.path = None;
            state.label = label.clone();
        }
        self.buffers.push(TextBuffer::with_id(buffer, ""));
        self.histories.push(EditorHistory {
            buffer_id: buffer,
            history: UndoHistory::default(),
        });
        self.editor_views.push(EditorView::new(buffer));
        let pane = self.session.add_editor(buffer, label);
        self.session.split_focused(pane, SplitAxis::Vertical, 0.5);
        self.events.push(ShellEvent::PaneChanged);
    }

    pub(crate) fn split_focused(&mut self, direction: SplitDirection) {
        let Some(buffer_id) = self.session.focused_editor_buffer() else {
            return;
        };
        let pane = self.session.add_editor(buffer_id, "split");
        let axis = match direction {
            SplitDirection::Right => SplitAxis::Vertical,
            SplitDirection::Down => SplitAxis::Horizontal,
        };
        self.session.split_focused(pane, axis, 0.5);
        self.events.push(ShellEvent::PaneChanged);
    }

    pub(crate) fn focus_next(&mut self) {
        let Some(index) = self
            .session
            .panes
            .iter()
            .position(|pane| pane.id == self.session.focused_pane)
        else {
            return;
        };
        let next = (index + 1) % self.session.panes.len();
        let next_id = self.session.panes[next].id;
        self.session.focus(next_id);
        self.events.push(ShellEvent::PaneChanged);
    }

    pub(crate) fn place_pane(&mut self, pane: PaneState, placement: PanePlacement) {
        let id = pane.id;
        self.session.panes.push(pane);
        match placement {
            PanePlacement::SplitRight => self.session.split_focused(id, SplitAxis::Vertical, 0.5),
            PanePlacement::SplitDown => self.session.split_focused(id, SplitAxis::Horizontal, 0.5),
            PanePlacement::TabFocused => self.session.tab_focused(id),
        }
        self.events.push(ShellEvent::PaneChanged);
    }

    pub(crate) fn undo_focused(&mut self) {
        let Some(id) = self.session.focused_editor_buffer() else {
            return;
        };
        let Some(buffer) = self.buffers.iter_mut().find(|buffer| buffer.id() == id) else {
            return;
        };
        let Some(history) = self.histories.iter_mut().find(|item| item.buffer_id == id) else {
            return;
        };
        if history.history.undo(buffer)
            && let Some(state) = self.session.buffer_mut(id)
        {
            state.dirty = buffer.is_dirty();
        }
    }

    pub(crate) fn redo_focused(&mut self) {
        let Some(id) = self.session.focused_editor_buffer() else {
            return;
        };
        let Some(buffer) = self.buffers.iter_mut().find(|buffer| buffer.id() == id) else {
            return;
        };
        let Some(history) = self.histories.iter_mut().find(|item| item.buffer_id == id) else {
            return;
        };
        if history.history.redo(buffer)
            && let Some(state) = self.session.buffer_mut(id)
        {
            state.dirty = buffer.is_dirty();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qivxif_action::{AppCommand, CommandEnvelope, CommandSource};

    #[test]
    fn commands_open_browser_and_render_markdown() {
        let mut shell = ShellModel::new(BrowserPolicy::locked_down("downloads".into()));
        shell.dispatch(CommandEnvelope::new(
            CommandSource::Shortcut,
            AppCommand::OpenBrowser("https://example.com".into()),
        ));
        shell.render_markdown("# Hello");
        assert_eq!(
            shell.browser_state.unwrap().current_url,
            "https://example.com"
        );
        assert_eq!(shell.markdown.blocks.len(), 1);
    }
}
