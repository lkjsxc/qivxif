# Purpose

qivxif is a browser-first Svelte workspace for personal and multi-user knowledge
work.

## Purpose Statement

The product helps users create, link, edit, publish, arrange, and revisit
information as a graph instead of a directory tree.

## Durable Shape

- A single optional server supports multiple users.
- A browser client works offline after first load.
- Durable data is typed KV plus typed graph records.
- Every durable mutation is an event.
- The tile shell is tabbed, split-capable, and restorable.
- Graph Map lets users arrange, inspect, and edit relationships directly.
- Resource orchestration protects dirty work and manages expensive tasks.

## Primary Proof Slice

Setup owner -> authenticated session -> create graph node -> edit text offline ->
persist locally -> sync events to server -> load from another client -> inspect
history.
