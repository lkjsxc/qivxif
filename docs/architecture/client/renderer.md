# Renderer

## Status

- Status: implemented for deterministic smoke output and native GPU frames.
- `crates/qivxif-render` exists.
- `SmokeFrame` remains the deterministic CPU evidence path.
- `GpuRenderer` owns native GPU frame submission.

## Current Codebase Facts

- Server returns `BlockCell` data.
- No mesh generation path exists.
- No render quality settings exist.

## Implemented Contract

- Accept server `BlockCell` data as the first render input.
- Produce deterministic nonblank smoke output before GPU feature work.
- Keep GPU code behind this crate boundary.
- Add visual or golden verification in Compose.
- Render simple colored quads from authoritative cells for the native client.

## Rule

- Do not make gameplay visibility depend on experimental GPU features.
- Keep CPU evidence available when native GPU proof is hard to inspect.
