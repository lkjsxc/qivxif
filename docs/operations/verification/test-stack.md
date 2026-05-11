# Test Stack

LLM purpose: identify test tools and where they fit in acceptance.

## Canon

Use a small Rust-native test stack that separates fast acceptance from deeper
regression evidence.

## Tools

| Tool | Use |
| --- | --- |
| `cargo nextest run` | Workspace test execution. |
| `cargo test --doc` | Doctest execution. |
| `insta` | Snapshot expectations for protocols, worldgen, and rendered output. |
| `proptest` | Property coverage for protocol, world, inventory, and persistence invariants. |
| `Criterion.rs` | Benchmark evidence for hot paths and regression budgets. |

## Rules

- Fast tests run in the `verify` gate.
- Doctests run as their own stage.
- Snapshot updates require owner-doc review.
- Benchmarks inform thresholds but do not replace functional gates.
- Live probes stay in Compose verification docs.
