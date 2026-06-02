# Public Browser Assets

Static files in this directory are copied into the web build output unchanged.

## Contents

- [manifest.json](manifest.json): install metadata for the app shell.
- [styles.css](styles.css): CSS import root.
- [styles/](styles/): authored CSS by concern.

## Rules

- Keep colors in CSS variables so qivxif can differ from the reference app.
- Keep shell layout CSS explicit: header, tile grid, pane body, and tab rail.
- Do not place generated assets here.
