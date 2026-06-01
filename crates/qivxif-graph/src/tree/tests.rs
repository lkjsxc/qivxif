use super::*;
use crate::{NodeKind, Tombstone};
use qivxif_core::{ActorId, MetadataMap, ServerTime, UserId, Visibility};

#[test]
fn projects_simple_tree() {
    let root = node();
    let child = node();
    let tree = project_tree(
        &root.id,
        vec![root.clone(), child.clone()],
        vec![edge(
            &root,
            &child,
            EdgeKind::Contains,
            MetadataMap::empty(),
        )],
    )
    .unwrap();
    assert_eq!(tree.root.node_id, root.id);
    assert_eq!(tree.root.children[0].node_id, child.id);
}

#[test]
fn orders_children_by_position_then_ordinal() {
    let root = node();
    let first = node();
    let second = node();
    let third = node();
    let tree = project_tree(
        &root.id,
        vec![root.clone(), first.clone(), second.clone(), third.clone()],
        vec![
            edge(&root, &third, EdgeKind::OrderedChild, meta("ordinal", "b")),
            edge(
                &root,
                &first,
                EdgeKind::OrderedChild,
                meta("position_key", "a"),
            ),
            edge(&root, &second, EdgeKind::OrderedChild, meta("ordinal", "a")),
        ],
    )
    .unwrap();
    let children: Vec<_> = tree
        .root
        .children
        .iter()
        .map(|child| &child.node_id)
        .collect();
    assert_eq!(children, vec![&first.id, &second.id, &third.id]);
}

#[test]
fn move_is_projected_from_active_relation_only() {
    let old_parent = node();
    let new_parent = node();
    let child = node();
    let mut old_edge = edge(
        &old_parent,
        &child,
        EdgeKind::ParentOf,
        MetadataMap::empty(),
    );
    old_edge.tombstone = Some(Tombstone {
        by: old_parent.created_by.clone(),
        reason: "moved".to_owned(),
    });
    let tree = project_tree(
        &new_parent.id,
        vec![old_parent.clone(), new_parent.clone(), child.clone()],
        vec![
            old_edge,
            edge(
                &new_parent,
                &child,
                EdgeKind::ParentOf,
                MetadataMap::empty(),
            ),
        ],
    )
    .unwrap();
    assert_eq!(tree.root.children[0].node_id, child.id);
}

#[test]
fn reports_cycle_and_duplicate_parent() {
    let left = node();
    let right = node();
    assert_eq!(
        project_tree(
            &left.id,
            vec![left.clone(), right.clone()],
            vec![
                edge(&left, &right, EdgeKind::ParentOf, MetadataMap::empty()),
                edge(&right, &left, EdgeKind::ParentOf, MetadataMap::empty()),
            ],
        )
        .unwrap_err(),
        GraphError::TreeCycle
    );
    let other = node();
    assert_eq!(
        project_tree(
            &left.id,
            vec![left.clone(), right.clone(), other.clone()],
            vec![
                edge(&left, &right, EdgeKind::ParentOf, MetadataMap::empty()),
                edge(&other, &right, EdgeKind::ParentOf, MetadataMap::empty()),
            ],
        )
        .unwrap_err(),
        GraphError::DuplicateActiveParent
    );
}

#[test]
fn reports_missing_child_and_ignores_tombstoned_relation() {
    let root = node();
    let child = node();
    let tombstone = Tombstone {
        by: root.created_by.clone(),
        reason: "removed".to_owned(),
    };
    assert_eq!(
        project_tree(
            &root.id,
            vec![root.clone()],
            vec![edge(
                &root,
                &child,
                EdgeKind::Contains,
                MetadataMap::empty()
            )],
        )
        .unwrap_err(),
        GraphError::NodeMissing
    );
    let mut removed = edge(&root, &child, EdgeKind::Contains, MetadataMap::empty());
    removed.tombstone = Some(tombstone);
    let root_id = root.id.clone();
    let tree = project_tree(&root_id, vec![root, child], vec![removed]).unwrap();
    assert!(tree.root.children.is_empty());
}

fn node() -> NodeRecord {
    let now = ServerTime::now();
    NodeRecord {
        id: NodeId::generate(),
        kind: NodeKind::Topic,
        owner_user_id: UserId::generate(),
        created_by: ActorId::generate(),
        created_at: now,
        updated_at: now,
        visibility: Visibility::Private,
        acl_ref: None,
        current_commit_group: None,
        current_text_ref: None,
        metadata_map: MetadataMap::empty(),
        tombstone: None,
    }
}

fn edge(
    from: &NodeRecord,
    to: &NodeRecord,
    kind: EdgeKind,
    metadata_map: MetadataMap,
) -> EdgeRecord {
    EdgeRecord {
        id: EdgeId::generate(),
        from_node: from.id.clone(),
        to_node: to.id.clone(),
        kind,
        created_by: from.created_by.clone(),
        created_at: ServerTime::now(),
        metadata_map,
        tombstone: None,
    }
}

fn meta(key: &str, value: &str) -> MetadataMap {
    let mut map = MetadataMap::empty();
    map.insert(key, value);
    map
}
