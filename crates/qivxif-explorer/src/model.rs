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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerScanResult {
    pub root: PathBuf,
    pub entries: Vec<ExplorerEntry>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerModel {
    pub roots: Vec<PathBuf>,
    pub entries: Vec<ExplorerEntry>,
    pub show_hidden: bool,
    pub expanded_dirs: Vec<PathBuf>,
    pub selected_path: Option<PathBuf>,
    pub error: Option<String>,
}

impl ExplorerModel {
    pub fn with_root(root: PathBuf) -> Self {
        Self {
            roots: vec![root],
            entries: Vec::new(),
            show_hidden: false,
            expanded_dirs: Vec::new(),
            selected_path: None,
            error: None,
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
        self.apply_scan(ExplorerScanResult {
            root,
            entries,
            error: None,
        });
        Ok(())
    }

    pub fn apply_scan(&mut self, result: ExplorerScanResult) {
        if !self.roots.contains(&result.root) {
            self.roots.push(result.root);
        }
        let mut entries: Vec<_> = result
            .entries
            .into_iter()
            .filter(|entry| self.show_hidden || !entry.name.starts_with('.'))
            .collect();
        entries.sort_by(|a, b| {
            b.is_dir
                .cmp(&a.is_dir)
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        self.entries = entries;
        self.error = result.error;
    }

    pub fn select(&mut self, path: PathBuf) {
        self.selected_path = Some(path);
    }

    pub fn toggle_expanded(&mut self, path: PathBuf) {
        match self.expanded_dirs.iter().position(|item| item == &path) {
            Some(index) => {
                self.expanded_dirs.remove(index);
            }
            None => self.expanded_dirs.push(path),
        }
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

    #[test]
    fn scan_applies_directory_first_sort() {
        let root = PathBuf::from("/tmp/root");
        let mut model = ExplorerModel::with_root(root.clone());
        model.apply_scan(ExplorerScanResult {
            root,
            error: None,
            entries: vec![
                ExplorerEntry {
                    path: "z.txt".into(),
                    name: "z.txt".into(),
                    is_dir: false,
                },
                ExplorerEntry {
                    path: "a".into(),
                    name: "a".into(),
                    is_dir: true,
                },
            ],
        });
        assert!(model.entries[0].is_dir);
    }
}
