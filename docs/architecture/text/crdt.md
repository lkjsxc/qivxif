# CRDT

## Contract

Text nodes use a CRDT event log for plain text.

The first text model is an ordered character sequence. Each inserted character has a stable id:

- `actor_id`
- `seq`

Each character stores:

- `id`
- `after`
- `value`
- `deleted`

`after` is null for root insertions or references another character id. Visible content is rendered by walking root children and child lists sorted by character id.

## Events

`text.insert` payload:

- `doc_id`
- `after`
- `chars`

Each inserted char contains `id` and `value`.

`text.delete` payload:

- `doc_id`
- `ids`

`text.restore` payload:

- `doc_id`
- `content`
- `actor_id`
- `first_seq`

Restore creates new inserted characters. It does not remove prior history.

## Rules

- Each editing session has a distinct actor identity.
- Local edits are durable before sync.
- Remote edits merge deterministically.
- Markdown is plain text plus preview projection.
- Duplicate text events do not change state twice.
- Inserts that reference missing anchors are rejected until the anchor arrives.
