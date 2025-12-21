# Multi-stage Dockerfile for Rust Pokemon Application
# Stage 1: Build dependencies (cached layer)
FROM rust:1.75-slim AS builder-deps

WORKDIR /usr/src/app

# Install required build dependencies
RUN apt-get update && \
  apt-get install -y pkg-config libssl-dev && \
  rm -rf /var/lib/apt/lists/*

# Copy only dependency manifests to cache dependencies
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && \
  echo "fn main() {}" > src/main.rs && \
  cargo build --release && \
  rm -rf src

# Stage 2: Build application
FROM rust:1.75-slim AS builder

WORKDIR /usr/src/app

# Install required build dependencies
RUN apt-get update && \
  apt-get install -y pkg-config libssl-dev && \
  rm -rf /var/lib/apt/lists/*

# Copy pre-built dependencies from previous stage
COPY --from=builder-deps /usr/src/app/target target
COPY --from=builder-deps /usr/local/cargo /usr/local/cargo

# Copy source code
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# Build the application in release mode
RUN cargo build --release && \
  strip target/release/pokemon

# Stage 3: Runtime image (minimal)
FROM debian:bookworm-slim

# Install runtime dependencies (if needed)
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

# Expose the port
EXPOSE 3000

# Set the binary as entrypoint
ENTRYPOINT ["./pokemon"]
