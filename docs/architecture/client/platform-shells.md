# Platform Shells

## Status

- Status: implemented for desktop smoke; dormant for mobile.
- `apps/qivxif-client-desktop` exists for smoke-frame execution.
- No Android shell exists.
- No iOS shell exists.

## Current Codebase Facts

- Server verification uses `apps/qivxifctl`.
- Probe transport uses `crates/qivxif-probe`.
- Neither crate owns gameplay presentation.

## Desktop Contract

- Keep shell code thin: arguments, runtime start, smoke entrypoint, and exit.
- Delegate protocol state to `qivxif-client-core`.
- Delegate image or renderer output to `qivxif-render`.
- Verify startup and connection behavior through desktop smoke.

## Dormant Mobile Scope

- Android and iOS shells wait until the desktop smoke path exists.
- Mobile lifecycle remains owned by [mobile-lifecycle.md](mobile-lifecycle.md).

## Rule

- Keep platform-specific code thin when shells are introduced.
