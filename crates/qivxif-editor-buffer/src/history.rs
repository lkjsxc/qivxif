use crate::{EditBatch, TextBuffer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoHistory {
    undo: Vec<EditBatch>,
    redo: Vec<EditBatch>,
}

impl UndoHistory {
    pub fn record(&mut self, batch: EditBatch) {
        if batch.before != batch.after {
            self.undo.push(batch);
            self.redo.clear();
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn undo(&mut self, buffer: &mut TextBuffer) -> bool {
        let Some(batch) = self.undo.pop() else {
            return false;
        };
        buffer.restore_text(batch.before.clone());
        self.redo.push(batch);
        true
    }

    pub fn redo(&mut self, buffer: &mut TextBuffer) -> bool {
        let Some(batch) = self.redo.pop() else {
            return false;
        };
        buffer.restore_text(batch.after.clone());
        self.undo.push(batch);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TextEdit;

    #[test]
    fn undo_and_redo_restore_text() {
        let mut buffer = TextBuffer::new("ab");
        let mut history = UndoHistory::default();
        history.record(buffer.apply(TextEdit::insert(2, "c")).unwrap());
        assert_eq!(buffer.text(), "abc");
        assert!(history.undo(&mut buffer));
        assert_eq!(buffer.text(), "ab");
        assert!(history.redo(&mut buffer));
        assert_eq!(buffer.text(), "abc");
    }
}
