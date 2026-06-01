# Panes

## Pane Kinds

- graph
- board
- text editor
- Markdown preview
- blog composer
- short post composer
- feed
- timeline
- search
- settings
- sync status
- history
- diff
- profile

## Rules

- Each pane has a stable pane ID.
- Pane state is serializable when it affects durable behavior.
- Components do not write directly to IndexedDB.
- Pane actions produce messages to tile layout, sync, editor, feed, or cache actors.
