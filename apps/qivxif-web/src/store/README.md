# IndexedDB Store

This directory owns structured browser-local persistence.

## Contents

- [indexed-db.ts](indexed-db.ts): small promise wrapper around object stores.

## Stores

- `events`: dirty, pending, accepted, or rejected local event records.
- `nodes` and `edges`: local projection cache records.
- `text_snapshots`: text content cache by node.
- `tab_snapshots`: pane-local drafts and scroll positions.
- `tile_layout`: local tile layout records and active resource pointers.

## Rules

- UI modules never write IndexedDB directly.
- Dirty events survive reload before server acceptance.
- Pane-local state is keyed by pane node ID, not resource node ID.
