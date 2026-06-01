# Content Addressing

## Records

- `BlobHash`
- `ChunkHash`
- payload hash
- text snapshot hash

## Rules

- Large payloads are hashed outside write transactions.
- Blob chunks are referenced by manifests.
- Hash access still requires ACL.
