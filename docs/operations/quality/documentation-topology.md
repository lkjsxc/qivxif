# Documentation Topology

Owner doc for docs tree shape.

## Directory Rule

Every directory under `docs/` must contain:

- exactly one `README.md`
- at least two child entries besides `README.md`
- links in `README.md` to each immediate Markdown file and child directory

## Root Rule

`docs/README.md` must include a compact recursive map that mentions every Markdown file under `docs/`.

## Reason

LLM agents need deterministic entry points and short owner files.
