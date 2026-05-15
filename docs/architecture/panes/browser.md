# Browser Architecture

Owner doc for browser adapter internals.

## Boundary

- `BrowserController` is the only shell-facing browser control surface.
- The controller owns navigation, title updates, loading state, and policy prompts.
- The controller applies policy before selecting embedded, detached, or external routing.
- Platform embedding is hidden behind adapter modules and never bypasses policy.
- Fallback mode is a first-class controller state.

## Security

- Default deny for sensitive permissions.
- No arbitrary page-to-Rust bridge.
- Downloads are confirmed and path checked.
- External links can be forced to the system browser.
- A denied or unsupported embed path must fall back without crashing the shell.

## Platform Risk

Linux embedding may fail on some compositor or toolkit combinations. The product remains valid with detached or external fallback.
