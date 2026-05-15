use qivxif_editor_buffer::BufferId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorView {
    pub buffer_id: BufferId,
    pub cursor: usize,
    pub scroll_line: usize,
    pub selection: Option<ViewSelection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewSelection {
    pub anchor: usize,
    pub active: usize,
}

impl EditorView {
    pub fn new(buffer_id: BufferId) -> Self {
        Self {
            buffer_id,
            cursor: 0,
            scroll_line: 0,
            selection: None,
        }
    }

    pub fn move_cursor(&mut self, offset: usize) {
        self.cursor = offset;
        self.selection = None;
    }

    pub fn select(&mut self, anchor: usize, active: usize) {
        self.cursor = active;
        self.selection = Some(ViewSelection { anchor, active });
    }

    pub fn normalized_selection(&self) -> Option<(usize, usize)> {
        self.selection
            .map(|selection| ordered(selection.anchor, selection.active))
    }
}

fn ordered(a: usize, b: usize) -> (usize, usize) {
    if a <= b { (a, b) } else { (b, a) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_normalizes() {
        let mut view = EditorView::new(BufferId::from_raw(9));
        view.select(8, 2);
        assert_eq!(view.normalized_selection(), Some((2, 8)));
    }
}
