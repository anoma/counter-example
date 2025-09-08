# Use NVIDIA CUDA base image with Ubuntu 22.04
FROM nvidia/cuda:12.2-devel-ubuntu22.04

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV CUDA_VISIBLE_DEVICES=all
ENV NVIDIA_VISIBLE_DEVICES=all
ENV NVIDIA_DRIVER_CAPABILITIES=compute,utility

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    libssl-dev \
    pkg-config \
    unzip \
    wget \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Rust with RISC-V target support
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add riscv32im-risc0-zkvm-elf

# Install RISC Zero toolchain
RUN cargo install cargo-binstall
RUN cargo binstall --no-confirm cargo-risczero
RUN cargo risczero install

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy native Rust code
COPY native/ ./native/

# Build Rust components
RUN cargo build --release

# Set the default command
CMD ["/bin/bash"]