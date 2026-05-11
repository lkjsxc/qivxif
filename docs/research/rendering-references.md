# Rendering References

## Scope

This file stores renderer research. It does not start client implementation.

## Findings

- Use `wgpu` as the renderer foundation.
- Use one renderer family with capability gates.
- Keep gameplay independent of experimental GPU features.
- Use `winit` platform shells when client work begins.
- Thin platform shells should share one Rust client core.
- Renderer work follows server authority, protocol behavior, and hot persistence.

## Current Status

Renderer implementation is deferred until the server slice is stable.
