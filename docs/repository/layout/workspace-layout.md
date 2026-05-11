# Workspace Layout

Owner doc for the active Rust workspace shape.

## Scope

The active workspace is the server, probe, and headless client slice. Renderer,
authentication, gameplay, and mobile crates are added only after their owner
docs are active.

## Apps

| App | Role |
|---|---|
| `qivxif-serverd` | Authoritative server. |
| `qivxif-client-cli` | Headless protocol client. |
| `qivxifctl` | Quality and probe command. |

## Crates

| Crate | Role |
|---|---|
| `qivxif-core` | Shared core types. |
| `qivxif-client-core` | Reusable headless client session support. |
| `qivxif-protocol` | Protocol payloads and schema-bound types. |
| `qivxif-net` | Network-facing support. |
| `qivxif-world` | World data support. |
| `qivxif-sim` | Simulation support. |
| `qivxif-storage` | Persistence support. |
| `qivxif-quality` | Quality and verification support. |
| `qivxif-probe` | Probe support. |

## Planned Areas

- Desktop and mobile platform shells.
- Renderer family.
- Authentication and identity.
- Gameplay systems.

## LLM Notes

- Do not add planned areas to the active workspace list until owner docs are active.
- Keep crate names exact.
