# Browser Pane

Owner doc for visible browser behavior.

## Behavior

- Browser panes show address, title, navigation controls, and loading state.
- Back, forward, reload, and open externally are required commands.
- Browser panes are rectangular and clipped to tile bounds when embedding is allowed.
- Policy chooses embedded, detached, or external routing before any page is loaded.
- If in-tile embedding is unavailable or disallowed, use a detached browser window or external browser fallback.

## Policy

- Camera, microphone, location, and notifications are denied by default.
- Downloads require confirmation and a configured directory.
- Navigation policy runs before embedding and can force external open.
- Web pages do not get a general Rust bridge.
- App-internal pages may use a private protocol only for trusted content.
