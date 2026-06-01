# Setup Session

## Result

Successful setup returns the same browser state as a successful login:

- signed-in owner user summary
- CSRF token for authenticated mutations
- HttpOnly session cookie
- next actor sequence cursor

## Rules

- The browser stores the returned auth payload through the login storage path.
- The sync actor starts only after setup state allows authenticated actions.
- The Setup tab disappears after owner creation succeeds.
- Reloaded clients use `/api/me` and local auth state in the same way as login.
