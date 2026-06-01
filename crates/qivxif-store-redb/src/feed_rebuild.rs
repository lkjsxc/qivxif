use crate::{
    FeedItem, StoreResult, codec::decode, feed_audience::audience_users,
    feed_support::feed_user_key, store::QivxifStore, tables,
};
use redb::ReadableTable;
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FeedRebuildReport {
    pub items: u64,
    pub markers: u64,
}

impl QivxifStore {
    pub fn rebuild_feed_indexes(&self) -> StoreResult<FeedRebuildReport> {
        let tx = self.database.begin_write()?;
        clear_user_index(&tx)?;
        let items = feed_items(&tx)?;
        let mut markers = 0;
        for item in &items {
            let audience = audience_users(&tx, &item.author_user_id)?;
            let mut by_user = tx.open_table(tables::FEED_ITEMS_BY_USER)?;
            for user_id in audience {
                by_user.insert(
                    feed_user_key(&user_id, &item.operation_id).as_str(),
                    ([] as [u8; 0]).as_slice(),
                )?;
                markers += 1;
            }
        }
        tx.commit()?;
        Ok(FeedRebuildReport {
            items: items.len() as u64,
            markers,
        })
    }
}

fn clear_user_index(tx: &redb::WriteTransaction) -> StoreResult<()> {
    let keys = {
        let by_user = tx.open_table(tables::FEED_ITEMS_BY_USER)?;
        let mut keys = Vec::new();
        for item in by_user.iter()? {
            let (key, _) = item?;
            keys.push(key.value().to_owned());
        }
        keys
    };
    let mut by_user = tx.open_table(tables::FEED_ITEMS_BY_USER)?;
    for key in keys {
        by_user.remove(key.as_str())?;
    }
    Ok(())
}

fn feed_items(tx: &redb::WriteTransaction) -> StoreResult<Vec<FeedItem>> {
    let table = tx.open_table(tables::FEED_ITEMS)?;
    let mut out = Vec::new();
    for item in table.iter()? {
        let (_, bytes) = item?;
        out.push(decode(bytes.value())?);
    }
    Ok(out)
}
