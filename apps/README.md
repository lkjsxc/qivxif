# Apps

Deployable application packages served or shipped with qivxif.

## Index

- [overview.md](overview.md): apps layout and build entry.

## Packages

- [qivxif-web](qivxif-web/README.md): SvelteKit browser client, service worker, and static assets.

## Rules

- Browser UI follows [../docs/architecture/client/README.md](../docs/architecture/client/README.md).
- Build output is `dist/` and is served by the Rust server through `QIVXIF_STATIC_DIR`.
- Each app directory has one `README.md` table of contents.

## Related Docs

- [../docs/repository/layout/web-layout.md](../docs/repository/layout/web-layout.md)
