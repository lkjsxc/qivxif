# Purpose

qivxif is a Rust-first Web super app for personal and multi-user knowledge work.

## Purpose Statement

The product helps users create, link, edit, publish, arrange, and revisit information as a graph instead of a directory tree.

## Durable Shape

- A single server supports multiple users.
- A browser client works offline after first load.
- Durable data is typed KV plus typed graph records.
- Every durable mutation is an operation.
- The workspace is tiled, tabbed, and restorable.
- `kjxlkj` is the graph composition surface where the network is directly edited.

## Primary Proof Slice

Admin user -> login -> create graph node -> edit text offline -> persist locally -> sync to server -> load from another client -> inspect history.
