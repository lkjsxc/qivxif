# Test Stack

## Canon

Use a small Rust-native test stack that separates fast acceptance from deeper
regression evidence.

## Tools

- `cargo nextest run`: workspace test execution.
- `cargo test --doc`: doctest execution.
- `insta`: snapshot expectations for protocols, worldgen, and rendered output.
- `proptest`: property coverage for protocol, world, inventory, and persistence
  invariants.
- `Criterion.rs`: benchmark evidence for hot paths and regression budgets.

## Rules

- Fast tests run in the `verify` gate.
- Doctests run as their own stage.
- Snapshot updates require owner-doc review.
- Benchmarks inform thresholds but do not replace functional gates.
- Live probes stay in Compose verification docs.
