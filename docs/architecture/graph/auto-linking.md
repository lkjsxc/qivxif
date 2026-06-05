# Automatic Relationship Suggestions

## Purpose

Automatic linking proposes relationships. It does not silently create durable
edges until a policy doc explicitly allows a class of safe edges.

## Deterministic Sources

- Title and alias exact match.
- Explicit `[[wikilinks]]` in text.
- URLs shared by two nodes.
- Tags and topics.
- Profile mentions.
- Shared media content hash.
- Follow relationships between profiles.
- Same source import.
- Backlink text patterns.

## Suggestion Record

A suggestion contains:

- suggestion id.
- source node id.
- target node id.
- proposed edge kind.
- provenance.
- score.
- supporting text or metadata reference.
- created by subsystem id.
- user action state.

## Scoring

Scores are deterministic for the same inputs. Exact explicit links score higher
than fuzzy aliases. Shared media hash scores higher than shared tags.

## User Actions

- Accept writes a real `edge.create` event.
- Dismiss records that the suggestion should not reappear for the same inputs.
- Defer keeps it visible in suggestion surfaces.

## Safety

- Suggestions respect ACL.
- Suggestions never expose hidden node titles through provenance.
- A suggestion is not a relationship until the user or documented policy accepts it.
