use qivxif_action::{AppCommand, CommandEnvelope, SplitDirection};
use qivxif_browser::{BrowserPolicy, BrowserState};
use qivxif_editor_buffer::TextBuffer;
use qivxif_editor_view::EditorView;
use qivxif_explorer::ExplorerModel;
use qivxif_markdown::{MarkdownDocument, parse_markdown};
use qivxif_tiles::SplitAxis;
use qivxif_workspace::WorkspaceSession;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug)]
pub struct ShellModel {
    pub session: WorkspaceSession,
    pub buffers: Vec<TextBuffer>,
    pub editor_views: Vec<EditorView>,
    pub explorer: ExplorerModel,
    pub browser_policy: BrowserPolicy,
    pub browser_state: Option<BrowserState>,
    pub markdown: MarkdownDocument,
    pub events: Vec<ShellEvent>,
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
        let session = WorkspaceSession::new_editor("scratch");
        let buffer_id = session.buffers[0].id;
        Self {
            session,
            buffers: vec![TextBuffer::with_id(buffer_id, "")],
            editor_views: vec![EditorView::new(buffer_id)],
            explorer: ExplorerModel::default(),
            browser_policy,
            browser_state: None,
            markdown: MarkdownDocument::default(),
            events: Vec::new(),
        }
    }

    pub fn dispatch(&mut self, envelope: CommandEnvelope) {
        match envelope.command {
            AppCommand::NewScratchBuffer => {
                self.new_scratch("scratch");
            }
            AppCommand::OpenPath(path) => {
                if let Err(error) = self.open_file(path) {
                    self.events.push(ShellEvent::Error(error.to_string()));
                }
            }
            AppCommand::SaveFocused => match self.save_focused() {
                Ok(()) => self.events.push(ShellEvent::SavedFocused),
                Err(error) => self.events.push(ShellEvent::Error(error.to_string())),
            },
            AppCommand::CloseFocusedPane => {
                self.session.close(self.session.focused_pane);
                self.events.push(ShellEvent::PaneChanged);
            }
            AppCommand::SplitFocused(direction) => self.split_focused(direction),
            AppCommand::FocusNextPane => self.focus_next(),
            AppCommand::ToggleExplorer => {
                let root = self.explorer.roots.first().cloned();
                let pane = self.session.add_explorer(root);
                self.session.split_focused(pane, SplitAxis::Vertical, 0.35);
                self.events.push(ShellEvent::PaneChanged);
            }
            AppCommand::ToggleMarkdownPreview => {
                let source = self.session.focused_editor_buffer();
                let pane = self.session.add_markdown(source);
                self.session.split_focused(pane, SplitAxis::Vertical, 0.5);
                self.events.push(ShellEvent::PaneChanged);
            }
            AppCommand::OpenBrowser(url) => {
                self.browser_state = Some(BrowserState::new(url.clone()));
                let pane = self.session.add_browser(1);
                self.session.split_focused(pane, SplitAxis::Vertical, 0.5);
                self.events.push(ShellEvent::BrowserOpened(url));
            }
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
        self.editor_views.push(EditorView::new(buffer));
        let pane = self.session.add_editor(buffer, label);
        self.session.split_focused(pane, SplitAxis::Vertical, 0.5);
        self.events.push(ShellEvent::PaneChanged);
    }

    fn split_focused(&mut self, direction: SplitDirection) {
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

    fn focus_next(&mut self) {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use qivxif_action::{CommandEnvelope, CommandSource};

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
