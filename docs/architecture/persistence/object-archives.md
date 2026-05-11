# Object Archives

## Purpose

Object storage holds bulky or off-host artifacts.

## Canon

Use the `object_store` API for cold artifacts so local and hosted backends share
one async storage boundary.

## Artifacts

- Snapshots.
- Replay bundles.
- Crash bundles.
- Trace archives.
- Render goldens.
- Build and verification artifacts.

## Archive Encoding

- Use `rkyv` for read-mostly archive indexes, replay indexes, mesh caches, and
  far-field summaries when zero-copy reads matter.
- Freeze `rkyv` format options in canon before an artifact shape is accepted.
- Validate archived data before trusting it across a process or storage
  boundary.

## Rule

The hot database remains the local source of active truth.
