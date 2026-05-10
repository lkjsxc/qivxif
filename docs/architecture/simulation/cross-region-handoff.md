# Cross-Region Handoff

## Purpose

Entity movement across ownership boundaries must be explicit.

## Contract

- Source region validates outgoing state.
- Destination region accepts or rejects handoff.
- Session routing follows the accepted owner.
- Failed handoff keeps the source region authoritative.
