# Runtime Config

LLM purpose: list server runtime config fields that must be explicit.

## Owner Scope

This file owns operational config names and deployment requirements. Field
semantics must stay aligned with implementation tests and owner docs.

## Fields

| Field | Meaning |
| --- | --- |
| `bind_addr` | Socket address for QUIC. |
| `data_dir` | Durable state directory. |
| `world_seed` | Deterministic world seed. |
| `build_contract` | Build identity gate. |
| `protocol_contract` | Wire contract gate. |

## Rule

Config must be explicit. Runtime defaults belong in owner docs and tests.
Production certificate paths, trust roots, and rotation ownership must be added
here before any non-local deployment accepts clients.
