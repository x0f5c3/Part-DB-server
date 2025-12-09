# Configuration

Part-DB is configured primarily through environment variables. This page documents all available configuration options.

## Environment Variables

### Database Configuration

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `DATABASE_URL` | Database connection string | *required* | `postgres://user:pass@localhost/partdb` |
| `DB_MAX_CONNECTIONS` | Maximum database connections | `10` | `20` |
| `DB_CONNECT_TIMEOUT` | Connection timeout in seconds | `30` | `60` |

### Server Configuration

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `HOST` | Server bind address | `0.0.0.0` | `127.0.0.1` |
| `PORT` | Server port | `3000` | `8080` |
| `SKIP_DB` | Skip database connection (health check only) | *not set* | `1` |

### Authentication

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `JWT_SECRET` | Secret key for JWT tokens | *auto-generated* | `your-secret-key` |
| `JWT_EXPIRY` | Token expiry in seconds | `86400` | `3600` |

## Configuration Files

### Backend (.env.local)

```env
# Database
DATABASE_URL=postgres://partdb:password@localhost/partdb
DB_MAX_CONNECTIONS=10
DB_CONNECT_TIMEOUT=30

# Server
HOST=0.0.0.0
PORT=3000

# Authentication
JWT_SECRET=your-super-secret-key-here
JWT_EXPIRY=86400
```

### Frontend (next.config.js)

The frontend configuration is in `frontend/next.config.js`:

```javascript
/** @type {import('next').NextConfig} */
const nextConfig = {
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: 'http://localhost:3000/api/:path*',
      },
    ];
  },
};

module.exports = nextConfig;
```

## Database Configuration

### PostgreSQL

```env
DATABASE_URL=postgres://username:password@host:port/database
```

Full connection string options:

```env
DATABASE_URL=postgres://username:password@host:port/database?sslmode=require
```

### MySQL

```env
DATABASE_URL=mysql://username:password@host:port/database
```

### SQLite

```env
DATABASE_URL=sqlite:./data/partdb.db
```

## Production Configuration

For production deployments, ensure:

1. **Use strong secrets**: Generate secure values for `JWT_SECRET`
2. **Use SSL**: Enable SSL for database connections
3. **Limit connections**: Set appropriate `DB_MAX_CONNECTIONS`
4. **Bind to localhost**: Use `HOST=127.0.0.1` when behind a reverse proxy

Example production configuration:

```env
# Production database with SSL
DATABASE_URL=postgres://partdb:secure-password@db.example.com:5432/partdb?sslmode=require
DB_MAX_CONNECTIONS=20
DB_CONNECT_TIMEOUT=10

# Server (behind reverse proxy)
HOST=127.0.0.1
PORT=3000

# Security
JWT_SECRET=generated-64-character-secret-key
JWT_EXPIRY=3600
```

## Reverse Proxy Configuration

When running behind a reverse proxy (nginx, Caddy, etc.), additional configuration may be required.

### Nginx Example

```nginx
server {
    listen 80;
    server_name partdb.example.com;

    location / {
        proxy_pass http://127.0.0.1:3001;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /api {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

See [Production Deployment](../deployment/production.md) for complete production setup instructions.
