# Runtime Config

## Fields

- `bind_addr`: socket address for QUIC.
- `data_dir`: durable state directory.
- `world_seed`: deterministic world seed.
- `build_epoch`: build gate.
- `protocol_epoch`: wire gate.

## Rule

Config must be explicit. Runtime defaults belong in owner docs and tests.
