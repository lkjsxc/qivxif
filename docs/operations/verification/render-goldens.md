# Render Goldens

LLM purpose: reserve the renderer regression gate without redefining renderer
architecture.

## Purpose

Render goldens verify client output once rendering exists.

## Deferred Scope

The initial server slice does not implement rendering.

## Future Gate

Render checks must run in Compose and include a nonblank output assertion.
