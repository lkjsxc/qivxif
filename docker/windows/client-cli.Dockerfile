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
    -p qivxif-client-cli

FROM debian:bookworm-slim AS artifact
COPY --from=build \
    /workspace/target/x86_64-pc-windows-gnu/release/qivxif-client-cli.exe \
    /qivxif-client-cli.exe
ENTRYPOINT ["sh", "-c", "cp /qivxif-client-cli.exe /dist/qivxif-client-cli.exe && ls -l /dist/qivxif-client-cli.exe"]
