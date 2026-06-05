# Media Storage

## Rule

Large blobs do not live as raw blob fields in SQLite or redb. Metadata lives in
records. Bytes live in content-addressed chunks.

## Media Asset Node

`media_asset` metadata stores:

- owner.
- content hash.
- size.
- MIME type.
- original filename.
- created time.
- visibility.
- ACL ref.
- processing state.
- metadata map.

## Media Variant Record

A media asset may have variants:

- original.
- thumbnail.
- preview.
- transcoded.

Each record stores hash, size, dimensions or duration when known, and storage
locator.

## Media Chunk Record

A chunk record stores:

- upload id.
- chunk index.
- hash.
- size.
- committed flag.

## Browser Storage

- SQLite stores metadata and queue state only.
- OPFS stores chunks, originals, thumbnails, and previews.
- Upload sessions survive refresh.
- Large files stream by chunk.
- The resource orchestrator manages retention.

## Server Storage

- redb stores metadata, ACL, upload sessions, and indexes.
- A blob directory or object-store-like path stores bytes by content hash.
- Upload chunks write to staging paths.
- Completion verifies hashes and atomically commits locators.
- Private bytes never bypass ACL checks.
