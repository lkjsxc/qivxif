# Text Nodes

## Behavior

- Open a text node in an editor pane.
- Edit while offline.
- Persist local edits before network delivery.
- Undo and redo local edit groups.
- Merge accepted remote edits.
- Show save and sync state.
- Restore after refresh.
- Map text edits into ordered character-id events instead of byte offsets.
- Keep cursor, selection, scroll, search, preview, and status state pane-local.
- Never report queued status before local durable persistence succeeds.

## Constraint

The editor UI may use a textarea or CodeMirror only if edits map into durable
text events.
