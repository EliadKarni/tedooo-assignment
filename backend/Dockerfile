# Use the official Rust image as the base
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and source code
COPY Cargo.toml Cargo.lock ./
RUN mkdir src
COPY src ./src

# Build the application
RUN cargo build --release

# Expose the port the app runs on
EXPOSE 8080

# Run the application
CMD ["./target/release/backend"]