use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryJournal {
    pub records: Vec<RecoveryRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRecord {
    pub revision: u64,
    pub text: String,
}

impl RecoveryJournal {
    pub fn append(&mut self, revision: u64, text: impl Into<String>) {
        self.records.push(RecoveryRecord {
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
        journal.append(1, "first");
        journal.append(2, "second");
        assert_eq!(journal.latest_text(), Some("second"));
    }
}
