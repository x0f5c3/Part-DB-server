# Installation

This page provides an overview of installation options for Part-DB.

## Installation Methods

### Docker (Recommended)

Docker is the easiest way to get Part-DB up and running. See [Docker Installation](./docker.md) for detailed instructions.

```bash
docker-compose up -d
```

### Manual Installation

For development or custom deployments, you can install Part-DB manually. See [Manual Installation](./manual.md) for detailed instructions.

## Prerequisites

Before installing Part-DB, ensure you have:

| Component | Requirement |
|-----------|-------------|
| Rust | 1.70 or higher |
| Node.js | 18.0 or higher |
| Database | PostgreSQL 14+, MySQL 5.7+, or SQLite 3 |
| Git | Any recent version |

## Post-Installation

After installing Part-DB:

1. **Configure the application** - See [Configuration](./configuration.md)
2. **Set up your first user** - Create an admin account
3. **Import existing data** - If migrating from another system
4. **Configure backups** - Set up regular database backups

## Upgrading

To upgrade Part-DB to a newer version:

1. Back up your database
2. Pull the latest code or Docker image
3. Run any database migrations
4. Restart the application

See [Deployment](../deployment/production.md) for production upgrade procedures.
