# Multi-User

## Roles

- owner
- admin
- member
- guest
- public

## Creation

- The first owner-admin is created through `qivxifctl`.
- Additional records are created through `qivxifctl admin create-user`.
- Omitted create-user roles produce a member account.
- Public is a viewer role and is not written as a stored user role by the CLI.
- Public signup is not a route until documented in the auth route table.

## Visibility

- private
- shared
- unlisted
- public

## ACL Actions

- read
- write
- link
- publish
- moderate
- administer
