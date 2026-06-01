use crate::{
    StoreError, StoreResult,
    operation_log::insert_operation,
    publish_support::{
        actor_matches, body_node_id, ensure_slug_free, ensure_text_body, public_blog_post,
        publish_envelope, publishable_post, unpublish_envelope, write_post,
    },
    records::OperationReceipt,
    store::QivxifStore,
};
use qivxif_auth::AuthContext;
use qivxif_core::{ActorId, NodeId, OperationId, ServerTime, Visibility};
use qivxif_graph::NodeRecord;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishPostInput {
    pub op_id: OperationId,
    pub actor_seq: u64,
    pub actor_id: ActorId,
    pub post_node_id: NodeId,
    pub author_name: String,
    pub slug: String,
    pub summary: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnpublishPostInput {
    pub op_id: OperationId,
    pub actor_seq: u64,
    pub actor_id: ActorId,
    pub post_node_id: NodeId,
    pub reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishPostResult {
    pub post: NodeRecord,
    pub receipt: OperationReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicBlogPost {
    pub title: String,
    pub author_name: String,
    pub slug: String,
    pub summary: String,
    pub body: String,
}

impl QivxifStore {
    pub fn publish_post(
        &self,
        auth: &AuthContext,
        input: PublishPostInput,
    ) -> StoreResult<PublishPostResult> {
        if self.get_operation(&input.op_id)?.is_some() {
            return self.replay_publish(&input.op_id, &input.post_node_id);
        }
        if !actor_matches(auth, &input.actor_id) {
            return Err(StoreError::Forbidden);
        }
        let op = publish_envelope(&input)?;
        let tx = self.database.begin_write()?;
        let mut post = publishable_post(&tx, auth, &input.post_node_id)?;
        let body_id = body_node_id(&post)?;
        ensure_text_body(&tx, &body_id)?;
        ensure_slug_free(&tx, &input.author_name, &input.slug, &input.post_node_id)?;
        let receipt = insert_operation(&tx, &op)?;
        post.visibility = Visibility::Public;
        post.updated_at = ServerTime::now();
        post.metadata_map.insert("slug", input.slug);
        post.metadata_map.insert("summary", input.summary);
        post.metadata_map.insert("author_name", input.author_name);
        post.metadata_map.insert("publication_state", "published");
        post.metadata_map
            .insert("published_at", post.updated_at.to_string());
        write_post(&tx, &post)?;
        tx.commit()?;
        Ok(PublishPostResult { post, receipt })
    }

    pub fn unpublish_post(
        &self,
        auth: &AuthContext,
        input: UnpublishPostInput,
    ) -> StoreResult<PublishPostResult> {
        if self.get_operation(&input.op_id)?.is_some() {
            return self.replay_publish(&input.op_id, &input.post_node_id);
        }
        if !actor_matches(auth, &input.actor_id) {
            return Err(StoreError::Forbidden);
        }
        let op = unpublish_envelope(&input)?;
        let tx = self.database.begin_write()?;
        let mut post = publishable_post(&tx, auth, &input.post_node_id)?;
        let receipt = insert_operation(&tx, &op)?;
        post.visibility = Visibility::Private;
        post.updated_at = ServerTime::now();
        post.metadata_map.insert("publication_state", "unpublished");
        post.metadata_map.insert("unpublish_reason", input.reason);
        write_post(&tx, &post)?;
        tx.commit()?;
        Ok(PublishPostResult { post, receipt })
    }

    pub fn public_blog_post(
        &self,
        author_name: &str,
        slug: &str,
    ) -> StoreResult<Option<PublicBlogPost>> {
        public_blog_post(&self.database, author_name, slug)
    }

    fn replay_publish(
        &self,
        op_id: &OperationId,
        post_node_id: &NodeId,
    ) -> StoreResult<PublishPostResult> {
        let post = self
            .get_node(post_node_id)?
            .ok_or(StoreError::OperationConflict)?;
        let receipt = self
            .operation_receipt(op_id)?
            .ok_or(StoreError::OperationConflict)?;
        Ok(PublishPostResult { post, receipt })
    }
}
