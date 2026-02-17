# Multi-stage Dockerfile for production deployment
# This builds both the API and frontend in a single container with nginx

# Stage 1: Build the Rust API
FROM rust:1.75 AS api-builder

WORKDIR /build

# Copy only dependency files first for better caching
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY rslib/Cargo.toml rslib/
COPY build/Cargo.toml build/
COPY proto/ proto/

# Create dummy source files to build dependencies
RUN mkdir -p rslib/src build/src && \
    echo "fn main() {}" > rslib/src/main.rs && \
    echo "fn main() {}" > build/src/main.rs

# Build dependencies
RUN cargo build --release --bin anki-api || true

# Now copy the real source code
COPY rslib/ rslib/
COPY build/ build/

# Build the actual API
RUN cargo build --release --bin anki-api

# Stage 2: Build the Svelte frontend
FROM node:20 AS frontend-builder

WORKDIR /build

# Copy package files
COPY package.json yarn.lock ./

# Install dependencies
RUN yarn install --frozen-lockfile

# Copy source code
COPY ts/ ts/
COPY tsconfig.json vite.config.ts ./

# Build the frontend
RUN yarn build

# Stage 3: Production runtime with nginx
FROM nginx:alpine

# Install required runtime dependencies
RUN apk add --no-cache ca-certificates

# Copy the built API binary
COPY --from=api-builder /build/target/release/anki-api /usr/local/bin/

# Copy the built frontend
COPY --from=frontend-builder /build/dist /usr/share/nginx/html

# Copy nginx configuration
COPY docs/webapp/nginx.conf /etc/nginx/nginx.conf

# Create data directory for SQLite databases
RUN mkdir -p /data && chmod 777 /data

# Expose ports
EXPOSE 80 8080

# Start script to run both nginx and the API
COPY --chmod=755 <<'EOF' /start.sh
#!/bin/sh
# Start the API in the background
RUST_LOG=info /usr/local/bin/anki-api &
# Start nginx in the foreground
nginx -g 'daemon off;'
EOF

CMD ["/start.sh"]
