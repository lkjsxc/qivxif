use crate::{FollowInput, ShortPostInput, StoreConfig, StoreResult, open_or_create, tables};
use qivxif_auth::{AuthContext, AuthRole, Viewer, hash_password};
use qivxif_core::{EdgeId, NodeId, OperationId, SessionId, Visibility};
use redb::ReadableTable;
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn repair_check_reports_missing_edge_index() -> StoreResult<()> {
    let (store, admin, member) = seeded_store("repair-edge")?;
    let result = store.follow_profile(
        &auth(&admin),
        FollowInput {
            op_id: OperationId::generate(),
            actor_seq: 1,
            edge_id: EdgeId::generate(),
            actor_id: admin.actor_id.clone(),
            follower_user_id: admin.id.clone(),
            follower_profile_node_id: admin.profile_node_id.clone(),
            target_profile_node_id: member.profile_node_id,
        },
    )?;
    let tx = store.database.begin_write()?;
    {
        let mut by_from = tx.open_table(tables::EDGES_BY_FROM)?;
        by_from.remove(
            format!(
                "{}:{}",
                admin.profile_node_id.as_str(),
                result.edge.id.as_str()
            )
            .as_str(),
        )?;
    }
    tx.commit()?;
    let report = store.repair_check()?;
    assert!(!report.ok);
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.code == "edge_from_index_missing")
    );
    Ok(())
}

#[test]
fn rebuild_feed_indexes_recreates_follow_audience() -> StoreResult<()> {
    let (store, admin, member) = seeded_store("feed-rebuild")?;
    store.follow_profile(
        &auth(&admin),
        FollowInput {
            op_id: OperationId::generate(),
            actor_seq: 1,
            edge_id: EdgeId::generate(),
            actor_id: admin.actor_id.clone(),
            follower_user_id: admin.id.clone(),
            follower_profile_node_id: admin.profile_node_id,
            target_profile_node_id: member.profile_node_id.clone(),
        },
    )?;
    store.create_short_post(
        &auth(&member),
        ShortPostInput {
            op_id: OperationId::generate(),
            actor_seq: 1,
            node_id: NodeId::generate(),
            actor_id: member.actor_id.clone(),
            author_user_id: member.id.clone(),
            author_name: member.name.clone(),
            body: "repairable feed".to_owned(),
            visibility: Visibility::Public,
            reply_to: None,
        },
    )?;
    insert_stale_marker(&store)?;
    let report = store.rebuild_feed_indexes()?;
    assert_eq!(report.items, 1);
    assert_eq!(report.markers, 2);
    assert_eq!(feed_marker_count(&store)?, 2);
    Ok(())
}

fn seeded_store(
    name: &str,
) -> StoreResult<(crate::QivxifStore, crate::StoredUser, crate::StoredUser)> {
    let store = open_or_create(StoreConfig::new(test_path(name)))?;
    let admin = store.create_admin_user("admin".to_owned(), hash_password("secret").unwrap())?;
    let member = store.create_user(
        "member".to_owned(),
        hash_password("member-secret").unwrap(),
        vec![AuthRole::Member],
    )?;
    Ok((store, admin, member))
}

fn auth(user: &crate::StoredUser) -> AuthContext {
    AuthContext {
        viewer: Viewer::Session {
            user_id: user.id.clone(),
            actor_id: user.actor_id.clone(),
            session_id: SessionId::generate(),
        },
        roles: user.roles.clone(),
    }
}

fn insert_stale_marker(store: &crate::QivxifStore) -> StoreResult<()> {
    let tx = store.database.begin_write()?;
    {
        let mut by_user = tx.open_table(tables::FEED_ITEMS_BY_USER)?;
        by_user.insert("usr_stale:op_stale", ([] as [u8; 0]).as_slice())?;
    }
    tx.commit()?;
    Ok(())
}

fn feed_marker_count(store: &crate::QivxifStore) -> StoreResult<u64> {
    let tx = store.database.begin_read()?;
    let table = tx.open_table(tables::FEED_ITEMS_BY_USER)?;
    let mut count = 0;
    for item in table.iter()? {
        item?;
        count += 1;
    }
    Ok(count)
}

fn test_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("qivxif-{name}-{}-{nanos}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path.join("qivxif.redb")
}
