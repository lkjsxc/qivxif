# Desktop Stack

Owner doc for desktop stack findings.

## Selected Stack

- `winit` for window and event loop.
- `wgpu` for graphics surface.
- `egui` for shell chrome and controls.
- `egui_tiles` for docking layout.

## Reason

This stack keeps the app native Rust, gives fast UI iteration, and supports custom editor rendering without making the whole product a web app.
