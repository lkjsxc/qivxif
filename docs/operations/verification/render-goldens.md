# Render Goldens

LLM purpose: reserve the renderer regression gate without redefining renderer
architecture.

## Purpose

Render goldens verify client output once rendering exists.

## Active First Gate

- [desktop-smoke.md](desktop-smoke.md) owns the first nonblank render artifact.
- The first renderer output is deterministic and may be CPU-produced.
- GPU goldens wait until `wgpu` code exists.

## Later Gate

Render checks must run in Compose and include a nonblank output assertion.
