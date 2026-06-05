# Admin Flows

## CLI Commands

```text
qivxifctl admin issue-invite --store <path> --role member --expires-in 7d --max-uses 1
qivxifctl admin list-invites --store <path>
qivxifctl admin revoke-invite --store <path> --invite-id <id>
qivxifctl admin issue-key --store <path> --user <name-or-id> --scope graph.read --scope media.write --expires-in 30d
qivxifctl admin list-keys --store <path> --user <name-or-id>
qivxifctl admin revoke-key --store <path> --key-id <id>
qivxifctl admin audit-keys --store <path>
```

## UI Flows

- Owner opens Admin tab.
- Owner issues invite and copies the secret once.
- Invitee accepts code and creates an account.
- Admin issues scoped API token for a user or automation.
- Admin revokes invite or key.
- Admin inspects audit trail.

## Security

- Scope checks happen server-side.
- Cookie-authenticated admin mutations require CSRF.
- Login-like and invite acceptance paths are rate-limited.
- Expired and revoked keys fail closed.
