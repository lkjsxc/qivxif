# Media Diagnostics

## Fields

- asset count.
- total metadata bytes.
- cached original bytes.
- cached chunk bytes.
- thumbnail bytes.
- upload sessions.
- download sessions.
- failed transfers.
- missing chunks.
- ACL denials.

## Actions

- Resume upload.
- Abort upload.
- Retry failed chunk.
- Verify local chunks.
- Pin for offline use.
- Unpin cached copy.
- Open resource-planner explanation.

## Safety

Diagnostics distinguish owned durable media from expendable local cache. Pruning
cache chunks never deletes server asset metadata or accepted graph records.
