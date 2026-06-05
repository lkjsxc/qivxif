# Multi-User

## Roles

- owner
- admin
- member
- guest
- public

## Creation

- The first owner-admin is created through `qivxifctl`.
- Additional records are created through `qivxifctl admin create-user` or accepted invite codes.
- Omitted create-user roles produce a member account.
- Public is a viewer role and is not written as a stored user role by the CLI.
- Public signup is unavailable; invite acceptance is the documented registration path.

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
