# Rendering

Owner doc for render architecture.

## Stack

- `wgpu` owns the surface and GPU device.
- `egui` owns chrome, menus, tabs, dialogs, and light controls.
- The custom qivxif tile engine owns pane rectangles and dock interactions.
- Editor text rendering uses a dedicated layout cache.
- Browser content is embedded or detached through the browser adapter.

## Rules

- Shape only visible editor lines plus overscan.
- Keep document revision separate from visual layout revision.
- Rebuild GPU resources on resize or device loss.
- Do not make pane layout depend on transient text measurement.
