# Renderer

## Status

- Status: implemented for deterministic desktop smoke output.
- `crates/qivxif-render` exists.
- No `wgpu` renderer code exists in the current workspace.

## Current Codebase Facts

- Server returns `BlockCell` data.
- No mesh generation path exists.
- No GPU capability gate exists.
- No render quality settings exist.

## Implemented Contract

- Accept server `BlockCell` data as the first render input.
- Produce deterministic nonblank smoke output before GPU feature work.
- Keep future `wgpu` code behind this crate boundary.
- Add visual or golden verification in Compose.

## Rule

- Do not make gameplay visibility depend on experimental GPU features.
