# Platform Shells

## Status

- Status: not implemented.
- No desktop shell exists.
- No Android shell exists.
- No iOS shell exists.

## Current Codebase Facts

- Server verification uses `apps/qivxifctl`.
- Probe transport uses `crates/qivxif-probe`.
- Neither crate owns gameplay presentation.

## Activation Requirements

- Add a client workspace member.
- Define platform input ownership.
- Define surface lifecycle ownership.
- Add verification for startup and connection behavior.

## Rule

- Keep platform-specific code thin when shells are introduced.
