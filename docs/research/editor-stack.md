# Editor Stack

Owner doc for editor stack findings.

## Selected Stack

- Rope-backed buffer for text storage.
- `cosmic-text` for shaping and layout.
- Transactional undo and redo.
- Visible-line cache for rendering.

## Reason

The editor must handle large local files, Unicode, selection, undo, and independent pane views without binding text ownership to UI widgets.
