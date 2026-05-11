# Transport

## Status

- Status: implemented for reliable QUIC streams.
- Owner: `crates/qivxif-net` and `apps/qivxif-serverd::app`.

## Implemented Facts

- qivxif uses Quinn for QUIC.
- `server_config()` creates a Quinn server config.
- `client_endpoint()` creates a Quinn client endpoint for probes.
- The server accepts bidirectional streams with `connection.accept_bi()`.
- The probe client opens bidirectional streams with `connection.open_bi()`.
- Unidirectional streams are disabled in server transport config.
- `recv_wire` caps each request body at 1 MiB.

## Current Lane

- Active lane: reliable bidirectional stream.
- Request pattern: one `ClientMsg` per stream.
- Response pattern: one `ServerMsg` per stream.

## Not Implemented

- Public datagram payloads.
- Gameplay input datagrams.
- Entity delta streams.
- Client asset transport.
