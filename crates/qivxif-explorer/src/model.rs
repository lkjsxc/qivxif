use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExplorerError {
    #[error("failed to read {path}: {source}")]
    ReadDir {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerModel {
    pub roots: Vec<PathBuf>,
    pub entries: Vec<ExplorerEntry>,
    pub show_hidden: bool,
}

impl ExplorerModel {
    pub fn with_root(root: PathBuf) -> Self {
        Self {
            roots: vec![root],
            entries: Vec::new(),
            show_hidden: false,
        }
    }

    pub fn refresh_root(&mut self, index: usize) -> Result<(), ExplorerError> {
        let Some(root) = self.roots.get(index).cloned() else {
            self.entries.clear();
            return Ok(());
        };
        let mut entries = Vec::new();
        for entry in fs::read_dir(&root).map_err(|source| ExplorerError::ReadDir {
            path: root.clone(),
            source,
        })? {
            let entry = entry.map_err(|source| ExplorerError::ReadDir {
                path: root.clone(),
                source,
            })?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if !self.show_hidden && name.starts_with('.') {
                continue;
            }
            entries.push(ExplorerEntry {
                is_dir: path.is_dir(),
                path,
                name,
            });
        }
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        self.entries = entries;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refresh_lists_visible_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("b.txt"), "b").unwrap();
        fs::write(dir.path().join(".hidden"), "x").unwrap();
        let mut model = ExplorerModel::with_root(dir.path().to_path_buf());
        model.refresh_root(0).unwrap();
        assert_eq!(model.entries.len(), 1);
        assert_eq!(model.entries[0].name, "b.txt");
    }
}
