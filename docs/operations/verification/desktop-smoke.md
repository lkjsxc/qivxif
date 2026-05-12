# Desktop Smoke

## Status

- Status: active once `qivxif-client-desktop` exists.
- Owner: `apps/qivxif-client-desktop --smoke-frame`.

## Contract

- Run through Docker Compose.
- Connect to the local server through QUIC.
- Send hello and join.
- Request the origin chunk neighborhood.
- Render one deterministic frame artifact from returned chunk cells.
- Assert the artifact exists and is nonblank.

## Output

- Smoke artifacts are generated output.
- Do not commit smoke artifacts.
- Use task evidence logs for command output.

## Current Gate

- Until the desktop client exists, renderer checks remain dormant in
  [render-goldens.md](render-goldens.md).
