# Platform Dependencies

Owner doc for native dependencies.

## Browser

- Windows uses WebView2 when browser embedding is enabled.
- macOS uses the system WebKit framework.
- Linux uses WebKitGTK and GTK packages.
- Portable Windows packaging keeps browser embedding disabled.

## Windows Build

- The portable archive uses `x86_64-pc-windows-gnu`.
- The linker is `x86_64-w64-mingw32-gcc`.
- The package image installs the MinGW toolchain and ZIP tools.
- Runtime DLLs detected from the executable are copied into the bundle.

## Graphics

- The shell uses `wgpu` backends supplied by the platform.
- Device-loss handling is part of the renderer contract.
- CI should install Linux webview and graphics build dependencies explicitly.
