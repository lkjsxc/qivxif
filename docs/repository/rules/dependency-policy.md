# Dependency Policy

Owner doc for repository-level dependency direction.

## Defaults

| Default | Constraint |
|---|---|
| Rust-native dependencies. | Prefer crates that fit the Rust workspace. |
| Boring stable crates for the initial slice. | Avoid dependency novelty for core paths. |
| Renderer and mobile dependencies need owner docs and verification. | Do not add them as incidental dependencies. |

## Current Direction

| Dependency | Use |
|---|---|
| Tokio | Async runtime. |
| Quinn | QUIC transport. |
| redb | Hot state. |
| `postcard` | Compact schema-bound protocol payloads. |
| serde JSON | Diagnostics or tool output that is not protocol traffic. |

## LLM Notes

- This file records existing dependency direction only.
- Do not add new dependency recommendations without an owner doc and verification path.
