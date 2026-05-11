# Security References

## Source Reports

- `tmp/deep-research-report (51).md`
- `tmp/deep-research-report (53).md`

## Durable Findings

- QUIC/TLS is the session transport security baseline.
- Argon2id fits password-equivalent secrets.
- Ed25519 fits signatures.
- `rcgen` is acceptable for local certificates only.
- `rustls` fits auxiliary HTTPS endpoints.
- Client caches and prediction are never trusted gameplay truth.

## Local Certificate Boundary

Local self-signed certificate helpers are acceptable for Compose and local
developer flows. A user-facing client needs a real trust policy before release.

## Owner Targets

| Finding | Durable owner |
| --- | --- |
| QUIC session trust | `architecture/network/session-lifecycle.md` |
| Local certificate helper | `operations/verification/compose-pipeline.md` |
| User-facing client trust policy | `decisions/open-questions.md` |
| Client prediction trust boundary | `architecture/client/` |
