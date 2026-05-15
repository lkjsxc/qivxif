use crate::{ShellEvent, ShellModel};
use qivxif_editor_buffer::TextBuffer;
use qivxif_editor_view::EditorView;
use qivxif_tiles::SplitAxis;
use std::{fs, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShellIoError {
    #[error("buffer has no path")]
    NoPath,
    #[error("io failed for {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

impl ShellModel {
    pub fn open_file(&mut self, path: PathBuf) -> Result<(), ShellIoError> {
        let text = fs::read_to_string(&path).map_err(|source| ShellIoError::Io {
            path: path.clone(),
            source,
        })?;
        let id = self.session.add_file_buffer(path.clone());
        self.buffers.push(TextBuffer::with_id(id, text));
        self.editor_views.push(EditorView::new(id));
        let pane = self.session.add_editor(id, label_for_path(&path));
        self.session.split_focused(pane, SplitAxis::Vertical, 0.5);
        self.events.push(ShellEvent::OpenedPath(path));
        Ok(())
    }

    pub fn save_focused(&mut self) -> Result<(), ShellIoError> {
        let id = self
            .session
            .focused_editor_buffer()
            .ok_or(ShellIoError::NoPath)?;
        let state = self.session.buffer_mut(id).ok_or(ShellIoError::NoPath)?;
        let path = state.path.clone().ok_or(ShellIoError::NoPath)?;
        let buffer = self
            .buffers
            .iter_mut()
            .find(|buffer| buffer.id() == id)
            .ok_or(ShellIoError::NoPath)?;
        fs::write(&path, buffer.text()).map_err(|source| ShellIoError::Io {
            path: path.clone(),
            source,
        })?;
        buffer.mark_saved();
        state.dirty = false;
        Ok(())
    }
}

fn label_for_path(path: &std::path::Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("untitled")
        .to_owned()
}
