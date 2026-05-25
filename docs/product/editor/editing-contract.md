# Editing Contract

Editor behavior is defined by buffers plus runtime editor state.

## Buffer State

- Scratch buffers have no path and save only after a path is assigned.
- File buffers have an absolute or command-provided path.
- Recovered buffers keep the original path when known and show recovery status.
- Dirty state is true when text revision is newer than the last saved revision.
- Large files above the configured limit open read-only with a visible diagnostic.

## History

- Every UI edit flows through `EditorCommand`.
- Undo and redo operate per buffer.
- Save clears dirty state and clears recovery for that buffer.
- Failed saves keep dirty state and expose the error.

## Recovery

- Dirty buffers emit recovery JSON records with buffer id, path, label, revision, and text.
- Startup loads recovery records before opening new scratch buffers.
- Recovery records are cleared only after successful save or explicit discard.

## Rendering

- The editor view renders the buffer text and must not mutate text directly.
- Cursor and selection state remain view state, not persistence state.
- Markdown preview observes source revision changes.
