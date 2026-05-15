# Sessions

Owner doc for visible workspace restore behavior.

## Restored State

- Window size and maximized state.
- Tile tree and selected tabs.
- Open buffers and recent paths.
- Focused pane.
- Pane-local scroll state where meaningful.

## Rules

- Session state is machine-written JSON.
- Settings remain separate TOML.
- Missing files are shown as recoverable broken entries.
- Corrupt state falls back to a clean workspace and keeps the bad file for diagnostics.
