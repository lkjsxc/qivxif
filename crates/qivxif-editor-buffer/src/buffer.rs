use crate::{BufferError, BufferId, EditBatch, TextEdit, TextRange};
use ropey::Rope;

#[derive(Debug, Clone)]
pub struct TextBuffer {
    id: BufferId,
    text: Rope,
    dirty: bool,
    revision: u64,
}

impl TextBuffer {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: BufferId::fresh(),
            text: Rope::from_str(&text.into()),
            dirty: false,
            revision: 0,
        }
    }

    pub fn with_id(id: BufferId, text: impl Into<String>) -> Self {
        Self {
            id,
            text: Rope::from_str(&text.into()),
            dirty: false,
            revision: 0,
        }
    }

    pub fn id(&self) -> BufferId {
        self.id
    }

    pub fn text(&self) -> String {
        self.text.to_string()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    pub fn line_count(&self) -> usize {
        self.text.len_lines()
    }

    pub fn line(&self, index: usize) -> Option<String> {
        (index < self.text.len_lines()).then(|| self.text.line(index).to_string())
    }

    pub fn apply(&mut self, edit: TextEdit) -> Result<EditBatch, BufferError> {
        self.apply_many([edit])
    }

    pub fn apply_many(
        &mut self,
        edits: impl IntoIterator<Item = TextEdit>,
    ) -> Result<EditBatch, BufferError> {
        let before = self.text.to_string();
        let mut staged = before.clone();
        let mut edits: Vec<_> = edits.into_iter().collect();
        edits.sort_by_key(|edit| edit.range.start);
        for edit in edits.into_iter().rev() {
            validate_range(&staged, edit.range)?;
            staged.replace_range(edit.range.start..edit.range.end, &edit.replacement);
        }
        self.text = Rope::from_str(&staged);
        self.revision += 1;
        self.dirty = true;
        Ok(EditBatch {
            before,
            after: staged,
        })
    }

    pub fn restore_text(&mut self, text: String) {
        if self.text != text {
            self.text = Rope::from_str(&text);
            self.revision += 1;
            self.dirty = true;
        }
    }
}

fn validate_range(text: &str, range: TextRange) -> Result<(), BufferError> {
    if range.start > range.end {
        return Err(BufferError::ReversedRange);
    }
    if range.end > text.len() {
        return Err(BufferError::OutOfBounds);
    }
    if !text.is_char_boundary(range.start) || !text.is_char_boundary(range.end) {
        return Err(BufferError::InvalidBoundary);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edit_replaces_valid_utf8_range() {
        let mut buffer = TextBuffer::new("alpha beta");
        buffer
            .apply(TextEdit::replace(TextRange::new(6, 10), "gamma"))
            .unwrap();
        assert_eq!(buffer.text(), "alpha gamma");
        assert!(buffer.is_dirty());
    }

    #[test]
    fn edit_rejects_non_boundary_offsets() {
        let mut buffer = TextBuffer::new("aé");
        let err = buffer.apply(TextEdit::insert(2, "!")).unwrap_err();
        assert_eq!(err, BufferError::InvalidBoundary);
    }
}
