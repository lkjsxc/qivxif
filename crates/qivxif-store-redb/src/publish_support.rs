use crate::{
    PublicBlogPost, PublishPostInput, StoreError, StoreResult, UnpublishPostInput,
    codec::{decode, encode},
    tables,
};
use qivxif_auth::{AuthContext, Viewer, can_publish};
use qivxif_core::{ActorId, EventId, NodeId, ServerTime, Visibility};
use qivxif_graph::{NodeKind, NodeRecord};
use qivxif_history::{
    EventEnvelope, EventKind, EventPayload, EventScope, hash_payload, text::TextDocState,
};
use redb::ReadableTable;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
struct PublishPayload {
    node_id: NodeId,
    slug: String,
    summary: String,
    author_name: String,
}

#[derive(Deserialize, Serialize)]
struct UnpublishPayload {
    node_id: NodeId,
    reason: String,
}

pub(crate) fn publishable_post(
    tx: &redb::WriteTransaction,
    auth: &AuthContext,
    node_id: &NodeId,
) -> StoreResult<NodeRecord> {
    let nodes = tx.open_table(tables::NODES)?;
    let Some(bytes) = nodes.get(node_id.as_str())? else {
        return Err(StoreError::NodeMissing);
    };
    let post: NodeRecord = decode(bytes.value())?;
    if post.kind != NodeKind::BlogPost || !can_publish(auth, &post) {
        return Err(StoreError::Forbidden);
    }
    Ok(post)
}

pub(crate) fn ensure_text_body(tx: &redb::WriteTransaction, body_id: &NodeId) -> StoreResult<()> {
    let nodes = tx.open_table(tables::NODES)?;
    let Some(bytes) = nodes.get(body_id.as_str())? else {
        return Err(StoreError::NodeMissing);
    };
    let body: NodeRecord = decode(bytes.value())?;
    if body.kind == NodeKind::Text {
        Ok(())
    } else {
        Err(StoreError::InvalidEvent)
    }
}

pub(crate) fn ensure_slug_free(
    tx: &redb::WriteTransaction,
    author_name: &str,
    slug: &str,
    post_node_id: &NodeId,
) -> StoreResult<()> {
    let nodes = tx.open_table(tables::NODES)?;
    for item in nodes.iter()? {
        let (_, bytes) = item?;
        let node: NodeRecord = decode(bytes.value())?;
        if node.id != *post_node_id && is_public_match(&node, author_name, slug) {
            return Err(StoreError::SlugConflict);
        }
    }
    Ok(())
}

pub(crate) fn public_blog_post(
    database: &Arc<redb::Database>,
    author_name: &str,
    slug: &str,
) -> StoreResult<Option<PublicBlogPost>> {
    let tx = database.begin_read()?;
    let nodes = tx.open_table(tables::NODES)?;
    let docs = tx.open_table(tables::TEXT_DOCS)?;
    for item in nodes.iter()? {
        let (_, bytes) = item?;
        let post: NodeRecord = decode(bytes.value())?;
        if !is_public_match(&post, author_name, slug) {
            continue;
        }
        let body_id = body_node_id(&post)?;
        let Some(text_bytes) = docs.get(body_id.as_str())? else {
            return Ok(None);
        };
        let state: TextDocState = decode(text_bytes.value())?;
        return Ok(Some(PublicBlogPost {
            title: post
                .metadata_map
                .get("title")
                .unwrap_or("Untitled")
                .to_owned(),
            author_name: author_name.to_owned(),
            slug: slug.to_owned(),
            summary: post.metadata_map.get("summary").unwrap_or("").to_owned(),
            body: state.content,
        }));
    }
    Ok(None)
}

pub(crate) fn body_node_id(post: &NodeRecord) -> StoreResult<NodeId> {
    post.metadata_map
        .get("body_node_id")
        .ok_or(StoreError::InvalidEvent)?
        .parse()
        .map_err(|_| StoreError::InvalidEvent)
}

pub(crate) fn write_post(tx: &redb::WriteTransaction, post: &NodeRecord) -> StoreResult<()> {
    let mut nodes = tx.open_table(tables::NODES)?;
    nodes.insert(post.id.as_str(), encode(post)?.as_slice())?;
    Ok(())
}

pub(crate) fn publish_envelope(input: &PublishPostInput) -> StoreResult<EventEnvelope> {
    let payload = PublishPayload {
        node_id: input.post_node_id.clone(),
        slug: input.slug.clone(),
        summary: input.summary.clone(),
        author_name: input.author_name.clone(),
    };
    envelope(
        &input.event_id,
        &input.actor_id,
        input.actor_seq,
        EventKind::PublishPost,
        &input.post_node_id,
        &payload,
    )
}

pub(crate) fn unpublish_envelope(input: &UnpublishPostInput) -> StoreResult<EventEnvelope> {
    let payload = UnpublishPayload {
        node_id: input.post_node_id.clone(),
        reason: input.reason.clone(),
    };
    envelope(
        &input.event_id,
        &input.actor_id,
        input.actor_seq,
        EventKind::PublishUnpublish,
        &input.post_node_id,
        &payload,
    )
}

pub(crate) fn actor_matches(auth: &AuthContext, actor_id: &ActorId) -> bool {
    matches!(
        &auth.viewer,
        Viewer::Session { actor_id: auth_actor, .. } if auth_actor == actor_id
    )
}

fn is_public_match(node: &NodeRecord, author_name: &str, slug: &str) -> bool {
    node.kind == NodeKind::BlogPost
        && node.visibility == Visibility::Public
        && node.metadata_map.get("publication_state") == Some("published")
        && node.metadata_map.get("author_name") == Some(author_name)
        && node.metadata_map.get("slug") == Some(slug)
}

fn envelope<T: Serialize>(
    event_id: &EventId,
    actor_id: &ActorId,
    actor_seq: u64,
    kind: EventKind,
    node_id: &NodeId,
    payload: &T,
) -> StoreResult<EventEnvelope> {
    let bytes = serde_json::to_vec(payload).map_err(|_| StoreError::InvalidEvent)?;
    Ok(EventEnvelope {
        event_id: event_id.clone(),
        actor_id: actor_id.clone(),
        actor_seq,
        parents: Vec::new(),
        scope: EventScope::Publish,
        kind,
        target_node_ids: vec![node_id.clone()],
        target_edge_ids: Vec::new(),
        target_event_ids: Vec::new(),
        payload: EventPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: Some(ServerTime::now()),
        auth_context: Some(node_id.to_string()),
    })
}
