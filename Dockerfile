# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml file first (for better caching)
COPY Cargo.toml ./

# Create a dummy src/main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this step will be cached if Cargo.toml doesn't change)
RUN cargo build --release
RUN rm src/main.rs

# Copy the actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies and debugging tools
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libc6 \
    libgcc-s1 \
    file \
    strace \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/enigma /usr/local/bin/enigma_cli

# Copy the assets directory and permutations.yaml file
COPY assets /assets
COPY permutations.yaml /permutations.yaml

# Create the empty print directory
RUN mkdir -p /print

# Make sure the binary is executable
RUN chmod +x /usr/local/bin/enigma_cli

# Verify the binary exists and check its dependencies
RUN ls -la /usr/local/bin/enigma_cli
RUN ldd /usr/local/bin/enigma_cli || echo "Static binary or ldd not available"
RUN file /usr/local/bin/enigma_cli

# Verify all required files and directories exist
RUN ls -la /assets/
RUN ls -la /permutations.yaml
RUN ls -la /print/

# Set working directory where assets are expected
WORKDIR /

# Set the default command
ENTRYPOINT ["/usr/local/bin/enigma_cli"]

# Default arguments (can be overridden when running the container)
CMD []