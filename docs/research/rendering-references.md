# Rendering References

## Findings

- Use `wgpu` as the renderer foundation.
- Use one renderer family with capability gates.
- Keep gameplay independent of experimental GPU features.
- Use `winit` platform shells when client work begins.

## Current Status

Renderer implementation is deferred until the server slice is stable.
