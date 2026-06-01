# redb Layout

## File

The default server database file is `data/qivxif.redb`.

## Encoding

- Keys are UTF-8 bytes for string IDs or packed tuple bytes for secondary indexes.
- Values are bincode bytes encoded from the Rust owner type.
- Public JSON is decoded at API boundaries before store calls.
- Flexible metadata maps are validated before encoding.

## Table Contracts

| Table | Key | Value | Rust owner type |
| --- | --- | --- | --- |
| `meta` | `MetaKey` | `MetaRecord` | `qivxif_store_redb::meta::MetaRecord` |
| `users` | `UserId` | `StoredUser` | `qivxif_store_redb::auth::StoredUser` |
| `user_names` | login name | `UserId` | `qivxif_store_redb::auth::UserNameIndex` |
| `sessions` | `SessionId` | `StoredSession` | `qivxif_store_redb::auth::StoredSession` |
| `nodes` | `NodeId` | `NodeRecord` | `qivxif_graph::NodeRecord` |
| `edges` | `EdgeId` | `EdgeRecord` | `qivxif_graph::EdgeRecord` |
| `edges_by_from` | `(NodeId, EdgeId)` | empty marker | `qivxif_store_redb::graph::EdgeFromIndex` |
| `edges_by_to` | `(NodeId, EdgeId)` | empty marker | `qivxif_store_redb::graph::EdgeToIndex` |
| `events_by_id` | `EventId` | `EventEnvelope` | `qivxif_history::EventEnvelope` |
| `event_ids_by_actor` | `(ActorId, actor_seq)` | `EventId` | `qivxif_store_redb::history::ActorEventIndex` |
| `event_ids_by_parent` | `(EventId, EventId)` | empty marker | `qivxif_store_redb::history::EventParentIndex` |
| `event_ids_by_target_node` | `(NodeId, EventId)` | empty marker | `qivxif_store_redb::history::NodeEventIndex` |
| `event_ids_by_target_edge` | `(EdgeId, EventId)` | empty marker | `qivxif_store_redb::history::EdgeEventIndex` |
| `event_ids_by_target_event` | `(EventId, EventId)` | empty marker | `qivxif_store_redb::history::TargetEventIndex` |
| `commit_groups` | `CommitGroupId` | `CommitGroup` | `qivxif_history::CommitGroup` |
| `blobs` | `BlobHash` | `BlobManifest` | `qivxif_store_redb::blob::BlobManifest` |
| `blob_chunks` | `ChunkHash` | `BlobChunk` | `qivxif_store_redb::blob::BlobChunk` |
| `text_docs` | `TextDocId` | `TextDocState` | `qivxif_history::text::TextDocState` |
| `text_snapshots` | `(TextDocId, EventId)` | `TextSnapshot` | `qivxif_history::text::TextSnapshot` |
| `feed_items` | `EventId` | `FeedItem` | `qivxif_store_redb::feed::FeedItem` |
| `feed_items_by_user` | `(UserId, EventId)` | empty marker | `qivxif_store_redb::feed::FeedUserIndex` |
| `auth_tokens` | token hash | `AuthTokenRecord` | `qivxif_store_redb::auth::AuthTokenRecord` |
| `sync_cursors` | `CursorId` | `SyncCursorRecord` | `qivxif_store_redb::sync::SyncCursorRecord` |
| `server_jobs` | job id | `ServerJobRecord` | `qivxif_store_redb::job::ServerJobRecord` |

## Meta Records

- `schema_contract` stores the current table contract name.
- `created_at` stores the first open time.
- `last_opened_at` updates after a successful open.
