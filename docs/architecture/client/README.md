# Client Architecture

Use this subtree for implemented and dormant client contracts.

## Current Implementation

- `apps/qivxif-client-cli` is the implemented headless protocol client.
- No renderer crate exists.
- No platform shell exists.
- `apps/qivxifctl` is a probe CLI, not a game client.
- `crates/qivxif-probe` is verification code, not gameplay client code.

## Child Index

- [headless-client.md](headless-client.md): implemented protocol client boundary.
- [platform-shells.md](platform-shells.md): dormant platform shell boundary.
- [renderer.md](renderer.md): dormant renderer boundary.
- [asset-streaming.md](asset-streaming.md): dormant asset streaming boundary.
- [prediction-reconciliation.md](prediction-reconciliation.md): dormant prediction boundary.
- [mobile-lifecycle.md](mobile-lifecycle.md): dormant mobile lifecycle boundary.

## Rule

- Describe only `qivxif-client-cli` behavior as implemented until shared client
  core, shell, or renderer crates exist.
