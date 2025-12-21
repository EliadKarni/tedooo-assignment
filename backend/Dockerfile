# Multi-stage Dockerfile for a Rust backend:
# - builder stage compiles the binary
# - runtime stage runs the compiled binary (smaller image)
# Note: builder + runtime use the same Debian release to avoid GLIBC issues.

FROM rustlang/rust:nightly-bullseye AS builder
WORKDIR /app

# Copy manifests first for better Docker layer caching
COPY . .

# Build the application with nightly toolchain (for edition2024)
RUN rustup default nightly && cargo build --release

FROM debian:bullseye-slim
WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/backend /app/backend

# Run the backend
CMD ["/app/backend"]