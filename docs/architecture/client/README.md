# Client Architecture

Use this subtree for dormant client contracts.

## Current Implementation

- No native client crate exists.
- No renderer crate exists.
- No platform shell exists.
- `apps/qivxifctl` is a probe CLI, not a game client.
- `crates/qivxif-probe` is verification code, not gameplay client code.

## Child Index

- [platform-shells.md](platform-shells.md): dormant platform shell boundary.
- [renderer.md](renderer.md): dormant renderer boundary.
- [asset-streaming.md](asset-streaming.md): dormant asset streaming boundary.
- [prediction-reconciliation.md](prediction-reconciliation.md): dormant prediction boundary.
- [mobile-lifecycle.md](mobile-lifecycle.md): dormant mobile lifecycle boundary.

## Rule

- Do not describe client behavior as implemented until workspace client crates exist.
