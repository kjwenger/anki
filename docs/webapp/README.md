# Anki Webapp - Docker Deployment Guide

This directory contains Docker configuration files for deploying the Anki web application.

## Architecture

The Anki webapp consists of two main services:

1. **API Backend** (Rust) - RESTful API server handling authentication, collections, decks, cards, etc.
2. **Frontend** (Svelte) - Single-page application providing the user interface

## Files

- `Dockerfile.api` - Builds the Rust backend API server
- `Dockerfile.frontend` - Builds and serves the Svelte frontend
- `docker-compose.yml` - Orchestrates both services
- `nginx.conf` - Nginx configuration for serving the frontend and proxying API requests
- `README.md` - This file

## Prerequisites

- Docker 20.10 or later
- Docker Compose 2.0 or later
- 2GB free disk space (for images and data)

## Quick Start

### 1. Build and Run

From the repository root directory:

```bash
cd docs/webapp
docker-compose up -d
```

This will:
- Build both the API and frontend Docker images
- Start the services in the background
- Create a persistent volume for user data

### 2. Access the Application

Open your browser and navigate to:
- **Frontend**: http://localhost
- **API**: http://localhost:8080

### 3. Create an Account

1. Click "Register" on the login page
2. Create your username and password
3. Login and start using Anki!

## Configuration

### Environment Variables

Create a `.env` file in the `docs/webapp` directory to customize settings:

```env
# JWT secret for authentication (CHANGE THIS IN PRODUCTION!)
JWT_SECRET=your-super-secret-key-change-this

# API configuration
RUST_LOG=info
DATABASE_URL=/app/data/anki.db

# CORS allowed origins
ALLOWED_ORIGINS=http://localhost,http://yourdomain.com
```

### Ports

By default:
- Frontend: `80` (HTTP)
- API: `8080`

To change ports, edit `docker-compose.yml`:

```yaml
services:
  frontend:
    ports:
      - "3000:80"  # Use port 3000 instead
  
  api:
    ports:
      - "8081:8080"  # Use port 8081 instead
```

## Data Persistence

User data (collections, decks, cards, media) is stored in a Docker volume named `anki-data`.

### Backup Data

```bash
# Create a backup
docker run --rm -v anki-data:/data -v $(pwd):/backup \
  alpine tar czf /backup/anki-backup-$(date +%Y%m%d).tar.gz -C /data .
```

### Restore Data

```bash
# Restore from backup
docker run --rm -v anki-data:/data -v $(pwd):/backup \
  alpine sh -c "cd /data && tar xzf /backup/anki-backup-20260216.tar.gz"
```

## Development Mode

For development with hot-reload, use the local development servers instead:

### Backend (API)

```bash
cd rslib/webapp
cargo run
```

### Frontend (Svelte)

```bash
cd ts
yarn dev
```

Then access:
- Frontend: http://localhost:5173
- API: http://localhost:8080

## Production Deployment

### Security Considerations

⚠️ **Before deploying to production:**

1. **Change JWT Secret**: Set a strong, random `JWT_SECRET` in `.env`
   ```bash
   JWT_SECRET=$(openssl rand -hex 32)
   ```

2. **Use HTTPS**: Set up a reverse proxy (nginx/Traefik) with SSL certificates
   ```bash
   # Example with Let's Encrypt
   docker run -d -p 80:80 -p 443:443 \
     -v /path/to/certs:/etc/letsencrypt \
     nginx
   ```

3. **Configure CORS**: Update `ALLOWED_ORIGINS` to only include your domain

4. **Limit Exposed Ports**: Only expose the frontend port to the internet

### Example Production docker-compose.yml

```yaml
version: '3.8'

services:
  api:
    build:
      context: ../..
      dockerfile: docs/webapp/Dockerfile.api
    expose:
      - "8080"  # Internal only
    environment:
      - JWT_SECRET=${JWT_SECRET}
      - RUST_LOG=warn
      - ALLOWED_ORIGINS=https://yourdomain.com
    restart: always

  frontend:
    build:
      context: ../..
      dockerfile: docs/webapp/Dockerfile.frontend
    expose:
      - "80"  # Behind reverse proxy
    restart: always

  # Reverse proxy with SSL
  nginx-proxy:
    image: nginx:alpine
    ports:
      - "443:443"
      - "80:80"
    volumes:
      - ./nginx-ssl.conf:/etc/nginx/conf.d/default.conf
      - /etc/letsencrypt:/etc/letsencrypt
    restart: always
```

## Monitoring

### Check Service Status

```bash
docker-compose ps
```

### View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f api
docker-compose logs -f frontend
```

### Health Checks

```bash
# API health
curl http://localhost:8080/health

# Frontend health
curl http://localhost
```

## Troubleshooting

### Services Won't Start

1. Check logs: `docker-compose logs`
2. Verify ports aren't in use: `netstat -tuln | grep -E '(80|8080)'`
3. Check disk space: `df -h`

### API Returns 500 Errors

1. Check API logs: `docker-compose logs api`
2. Verify database permissions
3. Check JWT_SECRET is set

### Frontend Shows Blank Page

1. Check nginx logs: `docker-compose logs frontend`
2. Verify API is running: `curl http://localhost:8080/health`
3. Check browser console for errors

### Cannot Login/Register

1. Check API logs for authentication errors
2. Verify JWT_SECRET is set correctly
3. Clear browser cookies and try again

## Stopping Services

```bash
# Stop services (keeps data)
docker-compose stop

# Stop and remove containers (keeps data)
docker-compose down

# Stop, remove containers, and delete data
docker-compose down -v
```

## Updating

```bash
# Pull latest changes
git pull

# Rebuild images
docker-compose build --no-cache

# Restart services
docker-compose down
docker-compose up -d
```

## Resource Requirements

### Minimum
- CPU: 1 core
- RAM: 512MB
- Disk: 1GB

### Recommended
- CPU: 2 cores
- RAM: 2GB
- Disk: 5GB

## Support

For issues or questions:
- GitHub Issues: https://github.com/kjwenger/anki/issues
- Documentation: See `README.webapp.md` in the repository root

## License

See LICENSE file in the repository root.
