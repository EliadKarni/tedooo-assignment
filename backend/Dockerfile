# Multi-stage Dockerfile for Rust backend

# Stage 1: Build
FROM rust:1.72 as builder
WORKDIR /app

# Copy source code and Cargo files
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/backend /app/backend

# Set the entrypoint
CMD ["/app/backend"]