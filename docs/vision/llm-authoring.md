# LLM Authoring

## Intent

Optimize the repository for future agents that need fast, accurate context.
Prefer explicit ownership, short files, stable headings, and verifiable claims.

## Format

- Start each directory with `README.md`.
- Keep each Markdown file under 300 lines.
- Keep files short and focused on one owner topic.
- Use stable headings that match the directory purpose.
- Link nearby owner docs instead of duplicating contracts.
- Use tables for indexes and decision matrices.
- Mark research-derived content by its durable owner, not by raw report wording.

## Change Behavior

1. Read the owner doc.
2. Update docs to the desired behavior.
3. Implement the smallest coherent batch.
4. Add or update verification.
5. Commit after the gate passes.

## Avoid

- Long prose blocks.
- Hidden assumptions in code comments.
- Multiple docs that define the same rule.
- Named product-line wording.
- Speculative content copied from reports without an owner-doc decision.
