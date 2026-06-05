# Media

## Purpose

Media is a first-class graph-backed subsystem for large files, previews,
attachments, local cache, and optional public serving.

## Contents

- [flows.md](flows.md): upload, import, preview, attach, publish, repair.
- [diagnostics.md](diagnostics.md): user-visible storage and transfer state.

## User Flows

- Upload a file by streaming chunks.
- Import a local file into browser storage.
- Preview images, audio, and video when supported.
- Attach media to graph nodes through real edges.
- Publish public or unlisted media when ACL allows it.
- Cache media locally for offline use.
- Delete or tombstone media according to ownership and retention rules.
- Repair missing chunks when another source is available.

## Durable Shape

A media asset is a graph node with metadata. Bytes live in content-addressed
chunks outside raw SQLite and redb value fields.
