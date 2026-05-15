# Explorer Pane

Owner doc for visible explorer behavior.

## Content

- Shows workspace roots and local directory trees.
- Supports hidden-file toggle.
- Shows recent files and pinned roots.
- Opens files into editor panes.

## File Operations

- Create, rename, move, and delete require confirmation when destructive.
- Rename of an open file updates its buffer path.
- File watcher refresh happens in the background.
- Explorer never owns document content; buffers own text.
