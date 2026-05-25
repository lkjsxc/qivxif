use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeadingLevel {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkdownBlock {
    Heading {
        level: HeadingLevel,
        text: String,
    },
    Paragraph(String),
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    Rule,
    ListItem {
        text: String,
        checked: Option<bool>,
    },
    TableRow(Vec<String>),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkdownDocument {
    pub blocks: Vec<MarkdownBlock>,
    pub source_revision: Option<u64>,
}
