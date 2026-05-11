# Network Security

## Status

- Status: local transport security only.
- Owner: `crates/qivxif-net`.

## Implemented Transport Facts

- Quinn QUIC provides encrypted transport.
- Local server certificates are generated with `rcgen`.
- Probe clients use a custom verifier that accepts the local certificate.
- The verifier bypass exists for local Compose probes.
- Remote client addresses are logged at debug level only.

## Not Implemented

- Account authentication.
- Session tokens.
- Admin capability tokens.
- HTTPS admin endpoints.
- Password storage.
- Manifest signatures.

## Rules For Future Activation

- Do not treat client prediction or cached assets as authority.
- Do not log reusable secrets when authentication exists.
- Production certificate ownership belongs in deployment docs when deployment exists.
