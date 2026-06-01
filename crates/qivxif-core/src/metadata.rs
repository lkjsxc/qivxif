use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct MetadataMap(BTreeMap<String, String>);

impl MetadataMap {
    pub fn new(entries: BTreeMap<String, String>) -> Self {
        Self(entries)
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }

    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.0
    }
}
