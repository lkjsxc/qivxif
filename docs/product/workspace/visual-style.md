# Visual Style

Native qivxif uses a compact dark workspace.

## Theme Tokens

| Token | Value | Use |
| --- | --- | --- |
| `background` | `#0f1117` | window and empty space |
| `surface` | `#171a22` | pane body |
| `surface_raised` | `#202431` | tabs and headers |
| `stroke` | `#303646` | inactive borders |
| `focus` | `#77b7ff` | keyboard focus and active pane |
| `accent` | `#8bd17c` | safe positive actions |
| `warning` | `#f4c76b` | dirty or recoverable state |
| `danger` | `#ff7b72` | destructive actions and errors |

## Pane Hierarchy

- The header is one compact row with primary commands, URL input, diagnostics path, and state path.
- Pane frames have visible borders; the active pane uses the `focus` border.
- Tab stacks show every tab, mark the active tab, and expose close and maximize controls.
- Split handles are visible separators and keep panes above minimum sizes.
- Maximized panes replace the central workspace but keep the global header visible.

## Display Expectations

- Windows portable builds must render the same dark native shell.
- Browser content is shown as fallback controls unless a real embedded webview is enabled and verified.
- Text must remain readable at 100%, 125%, and 150% display scaling.

## Accessibility

- Focus state is visible without relying on color alone.
- Dirty and recovery states include text labels.
- Buttons use stable labels and do not change size when state changes.
