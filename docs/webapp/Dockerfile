# Deployment Dockerfile for Anki Web App
# This expects pre-built artifacts from ./build-webapp.sh --release

# Production runtime using Debian for better compatibility with glibc-linked binaries
FROM debian:bookworm-slim

# Install nginx and required runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    nginx \
    ca-certificates \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Copy the built API binary (built on host)
COPY target/release/anki-webapp /usr/local/bin/

# Copy the built frontend assets (built on host)
COPY out/sveltekit /var/www/html

# Copy nginx configuration
# We use a full nginx.conf structure since we're replacing the main config
COPY --chmod=644 <<'EOF' /etc/nginx/nginx.conf
user www-data;
worker_processes auto;
pid /run/nginx.pid;
include /etc/nginx/modules-enabled/*.conf;

events {
    worker_connections 768;
}

http {
    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;
    types_hash_max_size 2048;

    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    # Ensure .mjs files are served with the correct MIME type
    types {
        application/javascript mjs;
    }

    access_log /var/log/nginx/access.log;
    error_log /var/log/nginx/error.log;

    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css text/xml text/javascript 
               application/x-javascript application/xml+rss 
               application/json application/javascript;

    server {
        listen 80;
        server_name localhost;
        root /var/www/html;
        index index.html;

        # Serve static files
        location / {
            try_files $uri $uri/ /index.html;
        }

        # Proxy API requests to the backend
        location /api/ {
            proxy_pass http://127.0.0.1:8080;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection 'upgrade';
            proxy_set_header Host $host;
            proxy_cache_bypass $http_upgrade;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # Health check
        location /health {
            access_log off;
            return 200 'OK';
        }
    }
}
EOF

# Create data directory for user collections
RUN mkdir -p /data && chown www-data:www-data /data && chmod 777 /data

# Expose ports
EXPOSE 80 8080

# Start script to run both nginx and the API
COPY --chmod=755 <<'EOF' /start.sh
#!/bin/sh
# Start the API in the background
# We bind the API to 0.0.0.0 so it's accessible from the host mapping
ANKI_WEBAPP_HOST=0.0.0.0 \
ANKI_WEBAPP_PORT=8080 \
ANKI_WEBAPP_DATA_DIR=/data \
RUST_LOG=info \
/usr/local/bin/anki-webapp &

# Start nginx in the foreground
nginx -g 'daemon off;'
EOF

CMD ["/start.sh"]
