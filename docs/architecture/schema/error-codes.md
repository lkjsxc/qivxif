# Error Codes

Error codes are stable public names. They are used by API routes, sync rejection records, CLI JSON output, and browser conflict surfaces.

## Registry

| Code | Meaning | Retry |
| --- | --- | --- |
| `auth.invalid_credentials` | login name or password did not verify | no |
| `auth.session_missing` | request has no valid session | after login |
| `auth.csrf_missing` | mutation lacks CSRF proof | after token refresh |
| `auth.forbidden` | authenticated actor lacks required action | no |
| `graph.not_found` | requested graph record is absent or hidden | no |
| `schema.invalid_id` | ID string failed parsing | no |
| `schema.invalid_input` | request field failed validation | no |
| `schema.unknown_node_kind` | node kind not in registry | no |
| `schema.unknown_edge_kind` | edge kind not in registry | no |
| `schema.unknown_operation_kind` | operation kind not in registry | no |
| `operation.duplicate_actor_seq` | actor sequence conflicts with different op | no |
| `operation.payload_hash_mismatch` | payload bytes do not match hash | no |
| `operation.missing_parent` | required parent operation is absent | after pull |
| `store.conflict` | unique store constraint failed | no |
| `store.unavailable` | database could not complete request | yes |
| `sync.batch_too_large` | request exceeds batch limit | with smaller batch |
| `sync.cursor_invalid` | cursor is absent or malformed | after full pull |
| `publish.slug_conflict` | public slug already owned by another post | no |
| `text.invalid_range` | text operation references an invalid range | no |
| `cache.quota_exceeded` | client cache plan cannot protect required data | after user action |

## Rules

- New codes are added here before use.
- Machine clients branch on `code`, not `message`.
- Field validation failures include the exact request field path.
