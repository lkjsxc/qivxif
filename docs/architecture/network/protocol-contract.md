# Protocol Contract

## Status

- Status: implemented.
- Owner: `crates/qivxif-protocol::CURRENT_PROTOCOL_CONTRACT` and server config.

## Fields

| Field | Source | Meaning |
| --- | --- | --- |
| `build_contract` | `ServerConfig` and `ClientMsg::Hello` | Build identity gate |
| `protocol_contract` | `ServerConfig` and `ClientMsg::Hello` | Wire contract gate |
| `world_id` | `WorldMeta` | Persistent world identity |

## Current Values

- Active public `protocol_contract`: `postcard-reliable-streams`.
- `WorldMeta::new(seed)` creates `world_id` as `world-{seed}`.
- `WorldMeta::new(seed)` sets `schema_contract` to `redb-chunk-overlays`.

## Rejection Rules

- Empty client or server build contract returns `BuildContractMissing`.
- Protocol contract mismatch returns `ProtocolContractMismatch`.
- Rejected hello does not mark the session as hello-complete.

## Codec Link

- Message shape and accepted bytes are owned by [protocol-codecs.md](protocol-codecs.md).
