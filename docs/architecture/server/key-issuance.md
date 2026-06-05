# Key Issuance

## Stored Records

Key storage records include:

- key id.
- key kind.
- user id when bound.
- hashed secret.
- scopes.
- role grant when invite.
- max uses.
- use count.
- expiry.
- revoked flag.
- created by.
- created time.
- last used time.

## Secret Handling

- Secrets are generated with high entropy.
- Secrets are shown once at creation.
- Only hashes are stored.
- Token prefixes identify invite, access key, or API token.
- Audit records never store full secrets.

## Routes

| Method | Path | Purpose |
| --- | --- | --- |
| `POST` | `/api/admin/invites` | issue invite |
| `GET` | `/api/admin/invites` | list invites |
| `POST` | `/api/admin/invites/{id}/revoke` | revoke invite |
| `POST` | `/api/admin/keys` | issue API token |
| `GET` | `/api/admin/keys` | list keys |
| `POST` | `/api/admin/keys/{id}/revoke` | revoke key |
| `POST` | `/api/invites/accept` | accept invite |
| `GET` | `/api/admin/key-audit` | list audit events |

## Enforcement

- Scope checks run before handler service calls.
- Cookie-authenticated admin mutations require CSRF.
- API-token requests use token scopes and never gain cookie privileges.
- Revoked, expired, and exhausted credentials fail closed.
- Key use writes audit rows for allowed and denied attempts.
- Rate limits apply to invite acceptance and token-authenticated mutations.
