# Object Archives

## Status

- Status: implemented for manifest smoke operations only.
- Owner: `crates/qivxif-storage::ArchiveStore`.

## Implemented Boundary

- `ArchiveStore` wraps `Arc<dyn ObjectStore>`.
- `ArchiveStore::local(root)` uses `LocalFileSystem` with the given prefix.
- `put_manifest(name, bytes)` writes bytes to an object path.
- `get_manifest(name)` reads bytes from an object path.
- `list_manifests()` lists objects under `manifests`.

## Manifest Path Rules

- Path shape: `manifests/{name}.json`.
- Name must not be empty.
- Name must not start with `.`.
- Name must not contain `..`.
- Name bytes must be ASCII alphanumeric, `_`, `.`, or `-`.
- Invalid names return `StoreError::InvalidArchiveName`.

## Implemented Verification

- Tests write, read, and list a local manifest.
- Tests reject invalid manifest names.
- Local archive checks do not require cloud credentials.

## Not Implemented

- Snapshots.
- Replay bundles.
- Crash bundles.
- Trace archives.
- Hosted object storage configuration.
- Archive signatures.

## Rule

- The redb hot database remains the active source of world truth.
