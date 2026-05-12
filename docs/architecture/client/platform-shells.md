# Platform Shells

## Status

- Status: implemented for desktop smoke and native e2e; dormant for mobile.
- `apps/qivxif-client-desktop` exists for smoke-frame, run, and e2e commands.
- No Android shell exists.
- No iOS shell exists.

## Current Codebase Facts

- Server verification uses `apps/qivxifctl`.
- Probe transport uses `crates/qivxif-probe`.
- Neither crate owns gameplay presentation.

## Desktop Contract

- Keep shell code thin: arguments, runtime start, native loop, and exit.
- Delegate protocol state to `qivxif-client-core`.
- Delegate image or renderer output to `qivxif-render`.
- Verify CPU output through desktop smoke.
- Verify native window behavior through native e2e.

## Dormant Mobile Scope

- Android and iOS shells wait until the desktop native path is stable.
- Mobile lifecycle remains owned by [mobile-lifecycle.md](mobile-lifecycle.md).

## Rule

- Keep platform-specific code thin when shells are introduced.
