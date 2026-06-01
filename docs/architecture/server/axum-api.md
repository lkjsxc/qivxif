# Axum API

## Endpoints

- `GET /health`
- `GET /api/server-info`
- `POST /api/auth/login`
- `POST /api/auth/logout`
- `GET /api/me`
- `POST /api/nodes`
- `GET /api/nodes/{node_id}`
- `GET /api/nodes/{node_id}/edges`
- `GET /api/graph/neighborhood`
- `POST /api/sync/push`
- `GET /api/sync/pull`
- `GET /api/text/{node_id}`
- `POST /api/text/{node_id}/ops`
- `GET /api/feed/home`
- `POST /api/publish/{node_id}`
- `POST /api/unpublish/{node_id}`

## Response Envelope

Every API response uses structured success or error data with request ID, server time, capabilities, and payload.
