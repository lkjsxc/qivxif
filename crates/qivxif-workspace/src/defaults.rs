use crate::{AppSettings, BufferState, PaneBinding, PaneState, WorkspaceSession};
use qivxif_tiles::{PaneKind, SplitAxis, TileLayout};
use std::path::PathBuf;

impl WorkspaceSession {
    pub fn default_workspace(root: Option<PathBuf>) -> Self {
        let buffer = BufferState::scratch("scratch");
        let explorer = PaneState::new(
            PaneKind::Explorer,
            "Explorer",
            PaneBinding::Explorer { root },
        );
        let editor = PaneState::editor(buffer.id, buffer.label.clone());
        let markdown = PaneState::new(
            PaneKind::Markdown,
            "Markdown",
            PaneBinding::Markdown {
                source_buffer_id: Some(buffer.id),
            },
        );
        let browser = PaneState::new(
            PaneKind::Browser,
            "Browser",
            PaneBinding::Browser { browser_id: 1 },
        );
        let settings = PaneState::new(PaneKind::Settings, "Settings", PaneBinding::Settings);
        let mut session = Self {
            panes: vec![
                explorer.clone(),
                editor.clone(),
                markdown,
                browser,
                settings,
            ],
            layout: TileLayout::single(explorer.id),
            buffers: vec![buffer],
            focused_pane: editor.id,
            settings: AppSettings::default(),
        };
        session
            .layout
            .split_focused(editor.id, SplitAxis::Vertical, 0.25);
        session.layout.tab_focused(session.panes[2].id);
        session.layout.tab_focused(session.panes[3].id);
        session.layout.tab_focused(session.panes[4].id);
        session.layout.focus(editor.id);
        session
    }
}
