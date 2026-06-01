# First Run

## Rule

An empty qivxif data store is configured from `/` inside the normal app shell.

## Behavior

- If no durable user exists, the app shell opens with a Setup tab.
- Setup is a tab, not a separate landing page.
- Setup creates the first owner account.
- Once any user exists, owner creation is closed until the data store is reset.
- Setup never creates sample data.
- Setup never bypasses redb durability.
- Setup uses the same session cookie and CSRF model as login.

## User Flow

1. The browser opens `/`.
2. The header and tile frame render immediately.
3. The client calls `GET /api/setup`.
4. When setup is required, the active tab becomes Setup.
5. The user enters Name and Password.
6. The client sends `POST /api/setup/owner`.
7. The server creates the first owner account.
8. The response returns a normal authenticated session and CSRF token.
9. The shell activates a normal product tab without leaving `/`.

## Constraints

- Setup requires a non-empty trimmed name.
- Setup requires a non-empty password with a minimum length.
- Setup cannot be repeated after the first durable user exists.
- Setup commands that need authentication stay hidden or disabled until setup succeeds.
