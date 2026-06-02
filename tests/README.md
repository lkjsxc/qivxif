# Tests

Repository-level integration and browser checks outside crate unit tests.

## Index

- [overview.md](overview.md): how repository tests layer together.

## Suites

- [offline/](offline/README.md): Playwright headless checks against the built web app and live API.

## Rules

- Offline tests use real server paths, not stubbed APIs.
- Selectors prefer roles, labels, and documented `data-*` attributes.
- Each test directory has one `README.md` table of contents.

## Related Docs

- [../docs/operations/verification/compose-pipeline.md](../docs/operations/verification/compose-pipeline.md)
- [../docs/operations/verification/dynamic-gates.md](../docs/operations/verification/dynamic-gates.md)
