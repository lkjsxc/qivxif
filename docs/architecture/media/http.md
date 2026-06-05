# Media HTTP

## Upload Routes

| Method | Path | Purpose |
| --- | --- | --- |
| `POST` | `/api/media/uploads` | initialize upload session |
| `PUT` | `/api/media/uploads/{upload_id}/chunks/{index}` | upload one chunk |
| `POST` | `/api/media/uploads/{upload_id}/complete` | verify and commit upload |
| `POST` | `/api/media/uploads/{upload_id}/abort` | abort upload session |

## Asset Routes

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/media/assets/{asset_id}` | metadata |
| `GET` | `/api/media/assets/{asset_id}/content` | content with Range support |
| `HEAD` | `/api/media/assets/{asset_id}/content` | size, type, cache metadata |
| `GET` | `/api/media/assets/{asset_id}/variants/{variant}` | variant bytes |
| `DELETE` | `/api/media/assets/{asset_id}` | tombstone or remove by policy |

## Route Rules

- Cookie-authenticated mutation routes require CSRF.
- API-token requests require matching scopes.
- Range requests return only visible content.
- `HEAD` exposes metadata only when the viewer can read the asset.
- Public media routes serve only public or unlisted permitted assets.
- OPFS isolation headers remain intact when static assets and media share a host.

## Upload Completion

Completion checks chunk count, chunk hashes, total size, MIME policy, ACL, and
idempotency before writing accepted metadata and graph edges.
