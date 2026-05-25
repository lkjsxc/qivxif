use serde::{Serialize, de::DeserializeOwned};
use std::{fs, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("io failed at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("json failed at {path}: {source}")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("toml decode failed at {path}: {source}")]
    TomlDecode {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("toml encode failed at {path}: {source}")]
    TomlEncode {
        path: PathBuf,
        #[source]
        source: toml::ser::Error,
    },
}

#[derive(Debug, Clone)]
pub struct JsonStore {
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct TomlStore {
    path: PathBuf,
}

impl JsonStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn load<T: DeserializeOwned>(&self) -> Result<T, PersistenceError> {
        let text = read_to_string(&self.path)?;
        serde_json::from_str(&text).map_err(|source| PersistenceError::Json {
            path: self.path.clone(),
            source,
        })
    }

    pub fn save<T: Serialize>(&self, value: &T) -> Result<(), PersistenceError> {
        let text =
            serde_json::to_string_pretty(value).map_err(|source| PersistenceError::Json {
                path: self.path.clone(),
                source,
            })?;
        write_string(&self.path, text)
    }
}

impl TomlStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn load<T: DeserializeOwned>(&self) -> Result<T, PersistenceError> {
        let text = read_to_string(&self.path)?;
        toml::from_str(&text).map_err(|source| PersistenceError::TomlDecode {
            path: self.path.clone(),
            source,
        })
    }

    pub fn save<T: Serialize>(&self, value: &T) -> Result<(), PersistenceError> {
        let text =
            toml::to_string_pretty(value).map_err(|source| PersistenceError::TomlEncode {
                path: self.path.clone(),
                source,
            })?;
        write_string(&self.path, text)
    }
}

fn read_to_string(path: &PathBuf) -> Result<String, PersistenceError> {
    fs::read_to_string(path).map_err(|source| PersistenceError::Io {
        path: path.clone(),
        source,
    })
}

fn write_string(path: &PathBuf, text: String) -> Result<(), PersistenceError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| PersistenceError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, text).map_err(|source| PersistenceError::Io {
        path: tmp.clone(),
        source,
    })?;
    fs::rename(&tmp, path).map_err(|source| PersistenceError::Io {
        path: path.clone(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Settings {
        name: String,
    }

    #[test]
    fn toml_store_round_trips_readable_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.toml");
        let store = TomlStore::new(path.clone());
        store.save(&Settings { name: "q".into() }).unwrap();
        assert!(std::fs::read_to_string(&path).unwrap().contains("name"));
        assert_eq!(store.load::<Settings>().unwrap().name, "q");
    }
}
