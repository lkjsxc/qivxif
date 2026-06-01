# Auth

## Requirements

- First admin is created through `qivxifctl`.
- Password hashes use Argon2id.
- Sessions are stored in redb.
- Session cookies are HttpOnly, Secure when TLS is active, and SameSite.
- Cookie-authenticated mutations require CSRF validation.

## Rules

- Client-side ACL only improves UX.
- Server-side ACL is authoritative.
- Auth context is passed to store/query methods.
