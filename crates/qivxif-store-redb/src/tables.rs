use redb::TableDefinition;

pub const META: TableDefinition<&str, &[u8]> = TableDefinition::new("meta");
pub const USERS: TableDefinition<&str, &[u8]> = TableDefinition::new("users");
pub const USER_NAMES: TableDefinition<&str, &[u8]> = TableDefinition::new("user_names");
pub const SESSIONS: TableDefinition<&str, &[u8]> = TableDefinition::new("sessions");
pub const NODES: TableDefinition<&str, &[u8]> = TableDefinition::new("nodes");
pub const EDGES: TableDefinition<&str, &[u8]> = TableDefinition::new("edges");
pub const EDGES_BY_FROM: TableDefinition<&str, &[u8]> = TableDefinition::new("edges_by_from");
pub const EDGES_BY_TO: TableDefinition<&str, &[u8]> = TableDefinition::new("edges_by_to");
pub const EVENTS_BY_ID: TableDefinition<&str, &[u8]> = TableDefinition::new("events_by_id");
pub const EVENT_IDS_BY_ACTOR: TableDefinition<&str, &[u8]> =
    TableDefinition::new("event_ids_by_actor");
pub const EVENT_IDS_BY_PARENT: TableDefinition<&str, &[u8]> =
    TableDefinition::new("event_ids_by_parent");
pub const EVENT_IDS_BY_TARGET_NODE: TableDefinition<&str, &[u8]> =
    TableDefinition::new("event_ids_by_target_node");
pub const EVENT_IDS_BY_TARGET_EDGE: TableDefinition<&str, &[u8]> =
    TableDefinition::new("event_ids_by_target_edge");
pub const EVENT_IDS_BY_TARGET_EVENT: TableDefinition<&str, &[u8]> =
    TableDefinition::new("event_ids_by_target_event");
pub const EVENT_IDS_BY_ACCEPTANCE: TableDefinition<&str, &[u8]> =
    TableDefinition::new("event_ids_by_acceptance");
pub const COMMIT_GROUPS: TableDefinition<&str, &[u8]> = TableDefinition::new("commit_groups");
pub const MEDIA_ASSETS: TableDefinition<&str, &[u8]> = TableDefinition::new("media_assets");
pub const MEDIA_VARIANTS: TableDefinition<&str, &[u8]> = TableDefinition::new("media_variants");
pub const MEDIA_CHUNKS: TableDefinition<&str, &[u8]> = TableDefinition::new("media_chunks");
pub const MEDIA_UPLOADS: TableDefinition<&str, &[u8]> = TableDefinition::new("media_uploads");
pub const TEXT_DOCS: TableDefinition<&str, &[u8]> = TableDefinition::new("text_docs");
pub const TEXT_SNAPSHOTS: TableDefinition<&str, &[u8]> = TableDefinition::new("text_snapshots");
pub const FEED_ITEMS: TableDefinition<&str, &[u8]> = TableDefinition::new("feed_items");
pub const FEED_ITEMS_BY_USER: TableDefinition<&str, &[u8]> =
    TableDefinition::new("feed_items_by_user");
pub const INVITE_CODES: TableDefinition<&str, &[u8]> = TableDefinition::new("invite_codes");
pub const ACCESS_KEYS: TableDefinition<&str, &[u8]> = TableDefinition::new("access_keys");
pub const AUTH_TOKENS: TableDefinition<&str, &[u8]> = TableDefinition::new("auth_tokens");
pub const KEY_AUDIT: TableDefinition<&str, &[u8]> = TableDefinition::new("key_audit");
pub const SYNC_CURSORS: TableDefinition<&str, &[u8]> = TableDefinition::new("sync_cursors");
pub const RESOURCE_ENTRIES: TableDefinition<&str, &[u8]> = TableDefinition::new("resource_entries");
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
    ("events_by_id", EVENTS_BY_ID),
    ("event_ids_by_actor", EVENT_IDS_BY_ACTOR),
    ("event_ids_by_parent", EVENT_IDS_BY_PARENT),
    ("event_ids_by_target_node", EVENT_IDS_BY_TARGET_NODE),
    ("event_ids_by_target_edge", EVENT_IDS_BY_TARGET_EDGE),
    ("event_ids_by_target_event", EVENT_IDS_BY_TARGET_EVENT),
    ("event_ids_by_acceptance", EVENT_IDS_BY_ACCEPTANCE),
    ("commit_groups", COMMIT_GROUPS),
    ("media_assets", MEDIA_ASSETS),
    ("media_variants", MEDIA_VARIANTS),
    ("media_chunks", MEDIA_CHUNKS),
    ("media_uploads", MEDIA_UPLOADS),
    ("text_docs", TEXT_DOCS),
    ("text_snapshots", TEXT_SNAPSHOTS),
    ("feed_items", FEED_ITEMS),
    ("feed_items_by_user", FEED_ITEMS_BY_USER),
    ("invite_codes", INVITE_CODES),
    ("access_keys", ACCESS_KEYS),
    ("auth_tokens", AUTH_TOKENS),
    ("key_audit", KEY_AUDIT),
    ("sync_cursors", SYNC_CURSORS),
    ("resource_entries", RESOURCE_ENTRIES),
    ("server_jobs", SERVER_JOBS),
];
