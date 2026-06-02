# Editor Surface

## Layout

```text
+------------------------------------------+
| node id (mono) · dirty chip              |
+------------------------------------------+
|                                          |
|  textarea fills remaining pane height    |
|                                          |
+------------------------------------------+
| [Save text event]  secondary actions     |
+------------------------------------------+
```

## Textarea

- Class `.editor`; `min-height` fills pane below chrome.
- `resize: none` inside tile shell; pane provides scroll.
- Input maps to `saveTextDraft` per pane through controller.
- Dirty state shows on tab frame (dot) and optional chip in pane.

## Toolbar Row

- Save is primary when `textDirty` is true.
- Secondary: create board, open sync — compact buttons, not a crowded menu.
- Node ID uses `.mono` at `--text-xs`.

## Empty State

- No node selected: single line prompt plus actions to create or open a node.
- No fake document content.

## Tests

- Playwright sets textarea value and saves; queue shows text event kind.
- Switching tabs retains draft via `tab_snapshots` when hidden stack is active.
