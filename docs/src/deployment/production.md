# Production Deployment

This guide covers deploying Part-DB in a production environment.

## Deployment Options

1. **Docker** (Recommended) - Easy deployment with containers
2. **Manual** - Direct installation on a server
3. **Cloud Platforms** - Deploy to cloud services

## Docker Deployment

### Prerequisites

- Docker Engine 20.10+
- Docker Compose v2.0+
- A domain name (optional, for HTTPS)

### Setup

1. Clone the repository:

```bash
git clone https://github.com/x0f5c3/Part-DB-server.git
cd Part-DB-server
```

2. Create production configuration:

```bash
cp .env.docker .env.production
```

3. Edit `.env.production`:

```env
# Database
DATABASE_URL=postgres://partdb:secure-password-here@db/partdb

# Server
HOST=0.0.0.0
PORT=3000

# Security
JWT_SECRET=generate-a-secure-random-string-here

# Database pool
DB_MAX_CONNECTIONS=20
DB_CONNECT_TIMEOUT=10
```

4. Create `docker-compose.prod.yml`:

```yaml
version: '3.8'

services:
  backend:
    build: .
    env_file: .env.production
    restart: always
    depends_on:
      - db

  frontend:
    build: ./frontend
    environment:
      - NEXT_PUBLIC_API_URL=http://backend:3000
    restart: always
    depends_on:
      - backend

  db:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: partdb
      POSTGRES_PASSWORD: secure-password-here
      POSTGRES_DB: partdb
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: always

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./certs:/etc/nginx/certs
    depends_on:
      - frontend
      - backend
    restart: always

volumes:
  postgres_data:
```

5. Start the services:

```bash
docker-compose -f docker-compose.prod.yml up -d
```

## Nginx Configuration

Create `nginx.conf`:

```nginx
events {
    worker_connections 1024;
}

http {
    upstream frontend {
        server frontend:3000;
    }

    upstream backend {
        server backend:3000;
    }

    server {
        listen 80;
        server_name partdb.example.com;

        # Redirect HTTP to HTTPS
        return 301 https://$server_name$request_uri;
    }

    server {
        listen 443 ssl;
        server_name partdb.example.com;

        ssl_certificate /etc/nginx/certs/fullchain.pem;
        ssl_certificate_key /etc/nginx/certs/privkey.pem;

        # Frontend
        location / {
            proxy_pass http://frontend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # Backend API
        location /api {
            proxy_pass http://backend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        location /health {
            proxy_pass http://backend;
        }
    }
}
```

## Security Checklist

- [ ] Use strong, unique passwords for database
- [ ] Generate a secure `JWT_SECRET` (32+ random characters)
- [ ] Enable HTTPS with valid SSL certificates
- [ ] Configure firewall to only expose ports 80/443
- [ ] Set up regular database backups
- [ ] Keep Docker images updated
- [ ] Monitor logs for suspicious activity

## Database Backups

### Automated Backups

Add a backup service to docker-compose:

```yaml
backup:
  image: postgres:16-alpine
  volumes:
    - ./backups:/backups
  environment:
    PGPASSWORD: secure-password-here
  command: >
    sh -c "while true; do
      pg_dump -h db -U partdb partdb > /backups/backup-$$(date +%Y%m%d-%H%M%S).sql
      find /backups -type f -mtime +7 -delete
      sleep 86400
    done"
  depends_on:
    - db
```

### Manual Backup

```bash
docker-compose exec db pg_dump -U partdb partdb > backup.sql
```

### Restore

```bash
cat backup.sql | docker-compose exec -T db psql -U partdb partdb
```

## Monitoring

### Health Checks

The backend exposes a health endpoint:

```
GET /health
```

Configure your monitoring tool to check this endpoint.

### Logs

View logs:

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f backend
```

## Scaling

For high-traffic deployments:

```yaml
backend:
  deploy:
    replicas: 3
```

Use a load balancer in front of the backend instances.

## Updates

To update to a new version:

```bash
# Pull latest changes
git pull

# Rebuild and restart
docker-compose -f docker-compose.prod.yml up -d --build
```

## Troubleshooting

### Container Won't Start

```bash
# Check logs
docker-compose logs backend

# Check container status
docker-compose ps
```

### Database Connection Issues

```bash
# Test connection
docker-compose exec backend sh -c 'psql $DATABASE_URL -c "SELECT 1"'
```

### Performance Issues

- Increase `DB_MAX_CONNECTIONS`
- Add more container replicas
- Check database indexes
