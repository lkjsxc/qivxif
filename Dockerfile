FROM rust:1.91-slim

WORKDIR /workspace
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    ca-certificates \
    libx11-dev \
    libxi-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxinerama-dev \
    libxkbcommon-dev \
    libxkbcommon-x11-0 \
    libwayland-dev \
    libvulkan-dev \
    mesa-vulkan-drivers \
    libgl1-mesa-dri \
    xvfb \
    xauth \
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --locked --release -p qivxif-superapp
ENTRYPOINT ["./target/release/qivxif-superapp"]
