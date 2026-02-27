# Build stage
FROM rust:latest as builder

WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the binary (release mode)
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

# Install curl for health checks
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/polymarket-weather-bot /app/polymarket-weather-bot

# Make binary executable
RUN chmod +x /app/polymarket-weather-bot

# Health check
HEALTHCHECK --interval=60s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Set environment variables for logging
ENV RUST_LOG=info

# Run the bot
CMD ["/app/polymarket-weather-bot"]
