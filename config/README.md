# Anki Web App Configuration

The Anki Web App supports multiple configuration methods with the following priority:

**Priority (highest to lowest):**

1. Environment Variables
2. Configuration File (`config/server.toml`)
3. Default Values

## Configuration Options

### Network Settings

- **`host`** - Server bind address
  - Default: `127.0.0.1`
  - ENV: `ANKI_WEBAPP_HOST`
  - Example: `0.0.0.0` (listen on all interfaces)

- **`port`** - Server port
  - Default: `8080`
  - ENV: `ANKI_WEBAPP_PORT`
  - Example: `8080`, `3000`

### Storage

- **`data_dir`** - Directory for user data and collections
  - Default: `./data`
  - ENV: `ANKI_WEBAPP_DATA_DIR`
  - Example: `/var/lib/anki-webapp`, `/home/user/anki-data`

### Security

- **`jwt_secret`** - JWT token signing secret
  - Default: `change-this-secret-in-production` ⚠️
  - ENV: `ANKI_WEBAPP_JWT_SECRET`
  - **IMPORTANT**: Must be changed in production!
  - Recommendation: Use a long random string (32+ characters)

- **`session_timeout_hours`** - Session expiration time
  - Default: `24` (hours)
  - ENV: `ANKI_WEBAPP_SESSION_TIMEOUT_HOURS`
  - Example: `168` (7 days), `720` (30 days)

## Usage

### Method 1: Environment Variables

```bash
export ANKI_WEBAPP_HOST="0.0.0.0"
export ANKI_WEBAPP_PORT="8080"
export ANKI_WEBAPP_DATA_DIR="/var/lib/anki-webapp"
export ANKI_WEBAPP_JWT_SECRET="your-very-long-random-secret-here"
export ANKI_WEBAPP_SESSION_TIMEOUT_HOURS="168"

./anki-webapp
```

### Method 2: Configuration File

1. Copy the example configuration:

```bash
cp config/server.toml.example config/server.toml
```

2. Edit `config/server.toml`:

```toml
host = "0.0.0.0"
port = 8080
data_dir = "/var/lib/anki-webapp"
jwt_secret = "your-very-long-random-secret-here"
session_timeout_hours = 168
```

3. Run the server:

```bash
./anki-webapp
```

### Method 3: Combine Both

You can use a config file for most settings and override specific ones with environment variables:

```bash
# config/server.toml has base settings
export ANKI_WEBAPP_PORT="9000"  # Override just the port
./anki-webapp
```

## Production Recommendations

### Security

1. **Always set a custom JWT secret:**

```bash
# Generate a random secret
openssl rand -base64 32

# Set it as environment variable
export ANKI_WEBAPP_JWT_SECRET="<generated-secret>"
```

2. **Use a long session timeout for convenience:**

```toml
session_timeout_hours = 720 # 30 days
```

3. **Bind to localhost if behind a reverse proxy:**

```toml
host = "127.0.0.1"
port = 8080
```

### Storage

1. **Use an absolute path for data_dir:**

```toml
data_dir = "/var/lib/anki-webapp"
```

2. **Ensure proper permissions:**

```bash
sudo mkdir -p /var/lib/anki-webapp
sudo chown anki-webapp:anki-webapp /var/lib/anki-webapp
sudo chmod 750 /var/lib/anki-webapp
```

### Deployment

1. **Systemd service example:**

```ini
[Unit]
Description=Anki Web App
After=network.target

[Service]
Type=simple
User=anki-webapp
WorkingDirectory=/opt/anki-webapp
Environment="ANKI_WEBAPP_JWT_SECRET=<your-secret>"
Environment="ANKI_WEBAPP_DATA_DIR=/var/lib/anki-webapp"
ExecStart=/opt/anki-webapp/anki-webapp
Restart=always

[Install]
WantedBy=multi-user.target
```

2. **Docker environment variables:**

```yaml
version: '3'
services:
  anki-webapp:
    image: anki-webapp:latest
    environment:
      - ANKI_WEBAPP_HOST=0.0.0.0
      - ANKI_WEBAPP_PORT=8080
      - ANKI_WEBAPP_JWT_SECRET=${JWT_SECRET}
      - ANKI_WEBAPP_DATA_DIR=/data
    volumes:
      - anki-data:/data
    ports:
      - "8080:8080"
```

## Validation

The server will validate configuration on startup and warn about:

- Using the default JWT secret in production
- Invalid host/port combinations
- Inaccessible data directories

Check the logs on startup:

```
INFO Server configuration:
INFO   Host: 127.0.0.1
INFO   Port: 8080
INFO   Data directory: ./data
WARN Using default JWT secret - please set ANKI_WEBAPP_JWT_SECRET in production!
```
