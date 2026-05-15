# Accepted Decisions

Owner doc for active choices.

## Product

- qivxif is a Rust-native tile super app.
- Text editing, Markdown preview, explorer, and browser pane are the core surface.
- Docs are the durable source of truth.

## Architecture

- Use `winit`, `wgpu`, and `egui` for the native shell.
- Use the custom qivxif tile engine for splits, tabs, focus, and layout persistence.
- Use rope-backed buffers for text.
- Use `cosmic-text` for text layout and shaping.
- Use `pulldown-cmark` for Markdown parsing.
- Use `wry` only behind browser policy, controller, and fallback boundaries.
- Use TOML for settings and JSON for workspace state.

## Process

- Retired voxel and server contracts are deleted rather than carried as aliases.
