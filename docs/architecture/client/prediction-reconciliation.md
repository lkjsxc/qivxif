# Prediction And Reconciliation

## Status

- Status: not implemented.
- No gameplay client exists.
- No movement protocol exists.

## Implemented Related Fact

- Terrain mutation acknowledgements exist as `MutationAck`.
- `MutationAck` echoes `request_id` and authoritative `BlockCell`.

## Not Implemented

- Movement prediction.
- Camera prediction.
- Position correction.
- Inventory reconciliation.
- Combat reconciliation.

## Activation Requirements

- Define client-owned predicted state.
- Define server correction messages.
- Define discard rules for predicted visuals.
- Add public protocol tests.

## Rule

- Prediction must be disposable when it exists.
