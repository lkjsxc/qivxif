# API Envelope

Every `/api` response uses the same envelope. Public page routes may return HTML, but their data source still comes from the same domain services.

## Success Shape

| Field | Type | Rule |
| --- | --- | --- |
| `request_id` | `RequestId` | generated per request |
| `server_time` | timestamp string | server observation time, never a sync cursor by itself |
| `capabilities` | string array | capabilities active for this response |
| `payload` | object | route-specific success payload |
| `error` | null | absent error |

## Error Shape

| Field | Type | Rule |
| --- | --- | --- |
| `request_id` | `RequestId` | same request id used in logs |
| `server_time` | timestamp string | server observation time |
| `capabilities` | string array | capabilities known for this response |
| `payload` | null | absent payload |
| `error` | object | structured error |

## Error Object

| Field | Type | Rule |
| --- | --- | --- |
| `code` | error code | from [error-codes.md](error-codes.md) |
| `message` | string | concise user-visible summary |
| `field_errors` | array | field-specific validation failures |
| `retry` | object or null | retry class and wait hint |
| `conflict` | object or null | durable conflict detail |
| `required_capability` | string or null | missing capability when applicable |

## Rules

- Handlers do not return raw ad hoc JSON.
- Route tests assert envelope shape for success and failure.
- Sync rejections reuse the same error code registry.
