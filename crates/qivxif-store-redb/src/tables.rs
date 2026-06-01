use redb::TableDefinition;

pub const META: TableDefinition<&str, &[u8]> = TableDefinition::new("meta");
pub const USERS: TableDefinition<&str, &[u8]> = TableDefinition::new("users");
pub const USER_NAMES: TableDefinition<&str, &[u8]> = TableDefinition::new("user_names");
pub const SESSIONS: TableDefinition<&str, &[u8]> = TableDefinition::new("sessions");
pub const NODES: TableDefinition<&str, &[u8]> = TableDefinition::new("nodes");
pub const EDGES: TableDefinition<&str, &[u8]> = TableDefinition::new("edges");
pub const EDGES_BY_FROM: TableDefinition<&str, &[u8]> = TableDefinition::new("edges_by_from");
pub const EDGES_BY_TO: TableDefinition<&str, &[u8]> = TableDefinition::new("edges_by_to");
pub const OPS: TableDefinition<&str, &[u8]> = TableDefinition::new("ops");
pub const OPS_BY_ACTOR: TableDefinition<&str, &[u8]> = TableDefinition::new("ops_by_actor");
pub const OPS_BY_NODE: TableDefinition<&str, &[u8]> = TableDefinition::new("ops_by_node");
pub const COMMIT_GROUPS: TableDefinition<&str, &[u8]> = TableDefinition::new("commit_groups");
pub const BLOBS: TableDefinition<&str, &[u8]> = TableDefinition::new("blobs");
pub const BLOB_CHUNKS: TableDefinition<&str, &[u8]> = TableDefinition::new("blob_chunks");
pub const TEXT_DOCS: TableDefinition<&str, &[u8]> = TableDefinition::new("text_docs");
pub const TEXT_SNAPSHOTS: TableDefinition<&str, &[u8]> = TableDefinition::new("text_snapshots");
pub const FEED_ITEMS: TableDefinition<&str, &[u8]> = TableDefinition::new("feed_items");
pub const FEED_ITEMS_BY_USER: TableDefinition<&str, &[u8]> =
    TableDefinition::new("feed_items_by_user");
pub const AUTH_TOKENS: TableDefinition<&str, &[u8]> = TableDefinition::new("auth_tokens");
pub const SYNC_CURSORS: TableDefinition<&str, &[u8]> = TableDefinition::new("sync_cursors");
pub const SERVER_JOBS: TableDefinition<&str, &[u8]> = TableDefinition::new("server_jobs");

pub const ALL: &[(&str, TableDefinition<&str, &[u8]>)] = &[
    ("meta", META),
    ("users", USERS),
    ("user_names", USER_NAMES),
    ("sessions", SESSIONS),
    ("nodes", NODES),
    ("edges", EDGES),
    ("edges_by_from", EDGES_BY_FROM),
    ("edges_by_to", EDGES_BY_TO),
    ("ops", OPS),
    ("ops_by_actor", OPS_BY_ACTOR),
    ("ops_by_node", OPS_BY_NODE),
    ("commit_groups", COMMIT_GROUPS),
    ("blobs", BLOBS),
    ("blob_chunks", BLOB_CHUNKS),
    ("text_docs", TEXT_DOCS),
    ("text_snapshots", TEXT_SNAPSHOTS),
    ("feed_items", FEED_ITEMS),
    ("feed_items_by_user", FEED_ITEMS_BY_USER),
    ("auth_tokens", AUTH_TOKENS),
    ("sync_cursors", SYNC_CURSORS),
    ("server_jobs", SERVER_JOBS),
];
