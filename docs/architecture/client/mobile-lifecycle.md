# Mobile Lifecycle

## Canon

Mobile clients target full gameplay parity.

## Rules

- Platform lifecycle owns surface creation and pause/resume.
- Shared Rust core owns gameplay state.
- Network and asset tasks must tolerate backgrounding.
- Quality gates tune rendering, not gameplay availability.
