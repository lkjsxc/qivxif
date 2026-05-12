# Windows Demo Bundle

## Status

- Status: implemented for an unsigned internal demo bundle.
- Owner: `docker/windows/demo-bundle.Dockerfile`.

## Contract

- Linux Docker cross-builds Windows GNU artifacts.
- The target is `x86_64-pc-windows-gnu`.
- The bundle is portable and unsigned.
- The bundle is for internal demos and smoke checks, not public release.
- Public signing, installers, and Windows-native MSVC release jobs are outside
  this contract.

## Expected Command

```bash
./scripts/verify-windows-demo-bundle.sh
```

## Destructive Output Behavior

- The verification script removes `dist/windows/` before building.
- Do not store hand-authored files under `dist/windows/`.

## Output Layout

```text
dist/windows/demo/qivxif-serverd.exe
dist/windows/demo/qivxif-client-cli.exe
dist/windows/demo/config/server.toml
dist/windows/demo/data/.keep
dist/windows/demo/start-server.cmd
dist/windows/demo/run-client-demo.cmd
dist/windows/demo/README.md
dist/windows/demo/manifest.json
dist/windows/demo/checksums.txt
dist/windows/qivxif-demo-windows-x86_64.zip
```

## Launcher Behavior

- `start-server.cmd` changes to the bundle directory and runs
  `qivxif-serverd.exe serve --config config/server.toml`.
- `run-client-demo.cmd` changes to the bundle directory and runs the client
  against `127.0.0.1:4443` with `--tls local-compose`.
- `run-client-demo.cmd` pauses after completion so output can be inspected.
- `config/server.toml` binds to `127.0.0.1:4443` and uses `data` as `data_dir`.

## Verification Requirements

- Both `.exe` files exist and are non-empty.
- Config, launchers, README, manifest, and checksums exist.
- `checksums.txt` includes both executables, launchers, and config.
- The zip exists and contains the same `demo/` tree.
