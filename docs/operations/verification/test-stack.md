# Test Stack

Owner doc for test tools.

## Layers

| Layer | Scope |
|---|---|
| Unit tests | Buffer edits, layout tree, policy logic. |
| Property tests | Undo round trips and layout invariants. |
| Snapshot tests | Markdown render models and workspace JSON. |
| Integration tests | File IO, recovery, session restore. |
| Smoke tests | Desktop startup and minimal workflows. |

## Rule

Tests should verify behavior described in owner docs, not historical behavior.
