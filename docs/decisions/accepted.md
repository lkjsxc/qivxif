# Accepted Decisions

Owner doc for active choices.

## Product

- qivxif is a Rust-native tile workspace.
- Text editing, Markdown preview, explorer, and browser pane are the core surface.
- Docs are the durable source of truth.

## Architecture

- Use `winit`, `wgpu`, and `egui` for the native shell.
- Use `egui_tiles` for tile layout.
- Use rope-backed buffers for text.
- Use `cosmic-text` for text layout and shaping.
- Use `pulldown-cmark` for Markdown parsing.
- Use `wry` behind a browser controller and policy boundary.
- Use TOML for settings and JSON for workspace state.

## Process

- Retired voxel and server contracts are deleted rather than carried as aliases.
