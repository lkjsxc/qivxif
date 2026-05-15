use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaneKind {
    Editor,
    Explorer,
    Markdown,
    Browser,
    Settings,
}
