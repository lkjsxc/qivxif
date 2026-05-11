# Network Security

## Authority

The server owns gameplay truth. Client prediction, local rendering, and cached
assets never authorize movement, combat, inventory, claims, markets, or terrain
mutation.

## Transport

- Quinn QUIC sessions provide the game transport.
- Session transport uses TLS through QUIC.
- Admin and content HTTPS endpoints use rustls when they exist.

## Secrets And Signatures

- Store password-equivalent secrets with Argon2id.
- Use Ed25519 signatures for manifests, admin capability tokens, and replay
  attestations when those features exist.
- Use rcgen for local development certificates only.
- Production certificate ownership belongs to deployment docs.

## Rules

- Authenticated identity is separate from character ownership.
- Session tokens are scoped to one connection and one joined character.
- Privileged admin actions require signed or otherwise capability-scoped proof.
- Logs, traces, and replay bundles must not contain reusable secrets.
