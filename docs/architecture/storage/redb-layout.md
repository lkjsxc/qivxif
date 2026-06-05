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
| `event_ids_by_acceptance` | internal sequence | `EventId` | `qivxif_store_redb::history::AcceptanceIndex` |
| `commit_groups` | `CommitGroupId` | `CommitGroup` | `qivxif_history::CommitGroup` |
| `media_assets` | `MediaAssetId` | `MediaAssetRecord` | `qivxif_store_redb::media::MediaAssetRecord` |
| `media_variants` | `(MediaAssetId, VariantKind)` | `MediaVariantRecord` | `qivxif_store_redb::media::MediaVariantRecord` |
| `media_chunks` | `(UploadId, chunk_index)` | `MediaChunkRecord` | `qivxif_store_redb::media::MediaChunkRecord` |
| `media_uploads` | `UploadId` | `MediaUploadSession` | `qivxif_store_redb::media::MediaUploadSession` |
| `text_docs` | `TextDocId` | `TextDocState` | `qivxif_history::text::TextDocState` |
| `text_snapshots` | `(TextDocId, EventId)` | `TextSnapshot` | `qivxif_history::text::TextSnapshot` |
| `feed_items` | `EventId` | `FeedItem` | `qivxif_store_redb::feed::FeedItem` |
| `feed_items_by_user` | `(UserId, EventId)` | empty marker | `qivxif_store_redb::feed::FeedUserIndex` |
| `invite_codes` | invite id | `InviteCodeRecord` | `qivxif_store_redb::auth::InviteCodeRecord` |
| `access_keys` | key id | `AccessKeyRecord` | `qivxif_store_redb::auth::AccessKeyRecord` |
| `auth_tokens` | token hash | `AuthTokenRecord` | `qivxif_store_redb::auth::AuthTokenRecord` |
| `key_audit` | audit id | `KeyAuditRecord` | `qivxif_store_redb::auth::KeyAuditRecord` |
| `sync_cursors` | cursor mapping key | cursor or sequence | `qivxif_store_redb::sync::SyncCursorRecord` |
| `resource_entries` | resource id | `ResourceEntry` | `qivxif_store_redb::resource::ResourceEntry` |
| `server_jobs` | job id | `ServerJobRecord` | `qivxif_store_redb::job::ServerJobRecord` |

## Meta Records

- `schema_contract` stores the current table contract name.
- `created_at` stores the first open time.
- `last_opened_at` updates after a successful open.
