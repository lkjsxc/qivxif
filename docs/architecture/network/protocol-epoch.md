# Protocol Epoch

## Status

- Status: implemented.
- Owner: `crates/qivxif-protocol::CURRENT_PROTOCOL_EPOCH` and server config.

## Fields

| Field | Source | Meaning |
| --- | --- | --- |
| `build_epoch` | `ServerConfig` and `ClientMsg::Hello` | Build identity gate |
| `protocol_epoch` | `ServerConfig` and `ClientMsg::Hello` | Wire contract gate |
| `world_epoch` | `WorldMeta` | Persistent world identity |

## Current Values

- Active public `protocol_epoch`: `1`.
- `WorldMeta::new(seed)` creates `world_epoch` as `world-{seed}`.
- `WorldMeta::new(seed)` sets `schema_epoch` to `1`.

## Rejection Rules

- Empty client or server build epoch returns `BuildEpochMissing`.
- Protocol epoch mismatch returns `ProtocolEpochMismatch`.
- Rejected hello does not mark the session as hello-complete.

## Codec Link

- Message shape and accepted bytes are owned by [protocol-codecs.md](protocol-codecs.md).
