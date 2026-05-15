# Active Work

Owner doc for the current implementation lane.

## Focus

- Replace the retired voxel canon with the Rust-native tile super app canon.
- Make the super app the default implemented path.
- Keep the quality gate small, strict, and easy for agents to run.

## Near Work

1. Land the documentation pivot.
2. Update topology and wording checks to match the stricter docs tree.
3. Add the super app entry crate and shell crates.
4. Add tiles, command routing, editor buffer, and editor view.
5. Add workspace state, persistence, explorer, Markdown preview, and browser policy.

## Acceptance

- Docs describe only the tile super app direction.
- No game, server, terrain, protocol, or world behavior remains as canon.
- Compose remains the acceptance boundary.
