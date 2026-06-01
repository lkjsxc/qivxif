use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    feed_audience::audience_users,
    feed_support::{
        ensure_reply_target, ensure_session_actor, feed_item, feed_order, feed_user_key,
        short_post_record, social_post_operation, validate_body,
    },
    moderation_query::feed_item_visible,
    operation_log::insert_operation,
    records::OperationReceipt,
    store::QivxifStore,
    tables,
};
use qivxif_auth::{AuthContext, can_read};
use qivxif_core::{ActorId, NodeId, OperationId, ServerTime, UserId, Visibility};
use qivxif_graph::NodeRecord;
use redb::ReadableTable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FeedItem {
    pub operation_id: OperationId,
    pub post_node_id: NodeId,
    pub author_user_id: UserId,
    pub author_name: String,
    pub body: String,
    pub visibility: Visibility,
    pub created_at: ServerTime,
    pub reply_to: Option<NodeId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortPostInput {
    pub op_id: OperationId,
    pub actor_seq: u64,
    pub node_id: NodeId,
    pub actor_id: ActorId,
    pub author_user_id: UserId,
    pub author_name: String,
    pub body: String,
    pub visibility: Visibility,
    pub reply_to: Option<NodeId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortPostResult {
    pub post: NodeRecord,
    pub feed_item: FeedItem,
    pub receipt: OperationReceipt,
}

impl QivxifStore {
    pub fn create_short_post(
        &self,
        auth: &AuthContext,
        input: ShortPostInput,
    ) -> StoreResult<ShortPostResult> {
        if self.get_operation(&input.op_id)?.is_some() {
            return self.replay_short_post(&input.op_id, &input.node_id);
        }
        ensure_session_actor(auth, &input.actor_id, &input.author_user_id)?;
        validate_body(&input.body)?;
        if let Some(reply_to) = &input.reply_to {
            ensure_reply_target(self, auth, reply_to)?;
        }
        let now = ServerTime::now();
        let post = short_post_record(&input, now);
        let feed_item = feed_item(&input, now);
        let op = social_post_operation(&input, &post)?;
        let tx = self.database.begin_write()?;
        let receipt = insert_operation(&tx, &op)?;
        {
            let mut nodes = tx.open_table(tables::NODES)?;
            if nodes.get(post.id.as_str())?.is_some() {
                return Err(StoreError::NodeExists);
            }
            nodes.insert(post.id.as_str(), encode(&post)?.as_slice())?;
            let mut items = tx.open_table(tables::FEED_ITEMS)?;
            items.insert(input.op_id.as_str(), encode(&feed_item)?.as_slice())?;
            let audience = audience_users(&tx, &input.author_user_id)?;
            let mut by_user = tx.open_table(tables::FEED_ITEMS_BY_USER)?;
            for user_id in audience {
                by_user.insert(
                    feed_user_key(&user_id, &input.op_id).as_str(),
                    ([] as [u8; 0]).as_slice(),
                )?;
            }
        }
        tx.commit()?;
        Ok(ShortPostResult {
            post,
            feed_item,
            receipt,
        })
    }

    pub fn home_feed(
        &self,
        auth: &AuthContext,
        limit: usize,
    ) -> StoreResult<(Vec<FeedItem>, Option<OperationId>, bool)> {
        let Some(user_id) = auth.user_id() else {
            return Err(StoreError::Forbidden);
        };
        let tx = self.database.begin_read()?;
        let index = tx.open_table(tables::FEED_ITEMS_BY_USER)?;
        let items = tx.open_table(tables::FEED_ITEMS)?;
        let nodes = tx.open_table(tables::NODES)?;
        let prefix = format!("{}:", user_id.as_str());
        let mut out = Vec::new();
        for item in index.iter()? {
            let (key, _) = item?;
            let Some(op_id_text) = key.value().strip_prefix(&prefix) else {
                continue;
            };
            if let Some(bytes) = items.get(op_id_text)? {
                let feed_item = decode::<FeedItem>(bytes.value())?;
                if let Some(node_bytes) = nodes.get(feed_item.post_node_id.as_str())? {
                    let node: NodeRecord = decode(node_bytes.value())?;
                    if can_read(auth, &node)
                        && feed_item_visible(&tx, user_id, &feed_item.author_user_id)?
                    {
                        out.push(feed_item);
                    }
                }
            }
        }
        out.sort_by(feed_order);
        let limit = limit.clamp(1, 100);
        let has_more = out.len() > limit;
        out.truncate(limit);
        let cursor = out.last().map(|item| item.operation_id.clone());
        Ok((out, cursor, has_more))
    }

    fn replay_short_post(
        &self,
        op_id: &OperationId,
        node_id: &NodeId,
    ) -> StoreResult<ShortPostResult> {
        let post = self
            .get_node(node_id)?
            .ok_or(StoreError::OperationConflict)?;
        let feed_item = self
            .get_feed_item(op_id)?
            .ok_or(StoreError::OperationConflict)?;
        let receipt = self
            .operation_receipt(op_id)?
            .ok_or(StoreError::OperationConflict)?;
        Ok(ShortPostResult {
            post,
            feed_item,
            receipt,
        })
    }

    fn get_feed_item(&self, op_id: &OperationId) -> StoreResult<Option<FeedItem>> {
        let tx = self.database.begin_read()?;
        let items = tx.open_table(tables::FEED_ITEMS)?;
        items
            .get(op_id.as_str())?
            .map(|bytes| decode(bytes.value()))
            .transpose()
    }
}
