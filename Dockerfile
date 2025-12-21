# Multi-stage Dockerfile for Rust Pokemon Application
# Using Rust 1.92 (latest stable)

# Stage 1: Build the application
FROM rust:1.92-slim AS builder

WORKDIR /usr/src/app

# Install required build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release && \
    strip target/release/pokemon

# Stage 2: Runtime image (minimal)
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 appuser

WORKDIR /app

# Copy only the binary from builder
COPY --from=builder /usr/src/app/target/release/pokemon .

# Change ownership to non-root user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Environment variables with defaults
ENV HOST=0.0.0.0
ENV PORT=3000
ENV POKE_API_BASE_URL=https://pokeapi.co/api/v2
ENV FUN_TRANSLATIONS_BASE_URL=https://api.funtranslations.com

# Expose the port
EXPOSE 3000

# Set the binary as entrypoint
ENTRYPOINT ["./pokemon"]
