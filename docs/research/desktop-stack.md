# Desktop Stack

Owner doc for desktop stack findings.

## Selected Stack

- `winit` for window and event loop.
- `wgpu` for graphics surface.
- `egui` for shell chrome and controls.
- Custom qivxif tile engine for docking layout.

## Reason

This stack keeps the app native Rust, gives fast UI iteration, and keeps tile behavior owned by qivxif instead of a docking crate.
