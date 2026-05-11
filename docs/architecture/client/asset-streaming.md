# Asset Streaming

## Status

- Status: not implemented.
- No asset pipeline exists.
- No client cache exists.

## Current Codebase Facts

- Public server protocol returns chunk cells only.
- No asset manifest protocol exists.
- `ArchiveStore` manifest tests are storage smoke tests, not client asset streaming.

## Activation Requirements

- Define asset manifest messages or files.
- Define cache ownership.
- Define decompression task ownership.
- Define eviction rules.
- Add mobile and desktop verification.
