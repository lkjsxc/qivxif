# Desktop Smoke

## Status

- Status: implemented.
- Owner: `apps/qivxif-client-desktop --smoke-frame`.

## Contract

- Run through Docker Compose.
- Connect to the local server through QUIC.
- Send hello and join.
- Request the origin chunk neighborhood.
- Place block `9` at `{ x: 1, y: 3, z: 1 }` through the authoritative path.
- Assert the mutation acknowledgement is present in `WorldCache`.
- Render one deterministic frame artifact after the acknowledgement.
- Assert the artifact exists and is nonblank.
- Print deterministic frame byte and nonzero-pixel counts.

## Output

- Smoke artifacts are generated output.
- Do not commit smoke artifacts.
- Use task evidence logs for command output.

## Current Gate

- The `desktop-smoke` Compose service runs inside `./scripts/verify-compose.sh`.
