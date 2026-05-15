# Event Loop

Owner doc for shell runtime shape.

## Contract

- `winit` owns the window event loop.
- UI state, tile layout, egui frame work, and pane focus live on the UI thread.
- Background tasks communicate with typed channels.
- The UI thread never blocks on filesystem scans, Markdown parsing, or browser work.

## Startup

1. Load settings.
2. Load workspace state or create default state.
3. Create window and renderer.
4. Attach panes to the tile host.
5. Enter redraw and input handling.

## Shutdown

- Flush settings and workspace state.
- Ask dirty buffers to save or keep recovery state.
- Cancel background tasks after state has been recorded.
