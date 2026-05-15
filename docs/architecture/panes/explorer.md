# Explorer Architecture

Owner doc for explorer internals.

## Model

- Roots are explicit paths.
- Tree nodes carry path, display name, expansion state, and lightweight metadata.
- Filesystem watching feeds refresh events into the model.
- File operations return workspace effects.

## Rules

- Path comparisons use canonical forms when available.
- Display labels preserve original paths.
- Slow scans run in background tasks.
- Errors are attached to nodes rather than panicking.
