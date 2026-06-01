# Setup API

## Routes

| Method | Path | Auth | CSRF | Purpose |
| --- | --- | --- | --- | --- |
| `GET` | `/api/setup` | public | no | report whether owner creation is open |
| `POST` | `/api/setup/owner` | public | no | create the first owner account |

## Status Payload

`GET /api/setup` returns:

- `required`: true when no durable user exists.
- `owner_creation_open`: true only while the data store has no users.

## Owner Creation Request

`POST /api/setup/owner` accepts:

- `name`: trimmed display or login name.
- `password`: first owner password.

## Owner Creation Result

Success returns:

- owner user summary
- CSRF token
- next actor sequence
- the same HttpOnly session cookie used by login

## Failure Mapping

- Invalid name or password returns `400`.
- Existing durable user returns `409`.
- Store failure returns `500`.

## Durability

- User existence is read from redb.
- User creation uses the same store path as first-admin creation.
- Password hashing uses the auth crate.
- CSRF hashing uses the auth crate.
- Session records use the same durable session table as login.
- Setup never writes browser-only owner records.
