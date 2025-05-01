# Stage 1: Build
FROM rust:latest as builder

WORKDIR /app/nft_marketplace_backend

# Copy the source code into the container
COPY . .

# Build the project
RUN cargo build --release

# Stage 2: Final image
FROM debian:bookworm-slim

# Create necessary directories
RUN mkdir -p /usr/local/bin

# Set environment variables
ENV DATABASE_URL=postgres://postgres:nft@172.17.0.1/postgres
ENV HOST=0.0.0.0
ENV PORT=8080

# Copy the release binary from the builder
COPY --from=builder /app/nft_marketplace_backend/target/release/nft_marketplace_backend /usr/local/bin/nft_marketplace_backend

# Add any other files, like static files or migrations
# COPY --from=builder /app/nft_marketplace_backend/migrations /app/migrations
EXPOSE 8080
# Set the default entrypoint
CMD ["/usr/local/bin/nft_marketplace_backend"]
