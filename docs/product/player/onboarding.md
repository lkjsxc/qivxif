# Onboarding

## Scope

This file owns the first-session path from sanctuary to starter basin.

## Flow

1. Player joins sanctuary.
2. Player selects or receives a starter basin.
3. Server verifies safe destination.
4. Player receives starter resources.
5. Player can travel into the open frontier.

## Failure

If safe placement fails, the server reports a clear retryable failure and keeps
the player in sanctuary.

## Cross-References

- Sanctuary and starter basin zone behavior is defined in [../world/zones.md](../world/zones.md).
- Starter basin placement rules are defined in [../world/starter-basins.md](../world/starter-basins.md).
- Starter resources connect to [../gameplay/inventory-crafting.md](../gameplay/inventory-crafting.md).
