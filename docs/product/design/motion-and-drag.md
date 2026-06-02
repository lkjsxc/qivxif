# Motion And Drag

## Body Classes

| Class | When | Effect |
| --- | --- | --- |
| `dragging-tab` | Active tab drag | Raises tab strip; enables drop layer hit testing |
| `tab-strip-drag-arming` | Long-press countdown | Disables text selection on strip |
| `tab-drag-armed` | Pointer drag active | `user-select: none` |

## Pointer Constants

| Constant | Value |
| --- | --- |
| Long-press arm | 250ms |
| Coarse cancel distance | 8px |
| Fine drag threshold | 6px |
| Edge fraction | 28% of body dimension |
| Edge clamp min | 56px |
| Edge clamp max | 128px |

## Drop Layer CSS Variables

`PaneDropLayer` sets on the overlay element:

- `--drop-left`, `--drop-top`, `--drop-width`, `--drop-height` in pane coordinates.
- Preview fill: `color-mix(in srgb, var(--q-text) 12%, transparent)`.
- No animated glow; optional 80ms opacity fade only.

## Tab Ghost

- Fixed-position label following pointer during pointer drag.
- Semi-opaque `--q-surface-raised` background, `--q-border-strong` border.
- Centered horizontally on pointer; offset 12px below cursor.

## Native Drag

- MIME: `application/x-qivxif-pane`.
- Fine pointers may use HTML5 drag after movement threshold.
- Coarse pointers use long-press plus pointer capture.

## Motion Limits

- Resize handle feedback is instant; no spring animation.
- Command palette backdrop fades in over 80ms max.
- Feed scroll is native; no parallax on shell chrome.
