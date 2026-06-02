# Design Tokens

## CSS Variables

All browser styles consume these variables from `tokens.css`.

### Color

| Token | Value | Use |
| --- | --- | --- |
| `--q-bg` | `#0b0b0c` | App background |
| `--q-bg-deep` | `#111113` | Header, overlays |
| `--q-surface` | `#151518` | Pane background |
| `--q-surface-raised` | `#1b1b1f` | Pane header, inputs |
| `--q-overlay` | `#222228` | Buttons, chips |
| `--q-border` | `#2d2d33` | Default borders |
| `--q-border-strong` | `#3a3a42` | Active borders |
| `--q-text` | `#e7e7ea` | Primary text |
| `--q-muted` | `#a1a1aa` | Secondary text |
| `--q-faint` | `#71717a` | Tertiary text |
| `--q-accent` | `#d4d4d8` | Active tab, primary action |
| `--q-focus` | `#f4f4f5` | Focus ring |
| `--q-danger` | `#ef4444` | Errors, destructive |
| `--q-warning` | `#eab308` | Warnings |
| `--q-success` | `#86efac` | Success, synced |

### Radius

| Token | Value |
| --- | --- |
| `--radius-ui` | `2px` |
| `--radius-panel` | `1px` |
| `--radius-tab` | `1px` |

### Spacing

| Token | Value |
| --- | --- |
| `--space-1` | `4px` |
| `--space-2` | `8px` |
| `--space-3` | `12px` |
| `--space-4` | `16px` |
| `--space-5` | `24px` |

### Typography

| Token | Value |
| --- | --- |
| `--font-ui` | `Inter, ui-sans-serif, system-ui, sans-serif` |
| `--font-mono` | `ui-monospace, SFMono-Regular, Menlo, monospace` |
| `--text-xs` | `11px` |
| `--text-sm` | `12px` |
| `--text-base` | `13px` |
| `--text-lg` | `15px` |
| `--text-xl` | `18px` |

## Usage Rules

- Shell chrome uses `--font-ui` at `--text-base` or `--text-sm`.
- Event IDs, node IDs, and diagnostics use `--font-mono` at `--text-xs` or
  `--text-sm`.
- Primary actions invert text onto `--q-accent` background.
- Danger actions keep `--q-danger` border or text, never as default chrome.
- Theme is dark only for the current product slice.

## File Ownership

- Token definitions live in `apps/qivxif-web/src/styles/tokens.css`.
- Component styles reference tokens only; no hard-coded hex in component files
  except token definitions.
