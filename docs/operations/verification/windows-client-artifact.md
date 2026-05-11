# Windows Client Artifact

## Status

- Status: planned for the headless client.
- Owner: `apps/qivxif-client-cli` artifact build.

## Contract

- Linux Docker builds the Windows headless client artifact.
- The target is `x86_64-pc-windows-gnu`.
- The artifact is an unsigned smoke artifact, not a public release.
- Public release signing and Windows-native release builds remain outside this
  contract.

## Expected Command

```bash
./scripts/verify-windows-client-cli.sh
```

## Required Behavior

- The command uses Docker Compose.
- The output directory is `dist/windows/client-cli/`.
- The expected executable is `qivxif-client-cli.exe`.
- Failure to build blocks the Windows artifact gate.
