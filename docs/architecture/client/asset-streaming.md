# Asset Streaming

## Status

- Status: implemented for embedded fallback assets.
- No asset pipeline exists.
- No client cache exists.

## Current Codebase Facts

- Public server protocol returns chunk cells only.
- No asset manifest protocol exists.
- `ArchiveStore` manifest tests are storage smoke tests, not client asset streaming.

## Implemented Contract

- Provide a tiny deterministic block palette for smoke rendering.
- Do not add asset manifest protocol messages yet.
- Keep desktop smoke independent of external asset downloads.

## Later Requirements

- Define asset manifest files or messages.
- Define cache ownership, decompression task ownership, and eviction rules.
- Add mobile and desktop verification after desktop smoke exists.
