# Desktop Artifacts

Owner doc for packaged desktop output.

## Targets

- Windows portable archive.
- macOS app bundle.
- Linux portable archive.

## Rules

- Packaging must not hide build failures.
- Artifacts include app binary, license, and minimal runtime assets.
- Installer polish follows working portable artifacts.
- Signing and notarization are tracked separately from core build health.
