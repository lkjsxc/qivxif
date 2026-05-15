# Settings State

Owner doc for app settings state.

## Data

- Theme, UI scale, and fonts.
- Editor defaults.
- Autosave policy.
- Browser policy.
- Explorer preferences.

## Format

- Human-readable TOML.
- Missing settings use documented defaults.
- Invalid fields produce diagnostics and fall back locally.
- Settings writes are atomic where the platform supports it.
