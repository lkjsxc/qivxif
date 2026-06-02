# Test Overview

## Layers

- Crate unit tests run inside `cargo test --workspace`.
- API integration tests run in the `api-test` Compose service.
- Browser offline tests run in the `offline-e2e` Compose service under [offline/](offline/README.md).

## Acceptance

Completed slices pass `docker compose -f docker-compose.verify.yml run --rm verify`
and the dynamic Compose services documented in
[../docs/operations/verification/compose-pipeline.md](../docs/operations/verification/compose-pipeline.md).
