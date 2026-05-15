use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppCommand {
    NewScratchBuffer,
    OpenPath(PathBuf),
    SaveFocused,
    CloseFocusedPane,
    SplitFocused(SplitDirection),
    FocusNextPane,
    ToggleExplorer,
    ToggleMarkdownPreview,
    OpenBrowser(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    Right,
    Down,
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
