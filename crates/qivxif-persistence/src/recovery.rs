use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryJournal {
    pub records: Vec<RecoveryRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRecord {
    pub buffer_id: u64,
    pub path: Option<PathBuf>,
    pub label: String,
    pub revision: u64,
    pub text: String,
}

impl RecoveryJournal {
    pub fn append(
        &mut self,
        buffer_id: u64,
        path: Option<PathBuf>,
        label: impl Into<String>,
        revision: u64,
        text: impl Into<String>,
    ) {
        self.records.push(RecoveryRecord {
            buffer_id,
            path,
            label: label.into(),
            revision,
            text: text.into(),
        });
    }

    pub fn latest_text(&self) -> Option<&str> {
        self.records.last().map(|record| record.text.as_str())
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_text_tracks_last_record() {
        let mut journal = RecoveryJournal::default();
        journal.append(1, None, "a", 1, "first");
        journal.append(1, None, "a", 2, "second");
        assert_eq!(journal.latest_text(), Some("second"));
    }
}
