FROM rust:1.91-slim AS build

WORKDIR /workspace
RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc-mingw-w64-x86-64 \
    binutils-mingw-w64-x86-64 \
    pkg-config \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN rustup target add --toolchain 1.91.0 x86_64-pc-windows-gnu
RUN cargo build --locked --release \
    --target x86_64-pc-windows-gnu \
    --bin qivxif-serverd \
    --bin qivxif-client-cli

FROM debian:bookworm-slim AS artifact

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    zip \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /bundle
COPY --from=build \
    /workspace/target/x86_64-pc-windows-gnu/release/qivxif-serverd.exe \
    /bundle/qivxif-serverd.exe
COPY --from=build \
    /workspace/target/x86_64-pc-windows-gnu/release/qivxif-client-cli.exe \
    /bundle/qivxif-client-cli.exe

RUN mkdir -p config data
RUN printf '%s\n' \
    'bind_addr = "127.0.0.1:4443"' \
    'data_dir = "data"' \
    'world_seed = 1001' \
    'build_contract = "windows-demo"' \
    'protocol_contract = "postcard-reliable-streams"' \
    > config/server.toml
RUN : > data/.keep
RUN printf '%s\r\n' \
    '@echo off' \
    'cd /d "%~dp0"' \
    'qivxif-serverd.exe serve --config config/server.toml' \
    'pause' \
    > start-server.cmd
RUN printf '%s\r\n' \
    '@echo off' \
    'cd /d "%~dp0"' \
    'qivxif-client-cli.exe connect --addr 127.0.0.1:4443 --server-name localhost --tls local-compose --player demo --chunk-x 0 --chunk-z 0' \
    'pause' \
    > run-client-demo.cmd
RUN printf '%s\n' \
    '# qivxif Windows Demo Bundle' \
    '' \
    'This unsigned portable bundle is for internal demo smoke runs only.' \
    '' \
    '1. Run `start-server.cmd` and leave the server window open.' \
    '2. Run `run-client-demo.cmd` from a second window.' \
    '' \
    'The server binds to `127.0.0.1:4443` and stores local state under `data/`.' \
    > README.md
RUN printf '%s\n' \
    '{' \
    '  "name": "qivxif-demo-windows-x86_64",' \
    '  "target": "x86_64-pc-windows-gnu",' \
    '  "signed": false,' \
    '  "release_class": "internal-demo",' \
    '  "entrypoints": ["start-server.cmd", "run-client-demo.cmd"]' \
    '}' \
    > manifest.json
RUN find . -type f ! -name checksums.txt ! -name checksums.tmp \
    | sed 's#^\./##' \
    | sort \
    | xargs sha256sum > checksums.tmp \
    && mv checksums.tmp checksums.txt

ENTRYPOINT ["sh", "-c", "rm -rf /dist/windows/demo /dist/windows/qivxif-demo-windows-x86_64.zip && mkdir -p /dist/windows && cp -a /bundle /dist/windows/demo && cd /dist/windows && zip -qr qivxif-demo-windows-x86_64.zip demo && find demo -type f | sort"]
