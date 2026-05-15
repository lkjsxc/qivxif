# Browser Stack

Owner doc for browser stack findings.

## Selected Stack

- `wry` for optional platform webview embedding.
- Browser controller trait for shell integration.
- Policy layer before embedding for permissions, downloads, and external links.
- Detached or external fallback for weak embed paths.

## Risk

Linux webview composition can vary by toolkit and compositor. The adapter boundary keeps that risk isolated.
