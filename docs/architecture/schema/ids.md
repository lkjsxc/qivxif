# ID Contract

All IDs use the form `<prefix>_<lowercase-hex>`. The hex body is 32 random bytes unless the owner doc states a content hash. Parsers reject missing prefixes, unknown prefixes, uppercase hex, short bodies, and extra separators.

## Registry

| ID | Prefix | Byte tag | Generation owner | Collision handling |
| --- | --- | --- | --- | --- |
| `UserId` | `usr` | `0x01` | server auth store | retry insert; report collision after repeated failure |
| `ActorId` | `act` | `0x02` | auth/session service | retry insert; actor remains bound to user |
| `SessionId` | `ses` | `0x03` | auth/session service | retry insert; never derive from user data |
| `NodeId` | `nod` | `0x10` | graph reducer caller | retry insert before operation acceptance |
| `EdgeId` | `edg` | `0x11` | graph reducer caller | retry insert before operation acceptance |
| `OperationId` | `op` | `0x20` | client or server actor | duplicate means idempotent replay |
| `CommitGroupId` | `cg` | `0x21` | operation author | duplicate means same user action |
| `BlobHash` | `blb` | `0x30` | content-addressing service | hash mismatch rejects write |
| `ChunkHash` | `chk` | `0x31` | content-addressing service | hash mismatch rejects write |
| `TextDocId` | `txt` | `0x40` | text document service | retry insert before first text op |
| `CursorId` | `cur` | `0x50` | sync service | duplicate updates same cursor record |

## String Rules

- The prefix is ASCII lowercase.
- The separator is one underscore.
- The body is lowercase hex.
- Public API DTOs expose the string form.
- Rust domain crates use newtypes instead of raw strings.

## Binary Rules

- The byte tag is stored before the raw bytes in generic byte-key contexts.
- Table-specific keys may omit the tag only when the table admits one ID type.
- Decoding validates both table owner type and byte tag when the tag is present.
