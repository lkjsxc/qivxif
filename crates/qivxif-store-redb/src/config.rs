use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoreConfig {
    pub database_file: PathBuf,
}

impl StoreConfig {
    pub fn new(database_file: impl Into<PathBuf>) -> Self {
        Self {
            database_file: database_file.into(),
        }
    }
}
