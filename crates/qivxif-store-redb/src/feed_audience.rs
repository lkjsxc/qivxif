use crate::{FeedItem, StoreResult, codec::decode, records::StoredUser, tables};
use qivxif_core::{NodeId, UserId};
use qivxif_graph::{EdgeKind, EdgeRecord};
use redb::ReadableTable;

pub(crate) fn audience_users(
    tx: &redb::WriteTransaction,
    author_user_id: &UserId,
) -> StoreResult<Vec<UserId>> {
    let users = all_users(tx)?;
    let Some(author) = users.iter().find(|user| &user.id == author_user_id) else {
        return Ok(vec![author_user_id.clone()]);
    };
    let mut out = vec![author_user_id.clone()];
    let edges = tx.open_table(tables::EDGES)?;
    let by_to = tx.open_table(tables::EDGES_BY_TO)?;
    let prefix = format!("{}:", author.profile_node_id.as_str());
    for item in by_to.iter()? {
        let (key, edge_id) = item?;
        if !key.value().starts_with(&prefix) {
            continue;
        }
        let Some(edge_bytes) =
            edges.get(decode::<qivxif_core::EdgeId>(edge_id.value())?.as_str())?
        else {
            continue;
        };
        let edge: EdgeRecord = decode(edge_bytes.value())?;
        if edge.kind == EdgeKind::Follows
            && edge.tombstone.is_none()
            && let Some(user) = user_by_profile_in(&users, &edge.from_node)
            && !out.contains(&user.id)
        {
            out.push(user.id.clone());
        }
    }
    Ok(out)
}

pub(crate) fn user_by_profile(
    tx: &redb::WriteTransaction,
    profile_node_id: &NodeId,
) -> StoreResult<Option<StoredUser>> {
    Ok(user_by_profile_in(&all_users(tx)?, profile_node_id).cloned())
}

pub(crate) fn remove_feed_markers_for_author(
    tx: &redb::WriteTransaction,
    follower_user_id: &UserId,
    author_user_id: &UserId,
) -> StoreResult<()> {
    let mut keys = Vec::new();
    {
        let by_user = tx.open_table(tables::FEED_ITEMS_BY_USER)?;
        let items = tx.open_table(tables::FEED_ITEMS)?;
        let prefix = format!("{}:", follower_user_id.as_str());
        for item in by_user.iter()? {
            let (key, _) = item?;
            let key_text = key.value();
            let Some(event_id) = key_text.strip_prefix(&prefix) else {
                continue;
            };
            if let Some(feed_bytes) = items.get(event_id)? {
                let feed_item: FeedItem = decode(feed_bytes.value())?;
                if &feed_item.author_user_id == author_user_id {
                    keys.push(key_text.to_owned());
                }
            }
        }
    }
    let mut by_user = tx.open_table(tables::FEED_ITEMS_BY_USER)?;
    for key in keys {
        by_user.remove(key.as_str())?;
    }
    Ok(())
}

fn all_users(tx: &redb::WriteTransaction) -> StoreResult<Vec<StoredUser>> {
    let users = tx.open_table(tables::USERS)?;
    let mut out = Vec::new();
    for item in users.iter()? {
        let (_, bytes) = item?;
        out.push(decode(bytes.value())?);
    }
    Ok(out)
}

fn user_by_profile_in<'a>(
    users: &'a [StoredUser],
    profile_node_id: &NodeId,
) -> Option<&'a StoredUser> {
    users
        .iter()
        .find(|user| &user.profile_node_id == profile_node_id)
}
