# Platform Dependencies

Owner doc for native dependencies.

## Browser

- Windows uses WebView2 when browser embedding is enabled.
- macOS uses the system WebKit framework.
- Linux uses WebKitGTK and GTK packages.

## Graphics

- The shell uses `wgpu` backends supplied by the platform.
- Device-loss handling is part of the renderer contract.
- CI should install Linux webview and graphics build dependencies explicitly.
