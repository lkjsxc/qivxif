# Native Client E2E

## Status

- Status: active.
- Owner: `qivxif-client-desktop e2e`.
- Compose service: `desktop-e2e`.

## Contract

- Run under Xvfb in Docker Compose.
- Connect to the real server at `server:4443`.
- Use `--tls local-compose` and the public QUIC protocol.
- Open a native `winit` window.
- Initialize the `qivxif-render` GPU renderer.
- Fetch the origin chunk neighborhood.
- Render frames from authoritative cached cells.
- Place block `9` at `{ x: 1, y: 3, z: 1 }`.
- Verify the acknowledgement and cache state.
- Remove the same block by sending block `0`.
- Verify the acknowledgement and cache state.
- Write a summary JSON file and nonblank PPM evidence frame.

## Required Evidence

- `connected` is `true`.
- `joined` is `true`.
- `chunks` is at least `9`.
- `cells` is greater than `0`.
- `gpu_frames` is greater than `0`.
- `nonzero_pixels` is greater than `0`.
- `place_ack` is `true`.
- `remove_ack` is `true`.

## Canonical Command

```bash
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm -T desktop-e2e
```

## Output

- Evidence is generated output.
- Do not commit generated PPM or JSON evidence files.
- A passing command prints `desktop e2e ... ok`.
