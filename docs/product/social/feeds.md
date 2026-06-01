# Feeds

## Feed Types

- home
- user
- mentions
- replies
- tag
- local public
- bookmarks

## Rules

- Default order is chronological and explainable.
- Feed indexes are derived from graph operations.
- Feed queries must not scan all posts.
- The first home feed returns accepted posts indexed for the signed-in user.
- Feed item creation happens in the same transaction as post acceptance.
