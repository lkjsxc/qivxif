FROM rust:1.91-slim AS build
WORKDIR /workspace
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config ca-certificates && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --locked --release --bin qivxif-serverd

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /workspace/target/release/qivxif-serverd /usr/local/bin/qivxif-serverd
ENTRYPOINT ["qivxif-serverd"]
