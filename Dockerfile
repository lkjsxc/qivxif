FROM rust:1.91-slim AS build

WORKDIR /workspace
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    pkg-config \
    libssl-dev \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*
COPY . .
RUN npm --prefix apps/qivxif-web run build
RUN cargo build --locked --release -p qivxif-server

FROM debian:bookworm-slim

WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /workspace/target/release/qivxif-server /app/qivxif-server
COPY --from=build /workspace/apps/qivxif-web/dist /app/static
ENV QIVXIF_BIND=0.0.0.0:8080
ENV QIVXIF_DATA_DIR=/data
ENV QIVXIF_DATABASE_FILE=/data/qivxif.redb
ENV QIVXIF_STATIC_DIR=/app/static
EXPOSE 8080
ENTRYPOINT ["/app/qivxif-server"]
