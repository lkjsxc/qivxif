# Message Lanes

## Status

- Status: one implemented public lane.
- Owner: [transport.md](transport.md) and [protocol-messages.md](protocol-messages.md).

## Implemented Lane

| Lane | Status | Current use |
| --- | --- | --- |
| Reliable bidirectional stream | Implemented | Hello, join, ping, chunk request, mutation, flush, error responses |

## Dormant Lanes

| Lane | Status | Activation condition |
| --- | --- | --- |
| Datagram latest-wins lane | Not implemented | Add public datagram payloads and protocol tests |
| Reliable bulk lane | Not separate | Split only after chunk payload verification requires it |
| Repair keyframe lane | Not implemented | Add latest-wins state first |

## Rule

- Do not document a lane as active until code sends and verifies payloads on it.
