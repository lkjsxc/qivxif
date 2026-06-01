use crate::{FeedItem, ShortPostInput, StoreError, StoreResult, codec::encode, store::QivxifStore};
use qivxif_auth::{AuthContext, Viewer, can_read};
use qivxif_core::{MetadataMap, NodeId, OperationId, ServerTime, UserId};
use qivxif_graph::{NodeKind, NodeRecord};
use qivxif_history::{
    OperationEnvelope, OperationKind, OperationPayload, OperationScope, hash_payload,
};

pub(crate) fn ensure_session_actor(
    auth: &AuthContext,
    actor: &qivxif_core::ActorId,
    user: &UserId,
) -> StoreResult<()> {
    match &auth.viewer {
        Viewer::Session {
            actor_id, user_id, ..
        } if actor_id == actor && user_id == user => Ok(()),
        _ => Err(StoreError::Forbidden),
    }
}

pub(crate) fn validate_body(body: &str) -> StoreResult<()> {
    let count = body.chars().count();
    if count == 0 || count > 512 {
        Err(StoreError::InvalidOperation)
    } else {
        Ok(())
    }
}

pub(crate) fn ensure_reply_target(
    store: &QivxifStore,
    auth: &AuthContext,
    reply_to: &NodeId,
) -> StoreResult<()> {
    let Some(node) = store.get_node(reply_to)? else {
        return Err(StoreError::NodeMissing);
    };
    if node.kind == NodeKind::ShortPost && can_read(auth, &node) {
        Ok(())
    } else {
        Err(StoreError::Forbidden)
    }
}

pub(crate) fn short_post_record(input: &ShortPostInput, now: ServerTime) -> NodeRecord {
    let mut metadata = MetadataMap::empty();
    metadata.insert("body", input.body.clone());
    metadata.insert("author_name", input.author_name.clone());
    metadata.insert("social_state", "posted");
    metadata.insert("posted_at", now.to_string());
    if let Some(reply_to) = &input.reply_to {
        metadata.insert("reply_to", reply_to.to_string());
    }
    NodeRecord {
        id: input.node_id.clone(),
        kind: NodeKind::ShortPost,
        owner_user_id: input.author_user_id.clone(),
        created_by: input.actor_id.clone(),
        created_at: now,
        updated_at: now,
        visibility: input.visibility,
        acl_ref: None,
        current_commit_group: None,
        current_text_ref: None,
        metadata_map: metadata,
        tombstone: None,
    }
}

pub(crate) fn feed_item(input: &ShortPostInput, now: ServerTime) -> FeedItem {
    FeedItem {
        operation_id: input.op_id.clone(),
        post_node_id: input.node_id.clone(),
        author_user_id: input.author_user_id.clone(),
        author_name: input.author_name.clone(),
        body: input.body.clone(),
        visibility: input.visibility,
        created_at: now,
        reply_to: input.reply_to.clone(),
    }
}

pub(crate) fn social_post_operation(
    input: &ShortPostInput,
    post: &NodeRecord,
) -> StoreResult<OperationEnvelope> {
    let bytes = encode(post)?;
    Ok(OperationEnvelope {
        op_id: input.op_id.clone(),
        actor_id: input.actor_id.clone(),
        actor_seq: input.actor_seq,
        parents: Vec::new(),
        scope: OperationScope::Social,
        kind: OperationKind::SocialShortPostCreate,
        target_node_ids: target_nodes(&input.node_id, input.reply_to.as_ref()),
        payload: OperationPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: Some(ServerTime::now()),
        auth_context: Some(input.author_user_id.to_string()),
    })
}

pub(crate) fn feed_user_key(user_id: &UserId, op_id: &OperationId) -> String {
    format!("{}:{}", user_id.as_str(), op_id.as_str())
}

pub(crate) fn feed_order(left: &FeedItem, right: &FeedItem) -> std::cmp::Ordering {
    right
        .created_at
        .to_string()
        .cmp(&left.created_at.to_string())
        .then_with(|| right.operation_id.as_str().cmp(left.operation_id.as_str()))
}

fn target_nodes(node_id: &NodeId, reply_to: Option<&NodeId>) -> Vec<NodeId> {
    let mut targets = vec![node_id.clone()];
    if let Some(reply_to) = reply_to {
        targets.push(reply_to.clone());
    }
    targets
}
