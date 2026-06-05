# Admin Keys

## Entities

- `invite_code`.
- `access_key`.
- `api_token`.
- `scope_grant`.
- `key_revocation`.
- `key_audit_event`.

## Scopes

- `graph.read`.
- `graph.write`.
- `graph.link`.
- `text.write`.
- `media.read`.
- `media.write`.
- `publish.write`.
- `sync.pull`.
- `sync.push`.
- `profile.write`.
- `admin.users`.
- `admin.keys`.
- `admin.media`.
- `admin.audit`.

## Invite Codes

- Invite codes grant registration permission.
- Codes may set role, scope defaults, max uses, and expiry.
- Acceptance creates the user and profile node in one durable transaction.
- Revoked, expired, or exhausted codes fail closed.

## API Tokens

- Tokens are generated with high entropy.
- Token secrets are displayed once.
- Only a salted or keyed hash is stored.
- Token prefixes identify token type for operators.
- Admin UI never shows a full token after creation.

## Audit

Audit records include issuance, use, denial, expiry, and revocation. Audit views
show actor, target, scopes, route or command, result, and reason code.
