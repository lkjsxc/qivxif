# Feed Surface

## Layout

```text
+------------------------------------------+
| compose bar (textarea + submit)          |
+------------------------------------------+
| [av] Author · 2h              [···]    |
|      Post body text wraps here.          |
|      [Reply] [Boost] (when applicable)   |
+------------------------------------------+
| empty state or more cards...             |
+------------------------------------------+
```

## Compose Bar

- Sticky at top of feed pane body, not shell header.
- Textarea grows up to 4 rows then scrolls internally.
- Submit disabled when body is empty or offline without queue.
- Primary submit uses accent fill; label "Post" or "Create short post".

## Post Card

- `.feed-card` border `1px solid --q-border`, radius `--radius-panel`.
- Author row: initials chip 32px, display name `--text-base`, handle `--q-muted`.
- Timestamp `--q-faint` `--text-xs`, right-aligned or after handle.
- Body `--text-base`, `overflow-wrap: anywhere`.
- Relationship actions stay compact icon buttons or text buttons in a row.

## Profile Targets

- Section below compose when authenticated.
- Each row: label, Follow, Mute, Block as separate hit targets.
- Empty: "No discovered profile targets."

## Empty States

- No posts: explain home feed indexing and follow graph.
- Signed out: link to login tab through pane context, not full-page redirect.

## Tests

- Playwright finds compose textarea and submit by role.
- Feed cards use `article.feed-card` with visible author text from API data.
- No fake lorem content on the main path.
