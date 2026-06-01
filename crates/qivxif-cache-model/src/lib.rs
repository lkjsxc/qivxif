use qivxif_core::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CacheEntry {
    pub key: String,
    pub kind: CacheEntryKind,
    pub bytes: u64,
    pub pinned: bool,
    pub dirty: bool,
    pub local_only_node: Option<NodeId>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheEntryKind {
    MediaPreview,
    FeedWindow,
    GraphNeighborProjection,
    TextSnapshot,
    PublicNodeBody,
    DirtyOperation,
    LocalOnlyNode,
    CurrentTileLayout,
    CacheManifest,
    EvictionJournal,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CacheManifest {
    pub entries: Vec<CacheEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CacheBudget {
    pub usage_bytes: u64,
    pub target_bytes: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CachePlan {
    pub actions: Vec<CachePlanAction>,
    pub journal: Vec<CacheJournalEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CachePlanAction {
    Evict { key: String, bytes: u64 },
    Keep { key: String, reason: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CacheJournalEntry {
    pub key: String,
    pub before: String,
    pub after: String,
}

pub fn plan_cache_mutations(manifest: &CacheManifest, budget: CacheBudget) -> CachePlan {
    let mut usage = budget.usage_bytes;
    let mut actions = Vec::new();
    let mut candidates: Vec<&CacheEntry> = manifest.entries.iter().collect();
    candidates.sort_by_key(|entry| (eviction_rank(entry.kind), entry.key.clone()));
    for entry in candidates {
        if protected(entry) {
            actions.push(CachePlanAction::Keep {
                key: entry.key.clone(),
                reason: "protected".to_owned(),
            });
            continue;
        }
        if usage > budget.target_bytes {
            usage = usage.saturating_sub(entry.bytes);
            actions.push(CachePlanAction::Evict {
                key: entry.key.clone(),
                bytes: entry.bytes,
            });
        } else {
            actions.push(CachePlanAction::Keep {
                key: entry.key.clone(),
                reason: "within_budget".to_owned(),
            });
        }
    }
    let journal = actions
        .iter()
        .map(|action| match action {
            CachePlanAction::Evict { key, .. } => CacheJournalEntry {
                key: key.clone(),
                before: "present".to_owned(),
                after: "evict".to_owned(),
            },
            CachePlanAction::Keep { key, reason } => CacheJournalEntry {
                key: key.clone(),
                before: "present".to_owned(),
                after: reason.clone(),
            },
        })
        .collect();
    CachePlan { actions, journal }
}

fn protected(entry: &CacheEntry) -> bool {
    entry.pinned
        || entry.dirty
        || entry.local_only_node.is_some()
        || matches!(
            entry.kind,
            CacheEntryKind::DirtyOperation
                | CacheEntryKind::LocalOnlyNode
                | CacheEntryKind::CurrentTileLayout
                | CacheEntryKind::CacheManifest
                | CacheEntryKind::EvictionJournal
        )
}

fn eviction_rank(kind: CacheEntryKind) -> u8 {
    match kind {
        CacheEntryKind::MediaPreview => 0,
        CacheEntryKind::FeedWindow => 1,
        CacheEntryKind::GraphNeighborProjection => 2,
        CacheEntryKind::TextSnapshot => 3,
        CacheEntryKind::PublicNodeBody => 4,
        _ => 9,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(key: &str, kind: CacheEntryKind) -> CacheEntry {
        CacheEntry {
            key: key.to_owned(),
            kind,
            bytes: 10,
            pinned: false,
            dirty: false,
            local_only_node: None,
        }
    }

    #[test]
    fn evicts_media_before_snapshot() {
        let manifest = CacheManifest {
            entries: vec![
                entry("snapshot", CacheEntryKind::TextSnapshot),
                entry("media", CacheEntryKind::MediaPreview),
            ],
        };
        let plan = plan_cache_mutations(
            &manifest,
            CacheBudget {
                usage_bytes: 20,
                target_bytes: 10,
            },
        );
        assert!(
            matches!(plan.actions[0], CachePlanAction::Evict { ref key, .. } if key == "media")
        );
    }
}
