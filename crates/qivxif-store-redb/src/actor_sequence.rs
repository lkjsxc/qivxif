use crate::{StoreError, StoreResult, store::QivxifStore, tables};
use qivxif_core::ActorId;
use redb::ReadableTable;

impl QivxifStore {
    pub fn next_actor_seq(&self, actor_id: &ActorId) -> StoreResult<u64> {
        let tx = self.database.begin_read()?;
        let actor_ops = tx.open_table(tables::OPS_BY_ACTOR)?;
        let prefix = format!("{}:", actor_id.as_str());
        let mut next = 1;
        for item in actor_ops.iter()? {
            let (key, _) = item?;
            let Some(seq_text) = key.value().strip_prefix(&prefix) else {
                continue;
            };
            let seq = seq_text
                .parse::<u64>()
                .map_err(|_| StoreError::InvalidOperation)?;
            next = next.max(seq.saturating_add(1));
        }
        Ok(next)
    }
}
