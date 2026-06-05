use crate::{GraphError, GraphResult};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    Text,
    BlogPost,
    ShortPost,
    Profile,
    Tag,
    Topic,
    GraphMap,
    GraphMapItem,
    MediaAsset,
    TileLayout,
    Pane,
    FeedWindow,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    LinksTo,
    Contains,
    ParentOf,
    OrderedChild,
    References,
    ReferencesText,
    TaggedWith,
    AuthoredBy,
    ReplyTo,
    Mentions,
    Reposts,
    Bookmarks,
    Reacts,
    Follows,
    Mutes,
    Blocks,
    PlacedOnGraphMap,
    MediaAttachment,
    TileContainsPane,
    PaneViewsNode,
    Supersedes,
    Tombstones,
}

impl FromStr for NodeKind {
    type Err = GraphError;

    fn from_str(value: &str) -> GraphResult<Self> {
        serde_json::from_str(&format!("\"{value}\"")).map_err(|_| GraphError::UnknownNodeKind)
    }
}

impl FromStr for EdgeKind {
    type Err = GraphError;

    fn from_str(value: &str) -> GraphResult<Self> {
        serde_json::from_str(&format!("\"{value}\"")).map_err(|_| GraphError::UnknownEdgeKind)
    }
}
