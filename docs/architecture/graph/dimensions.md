# Graph Dimensions

## Purpose

Dimensions are typed projection lenses over graph data. They filter and annotate
visible nodes and edges without deleting or rewriting durable records.

## Dimension Sources

- Edge kind.
- Node kind.
- Tag or topic edges.
- ACL projection.
- Time range from accepted event metadata.
- Author profile.
- Media type.
- Profile relationships.
- Text references and backlinks.
- Publication relationships.
- System and sync relationships.

## Rules

- Toggling a dimension changes only the current projection.
- Disabled dimensions never tombstone edges.
- ACL filtering always runs before visual dimension filtering.
- Dirty local edges can appear with dirty status if both endpoints are visible.
- Server projections omit hidden nodes and omit edges whose endpoints are hidden.

## Projection Shape

```typescript
type GraphDimension = {
  id: string;
  label: string;
  source: "edge_kind" | "node_kind" | "tag" | "acl" | "time" | "author" | "media" | "profile" | "text" | "publication" | "system";
  enabled: boolean;
};
```

## Diagnostics

Graph Map diagnostics list active dimensions, hidden counts when known, query
bounds, layout status, and ACL omission count when the server can report it.
