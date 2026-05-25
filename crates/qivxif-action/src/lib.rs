use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppCommand {
    NewScratchBuffer,
    OpenPath(PathBuf),
    SaveFocused,
    EditBuffer { buffer_id: u64, text: String },
    UndoFocused,
    RedoFocused,
    CloseFocusedPane,
    ClosePane(u64),
    SplitFocused(SplitDirection),
    TabFocused(PaneSpawn),
    SpawnPane(PaneSpawn, PanePlacement),
    FocusNextPane,
    FocusPane(u64),
    ToggleMaximize,
    ToggleExplorer,
    ToggleMarkdownPreview,
    OpenBrowser(String),
    BrowserBack,
    BrowserForward,
    BrowserReload,
    BrowserOpenExternal,
    ToggleHiddenFiles,
    RefreshExplorer,
    PersistSettings,
    SetFontSize(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    Right,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaneSpawn {
    Editor { buffer_id: Option<u64> },
    Explorer,
    Markdown { source_buffer_id: Option<u64> },
    Browser { url: Option<String> },
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanePlacement {
    SplitRight,
    SplitDown,
    TabFocused,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandEnvelope {
    pub source: CommandSource,
    pub command: AppCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandSource {
    Menu,
    Shortcut,
    Pane,
    Startup,
}

impl CommandEnvelope {
    pub fn new(source: CommandSource, command: AppCommand) -> Self {
        Self { source, command }
    }
}
