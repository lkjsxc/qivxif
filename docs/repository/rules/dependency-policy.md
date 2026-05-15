# Dependency Policy

Owner doc for dependency choices.

## Rules

- Prefer crates that directly support the documented architecture.
- Keep browser embedding behind policy, controller, fallback, and feature boundaries.
- Avoid adding dependencies for behavior that the standard library covers cleanly.
- Document high-risk platform dependencies in operations docs.

## Selected Stack

- `winit`, `wgpu`, and `egui` for native shell and chrome.
- Custom qivxif tile engine for docking layout and persistence.
- `cosmic-text` for text shaping and layout.
- `pulldown-cmark` for Markdown parsing.
- `wry` for optional browser embedding after policy accepts the navigation.
