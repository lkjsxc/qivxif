# Dependency Policy

Owner doc for dependency choices.

## Rules

- Prefer crates that directly support the documented architecture.
- Keep browser embedding behind a feature boundary.
- Avoid adding dependencies for behavior that the standard library covers cleanly.
- Document high-risk platform dependencies in operations docs.

## Selected Stack

- `winit`, `wgpu`, and `egui` for native shell and chrome.
- `egui_tiles` for docking layout.
- `cosmic-text` for text shaping and layout.
- `pulldown-cmark` for Markdown parsing.
- `wry` for browser embedding behind policy.
