# CRDT

## Contract

Text nodes use a CRDT operation log for plain text.

## Rules

- Each editing session has a distinct actor identity.
- Local edits are durable before sync.
- Remote edits merge deterministically.
- Markdown is plain text plus preview projection.
