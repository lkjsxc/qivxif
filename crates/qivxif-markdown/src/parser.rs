use crate::{HeadingLevel, MarkdownBlock, MarkdownDocument};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel as MdLevel, Options, Parser, Tag, TagEnd};

pub fn parse_markdown(source: &str) -> MarkdownDocument {
    let parser = Parser::new_ext(source, Options::ENABLE_TABLES | Options::ENABLE_TASKLISTS);
    let mut blocks = Vec::new();
    let mut current = CurrentBlock::None;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                current = CurrentBlock::Heading {
                    level: convert_level(level),
                    text: String::new(),
                };
            }
            Event::Start(Tag::Paragraph) => {
                current = CurrentBlock::Paragraph(String::new());
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                current = CurrentBlock::Code {
                    language: code_language(kind),
                    code: String::new(),
                };
            }
            Event::Text(text) | Event::Code(text) => current.push_text(&text),
            Event::SoftBreak | Event::HardBreak => current.push_text("\n"),
            Event::Rule => blocks.push(MarkdownBlock::Rule),
            Event::End(TagEnd::Heading(_))
            | Event::End(TagEnd::Paragraph)
            | Event::End(TagEnd::CodeBlock) => {
                if let Some(block) = current.take() {
                    blocks.push(block);
                }
            }
            _ => {}
        }
    }

    MarkdownDocument { blocks }
}

enum CurrentBlock {
    None,
    Heading {
        level: HeadingLevel,
        text: String,
    },
    Paragraph(String),
    Code {
        language: Option<String>,
        code: String,
    },
}

impl CurrentBlock {
    fn push_text(&mut self, text: &str) {
        match self {
            CurrentBlock::Heading { text: target, .. }
            | CurrentBlock::Paragraph(target)
            | CurrentBlock::Code { code: target, .. } => target.push_str(text),
            CurrentBlock::None => {}
        }
    }

    fn take(&mut self) -> Option<MarkdownBlock> {
        match std::mem::replace(self, CurrentBlock::None) {
            CurrentBlock::None => None,
            CurrentBlock::Heading { level, text } => Some(MarkdownBlock::Heading { level, text }),
            CurrentBlock::Paragraph(text) => Some(MarkdownBlock::Paragraph(text)),
            CurrentBlock::Code { language, code } => {
                Some(MarkdownBlock::CodeBlock { language, code })
            }
        }
    }
}

fn convert_level(level: MdLevel) -> HeadingLevel {
    match level {
        MdLevel::H1 => HeadingLevel::One,
        MdLevel::H2 => HeadingLevel::Two,
        MdLevel::H3 => HeadingLevel::Three,
        MdLevel::H4 => HeadingLevel::Four,
        MdLevel::H5 => HeadingLevel::Five,
        MdLevel::H6 => HeadingLevel::Six,
    }
}

fn code_language(kind: CodeBlockKind<'_>) -> Option<String> {
    match kind {
        CodeBlockKind::Indented => None,
        CodeBlockKind::Fenced(language) if language.is_empty() => None,
        CodeBlockKind::Fenced(language) => Some(language.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_headings_paragraphs_and_code() {
        let doc = parse_markdown("# Title\n\nBody\n\n```rust\nlet x = 1;\n```");
        assert_eq!(doc.blocks.len(), 3);
        assert!(matches!(doc.blocks[0], MarkdownBlock::Heading { .. }));
        assert!(matches!(doc.blocks[2], MarkdownBlock::CodeBlock { .. }));
    }
}
