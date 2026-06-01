# ID Contract

All random durable IDs use `<prefix>_<64 lowercase hex characters>`.
The body is 32 bytes from OS randomness.

No durable ID includes time, counter, actor sequence, shard, host, process, or
sortable prefix data.

## Registry

| ID | Prefix | Byte tag | Generation owner | Collision handling |
| --- | --- | --- | --- | --- |
| `UserId` | `usr` | `0x01` | server auth store | retry insert; report collision after repeated failure |
| `ActorId` | `act` | `0x02` | auth/session service | retry insert; actor remains bound to user |
| `SessionId` | `ses` | `0x03` | auth/session service | retry insert; never derive from user data |
| `NodeId` | `nod` | `0x10` | graph reducer caller | retry insert before event acceptance |
| `EdgeId` | `edg` | `0x11` | graph reducer caller | retry insert before event acceptance |
| `EventId` | `evt` | `0x20` | client or server actor | duplicate means idempotent replay only when payload hash matches |
| `CommitGroupId` | `cg` | `0x21` | event author | duplicate means same user action group |
| `BlobHash` | `blb` | `0x30` | content-addressing service | hash mismatch rejects write |
| `ChunkHash` | `chk` | `0x31` | content-addressing service | hash mismatch rejects write |
| `TextDocId` | `txt` | `0x40` | text document service | retry insert before first text event |
| `CursorId` | `cur` | `0x50` | sync service | duplicate updates same cursor record |
| `RequestId` | `req` | `0x60` | server request boundary | diagnostic only; not durable truth |

## String Rules

- The prefix is ASCII lowercase.
- The separator is one underscore.
- The body is lowercase hex.
- The body length is exactly 64 characters.
- Parsers reject missing prefixes, unknown prefixes, uppercase hex, UUID-length
  bodies, short bodies, and extra separators.
- Public API DTOs expose the string form.
- Rust domain crates use newtypes instead of raw strings.

## Binary Rules

- The byte tag is stored before the raw bytes in generic byte-key contexts.
- Table-specific keys may omit the tag only when the table admits one ID type.
- Decoding validates both table owner type and byte tag when the tag is present.
