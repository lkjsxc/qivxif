# Compaction

## Rules

- Compaction is available only after repair checks exist.
- Public history remains inspectable.
- Dirty client data is outside server compaction authority.
- Snapshot creation may reduce replay cost but does not replace durable operations.
