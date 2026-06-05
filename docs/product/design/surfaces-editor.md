# Editor Surface

## Layout

```text
+------------------------------------------+
| title · node id · dirty and sync chips    |
+------------------------------------------+
| editor area                              |
| optional Markdown preview                |
+------------------------------------------+
| save state · counts · search controls     |
+------------------------------------------+
```

## Edit Region

- Class `.editor`; fills pane below chrome.
- Pane provides vertical scroll when the editor widget does not.
- Input maps to `saveTextDraft` and durable text events through controller.
- Dirty state shows on tab frame and status line.
- Cursor, selection, search, preview, and scroll are pane-local.

## Toolbar Row

- Save is primary when local text differs from accepted text.
- Secondary actions: create Graph Map, open sync, open history.
- Node ID uses `.mono` at `--text-xs`.

## Empty State

- No node selected: single line prompt plus actions to create or open a node.
- No invented document content.

## Tests

- Browser tests type, delete, undo, redo, paste multiline text, and save.
- Switching tabs retains draft through `tab_snapshots` when hidden stack is active.
- Markdown preview escapes raw HTML.
