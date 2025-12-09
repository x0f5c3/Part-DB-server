# Docker Installation

Docker is the recommended way to run Part-DB in production. This guide walks you through setting up Part-DB using Docker and Docker Compose.

## Prerequisites

- Docker Engine 20.10+
- Docker Compose v2.0+

## Quick Start

1. Clone the repository:

```bash
git clone https://github.com/x0f5c3/Part-DB-server.git
cd Part-DB-server
```

2. Create a `.env` file with your configuration:

```bash
cp .env.docker .env.local
```

3. Edit `.env.local` with your settings:

```env
# Database configuration
DATABASE_URL=postgres://partdb:partdb@db/partdb

# Application settings
HOST=0.0.0.0
PORT=3000
```

4. Start the containers:

```bash
docker-compose up -d
```

5. Access Part-DB at `http://localhost:3000`

## Docker Compose Configuration

The default `docker-compose.yml` includes:

- **backend**: Rust API server
- **frontend**: Next.js web application
- **db**: PostgreSQL database

Example `docker-compose.yml`:

```yaml
version: '3.8'

services:
  backend:
    build:
      context: .
      dockerfile: Dockerfile
    environment:
      - DATABASE_URL=postgres://partdb:partdb@db/partdb
      - HOST=0.0.0.0
      - PORT=3000
    ports:
      - "3000:3000"
    depends_on:
      - db

  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile
    ports:
      - "3001:3000"
    depends_on:
      - backend

  db:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: partdb
      POSTGRES_PASSWORD: partdb
      POSTGRES_DB: partdb
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

volumes:
  postgres_data:
```

## Customization

### Using a Different Database

To use MySQL instead of PostgreSQL:

```yaml
db:
  image: mysql:8
  environment:
    MYSQL_ROOT_PASSWORD: root
    MYSQL_DATABASE: partdb
    MYSQL_USER: partdb
    MYSQL_PASSWORD: partdb
  volumes:
    - mysql_data:/var/lib/mysql
```

Update `DATABASE_URL` accordingly:

```env
DATABASE_URL=mysql://partdb:partdb@db/partdb
```

### Persistent Storage

Ensure your data persists by using volumes:

```yaml
volumes:
  postgres_data:
  uploads:
```

Mount uploads:

```yaml
backend:
  volumes:
    - uploads:/app/uploads
```

## Production Considerations

For production deployments:

1. **Use a reverse proxy** (nginx, Caddy) for TLS termination
2. **Set secure passwords** for the database
3. **Configure backups** for the database volume
4. **Set resource limits** on containers
5. **Use Docker secrets** for sensitive configuration

See [Production Deployment](../deployment/production.md) for more details.
