# Renderer

## Status

- Status: not implemented.
- No renderer crate exists.
- No `wgpu` renderer code exists in the current workspace.

## Current Codebase Facts

- Server returns `BlockCell` data.
- No mesh generation path exists.
- No GPU capability gate exists.
- No render quality settings exist.

## Activation Requirements

- Add renderer crate ownership.
- Define chunk mesh input format.
- Define capability gates.
- Add visual or golden verification.

## Rule

- Do not make gameplay visibility depend on experimental GPU features.
