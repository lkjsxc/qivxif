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

## Windows Command

```bash
docker compose -f docker-compose.package.yml run --rm --build package-windows
```

Output:

- `dist/windows/qivxif-superapp-windows-x86_64/`
- `dist/windows/qivxif-superapp-windows-x86_64.zip`

## Windows Contents

- `qivxif-superapp.exe`
- `run.bat`
- `LICENSE`
- `README.md`
- Runtime DLLs required by the Windows GNU build.

## Rules

- Packaging must not hide build failures.
- Artifacts include app binary, license, and minimal runtime assets.
- Installer polish follows working portable artifacts.
- Signing and notarization are tracked separately from core build health.
