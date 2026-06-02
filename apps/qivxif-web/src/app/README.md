# App

## Contents

- [bootstrap.ts](bootstrap.ts): mount root and start controller.
- [controller.ts](controller.ts): workspace state, dispatch, subscribe.

## Boundary

UI subscribes to controller state. Commands flow through `dispatch`.
