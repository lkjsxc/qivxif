# Orientation

## Current Repository

The repository now has a docs-first canon and an initial server/probe slice. The
docs canon is the source of truth for implementation.

## Primary Paths

- `docs/`: authoritative behavior and architecture.
- `apps/`: executable binaries.
- `crates/`: reusable Rust crates.
- `config/`: runtime and verification config.
- `scripts/`: Compose-run verification helpers.
- `tmp/`: input material, not canon.

## First Reading Order

1. `docs/README.md` for the global documentation map.
2. `docs/vision/README.md` for project intent.
3. This directory for local workflow.
4. The owner doc for the behavior being changed.

## Research Boundary

Deep research reports in `tmp/` may contain useful synthesis. Copy only durable,
non-speculative findings into owner docs before using them for implementation.
