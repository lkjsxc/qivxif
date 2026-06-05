# Media Flows

## Upload

- User selects a file.
- Browser creates an upload session.
- File is sliced into chunks.
- Chunks are hashed and stored locally before transfer when possible.
- Server receives chunks and commits by content hash.
- Refresh resumes from upload session metadata.

## Import

- Browser records metadata and chunk locators.
- Large files are not read fully into memory.
- The resource planner decides which chunks remain cached.

## Preview

- Metadata renders immediately.
- Thumbnail jobs run in the orchestrator.
- Preview failures are visible diagnostics.

## Attach

- Attaching media to a node writes a `media_attachment` edge.
- Removing an attachment tombstones that edge.
- Deleting an asset follows ACL and retention rules.

## Publish

- Public media serving checks visibility and ACL.
- Private media never moves through public routes.
- Range reads are allowed only for visible content.
