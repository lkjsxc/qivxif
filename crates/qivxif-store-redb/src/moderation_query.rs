use crate::{StoreResult, codec::decode, records::StoredUser, store::QivxifStore, tables};
use qivxif_core::{NodeId, UserId};
use qivxif_graph::{EdgeKind, EdgeRecord};
use redb::ReadableTable;

pub(crate) fn feed_item_visible(
    tx: &redb::ReadTransaction,
    viewer_user_id: &UserId,
    author_user_id: &UserId,
) -> StoreResult<bool> {
    if viewer_user_id == author_user_id {
        return Ok(true);
    }
    let users = all_users(tx)?;
    let Some(viewer) = user_by_id(&users, viewer_user_id) else {
        return Ok(false);
    };
    let Some(author) = user_by_id(&users, author_user_id) else {
        return Ok(false);
    };
    if has_active_edge(
        tx,
        &viewer.profile_node_id,
        &author.profile_node_id,
        EdgeKind::Mutes,
    )? {
        return Ok(false);
    }
    if has_active_edge(
        tx,
        &viewer.profile_node_id,
        &author.profile_node_id,
        EdgeKind::Blocks,
    )? {
        return Ok(false);
    }
    if has_active_edge(
        tx,
        &author.profile_node_id,
        &viewer.profile_node_id,
        EdgeKind::Blocks,
    )? {
        return Ok(false);
    }
    Ok(true)
}

pub(crate) fn interaction_blocked(
    store: &QivxifStore,
    actor_user_id: &UserId,
    target_user_id: &UserId,
) -> StoreResult<bool> {
    if actor_user_id == target_user_id {
        return Ok(false);
    }
    let tx = store.database.begin_read()?;
    let users = all_users(&tx)?;
    let Some(actor) = user_by_id(&users, actor_user_id) else {
        return Ok(true);
    };
    let Some(target) = user_by_id(&users, target_user_id) else {
        return Ok(true);
    };
    let actor_blocks = has_active_edge(
        &tx,
        &actor.profile_node_id,
        &target.profile_node_id,
        EdgeKind::Blocks,
    )?;
    let target_blocks = has_active_edge(
        &tx,
        &target.profile_node_id,
        &actor.profile_node_id,
        EdgeKind::Blocks,
    )?;
    Ok(actor_blocks || target_blocks)
}

fn has_active_edge(
    tx: &redb::ReadTransaction,
    from_node: &NodeId,
    to_node: &NodeId,
    kind: EdgeKind,
) -> StoreResult<bool> {
    let by_from = tx.open_table(tables::EDGES_BY_FROM)?;
    let edges = tx.open_table(tables::EDGES)?;
    let prefix = format!("{}:", from_node.as_str());
    for item in by_from.iter()? {
        let (key, edge_id) = item?;
        if !key.value().starts_with(&prefix) {
            continue;
        }
        let edge_id: qivxif_core::EdgeId = decode(edge_id.value())?;
        let Some(edge_bytes) = edges.get(edge_id.as_str())? else {
            continue;
        };
        let edge: EdgeRecord = decode(edge_bytes.value())?;
        if edge.kind == kind && &edge.to_node == to_node && edge.tombstone.is_none() {
            return Ok(true);
        }
    }
    Ok(false)
}

fn all_users(tx: &redb::ReadTransaction) -> StoreResult<Vec<StoredUser>> {
    let users = tx.open_table(tables::USERS)?;
    let mut out = Vec::new();
    for item in users.iter()? {
        let (_, bytes) = item?;
        out.push(decode(bytes.value())?);
    }
    Ok(out)
}

fn user_by_id<'a>(users: &'a [StoredUser], user_id: &UserId) -> Option<&'a StoredUser> {
    users.iter().find(|user| &user.id == user_id)
}
