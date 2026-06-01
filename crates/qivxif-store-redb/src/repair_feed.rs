use crate::{
    FeedItem, StoreResult,
    codec::decode,
    repair::{RepairFinding, finding},
    tables,
};
use redb::ReadableTable;

pub(crate) fn check_feed(
    tx: &redb::ReadTransaction,
    findings: &mut Vec<RepairFinding>,
) -> StoreResult<()> {
    let items = tx.open_table(tables::FEED_ITEMS)?;
    let nodes = tx.open_table(tables::NODES)?;
    for item in items.iter()? {
        let (key, bytes) = item?;
        let key = key.value().to_owned();
        let Ok(feed_item) = decode::<FeedItem>(bytes.value()) else {
            findings.push(finding(
                "decode_failed",
                "feed_items",
                &key,
                "feed item did not decode",
            ));
            continue;
        };
        if nodes.get(feed_item.post_node_id.as_str())?.is_none() {
            findings.push(finding(
                "feed_item_post_missing",
                "feed_items",
                &key,
                "post node is missing",
            ));
        }
    }
    let by_user = tx.open_table(tables::FEED_ITEMS_BY_USER)?;
    for item in by_user.iter()? {
        let (key, _) = item?;
        let key = key.value().to_owned();
        let Some((_, event_id)) = key.split_once(':') else {
            findings.push(finding(
                "decode_failed",
                "feed_items_by_user",
                &key,
                "marker key is malformed",
            ));
            continue;
        };
        if items.get(event_id)?.is_none() {
            findings.push(finding(
                "feed_user_index_dangling",
                "feed_items_by_user",
                &key,
                "feed item is missing",
            ));
        }
    }
    Ok(())
}
