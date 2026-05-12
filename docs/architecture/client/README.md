# Client Architecture

Use this subtree for implemented, active, and dormant client contracts.

## Current Implementation

- `crates/qivxif-client-core` owns reusable headless client behavior.
- `apps/qivxif-client-cli` is the implemented headless protocol client.
- Desktop GUI runtime, shell, renderer, input, UI, and assets are implemented
  for smoke output and native e2e verification.
- `apps/qivxifctl` is a probe CLI, not a game client.
- `crates/qivxif-probe` is verification code, not gameplay client code.

## Child Index

- [headless-client.md](headless-client.md): implemented protocol client boundary.
- [gui-runtime.md](gui-runtime.md): active desktop GUI runtime boundary.
- [native-e2e-client.md](native-e2e-client.md): active native window client gate.
- [platform-shells.md](platform-shells.md): platform shell boundary.
- [renderer.md](renderer.md): renderer boundary.
- [asset-streaming.md](asset-streaming.md): asset and fallback pack boundary.
- [prediction-reconciliation.md](prediction-reconciliation.md): dormant prediction boundary.
- [mobile-lifecycle.md](mobile-lifecycle.md): dormant mobile lifecycle boundary.

## Rule

- Desktop GUI behavior is implemented through the smoke and native e2e gates.
