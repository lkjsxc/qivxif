# Desktop Artifacts

Owner doc for packaged desktop output.

## Targets

- Windows portable archive.
- macOS app bundle.
- Linux portable archive.

## Linux Command

```bash
./scripts/package-linux.sh
```

Output:

- `dist/linux/qivxif-superapp-linux-x86_64/`
- `dist/linux/qivxif-superapp-linux-x86_64.tar.gz`

## Rules

- Packaging must not hide build failures.
- Artifacts include app binary, license, and minimal runtime assets.
- Installer polish follows working portable artifacts.
- Signing and notarization are tracked separately from core build health.
