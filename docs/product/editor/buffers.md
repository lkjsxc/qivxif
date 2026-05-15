# Buffers

Owner doc for visible buffer behavior.

## Buffer Kinds

| Kind | Behavior |
|---|---|
| File-backed | Reads and writes a local path. |
| Scratch | Lives in app state until saved or discarded. |
| Recovered | Restored from recovery state after an interrupted session. |

## Rules

- Every open document has a stable buffer identity.
- The dirty marker means buffer text differs from its last confirmed save.
- Multiple editor panes may show the same buffer.
- Each pane keeps independent scroll and cursor state.
- The buffer core stores text as Unicode and preserves detected line endings when saving.
